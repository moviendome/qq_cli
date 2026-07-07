//! The resolution engine (R2): parse definitions, detect the project type,
//! merge layers, and evaluate candidate gates against the target directory.
//!
//! Layer precedence, most explicit first (KTD6):
//!   1. per-project `[commands]` block
//!   2. the winning type's commands, merged per command across layers
//!      (per-project > global > built-in for the same type key)
//!   3. global `[commands]` block
//!   4. when a config-declared type won: the built-in type that would have
//!      detected in this directory, as merge fallback (AE2)
//!
//! Gate evaluation is dir-parameterized (`when_path` joins the target
//! directory); this is behavior-equivalent to the legacy cwd-relative gates
//! because qq always runs with cwd == target — the differential parity test
//! in tests/resolver_engine.rs proves the equivalence.

use crate::config::{self, ConfigPaths, ProjectConfig};
use crate::definition::{Candidate, DefinitionError, DefinitionFile, TypeDef};
use crate::definitions::builtin_types;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct ResolveOutcome {
    pub resolution: Option<Resolution>,
    /// Set when a `.qq.toml` exists but is not approved — the CLI prints the
    /// notice and proceeds with built-in behavior (R13).
    pub untrusted_project_config: Option<PathBuf>,
}

#[derive(Debug)]
pub struct Resolution {
    /// Display name for "Detected: {name}"; None for a typeless resolution
    /// (commands supplied purely by config in an unrecognized directory).
    pub display_name: Option<String>,
    commands: BTreeMap<String, Vec<Candidate>>,
}

impl Resolution {
    /// Resolve a command name to its shell string: first candidate whose
    /// gates pass. None = "not supported" (no entry, or every gate failed).
    pub fn command(&self, name: &str, target_dir: &Path) -> Option<String> {
        self.commands
            .get(name)?
            .iter()
            .find(|c| gate_passes(c, target_dir))
            .map(|c| c.run.clone())
    }

    /// Command names available for autocomplete/help/dynamic subcommands.
    pub fn command_names(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }
}

pub fn resolve(target_dir: &Path, paths: &ConfigPaths) -> Result<ResolveOutcome, DefinitionError> {
    let global = config::load_global(paths)?;
    let (project, untrusted) = match config::load_project(paths)? {
        ProjectConfig::Missing => (DefinitionFile::default(), None),
        ProjectConfig::Untrusted { path } => (DefinitionFile::default(), Some(path)),
        ProjectConfig::Trusted { file } => (file, None),
    };
    let builtins = builtin_types()?;

    let winner = detect(target_dir, &project, &global, &builtins);

    let mut commands: BTreeMap<String, Vec<Candidate>> = BTreeMap::new();

    // Layer 4: built-in fallback when a config-declared type shadowed it.
    if let Some(w) = &winner {
        if w.layer != Layer::Builtin {
            if let Some((bkey, _)) = builtins
                .iter()
                .find(|(_, t)| markers_match(target_dir, &t.detect, &t.detect_any, false))
            {
                if bkey != &w.key {
                    merge_type_commands(&mut commands, bkey, &project, &global, &builtins);
                }
            }
        }
    }
    // Layer 3: global typeless commands.
    for (name, candidates) in &global.commands {
        commands.insert(name.clone(), candidates.clone());
    }
    // Layer 2: the winning type's per-command merge across layers.
    if let Some(w) = &winner {
        merge_type_commands(&mut commands, &w.key, &project, &global, &builtins);
    }
    // Layer 1: per-project typeless commands.
    for (name, candidates) in &project.commands {
        commands.insert(name.clone(), candidates.clone());
    }

    let resolution = if winner.is_none() && commands.is_empty() {
        None
    } else {
        Some(Resolution {
            display_name: winner.map(|w| w.display_name),
            commands,
        })
    };

    Ok(ResolveOutcome {
        resolution,
        untrusted_project_config: untrusted,
    })
}

#[derive(PartialEq, Clone, Copy)]
enum Layer {
    Project,
    Global,
    Builtin,
}

struct Winner {
    key: String,
    display_name: String,
    layer: Layer,
}

/// Detection: config-declared types run ahead of built-ins regardless of
/// priority (R6); within a layer, lower priority runs earlier (R9). A type
/// declared in the per-project file with no markers matches its own
/// directory implicitly; markerless types in other layers never match.
fn detect(
    target_dir: &Path,
    project: &DefinitionFile,
    global: &DefinitionFile,
    builtins: &[(String, TypeDef)],
) -> Option<Winner> {
    let mut candidates: Vec<(Layer, i64, &String, &TypeDef)> = Vec::new();
    for (key, t) in &project.types {
        candidates.push((Layer::Project, t.priority, key, t));
    }
    for (key, t) in &global.types {
        candidates.push((Layer::Global, t.priority, key, t));
    }
    for (key, t) in builtins {
        candidates.push((Layer::Builtin, t.priority, key, t));
    }
    candidates.sort_by_key(|(layer, priority, key, _)| {
        let rank = match layer {
            Layer::Project => 0,
            Layer::Global => 1,
            Layer::Builtin => 2,
        };
        (rank, *priority, (*key).clone())
    });

    for (layer, _, key, t) in candidates {
        let implicit_ok = layer == Layer::Project;
        if markers_match(target_dir, &t.detect, &t.detect_any, implicit_ok) {
            let display_name = resolve_display_name(key, project, global, builtins)
                .unwrap_or_else(|| key.clone());
            return Some(Winner {
                key: key.clone(),
                display_name,
                layer,
            });
        }
    }
    None
}

fn markers_match(dir: &Path, detect: &[String], detect_any: &[String], implicit_ok: bool) -> bool {
    if detect.is_empty() && detect_any.is_empty() {
        return implicit_ok;
    }
    let all = detect.iter().all(|m| dir.join(m).exists());
    let any = detect_any.is_empty() || detect_any.iter().any(|m| dir.join(m).exists());
    all && any
}

/// Display name for a type key: most explicit layer that sets one.
fn resolve_display_name(
    key: &str,
    project: &DefinitionFile,
    global: &DefinitionFile,
    builtins: &[(String, TypeDef)],
) -> Option<String> {
    project
        .types
        .get(key)
        .and_then(|t| t.name.clone())
        .or_else(|| global.types.get(key).and_then(|t| t.name.clone()))
        .or_else(|| {
            builtins
                .iter()
                .find(|(k, _)| k == key)
                .and_then(|(_, t)| t.name.clone())
        })
}

/// Overlay a type's commands into `commands`, least explicit layer first,
/// so per-command fallback works within the same type key (R5).
fn merge_type_commands(
    commands: &mut BTreeMap<String, Vec<Candidate>>,
    key: &str,
    project: &DefinitionFile,
    global: &DefinitionFile,
    builtins: &[(String, TypeDef)],
) {
    if let Some((_, t)) = builtins.iter().find(|(k, _)| k == key) {
        for (name, candidates) in &t.commands {
            commands.insert(name.clone(), candidates.clone());
        }
    }
    if let Some(t) = global.types.get(key) {
        for (name, candidates) in &t.commands {
            commands.insert(name.clone(), candidates.clone());
        }
    }
    if let Some(t) = project.types.get(key) {
        for (name, candidates) in &t.commands {
            commands.insert(name.clone(), candidates.clone());
        }
    }
}

fn gate_passes(candidate: &Candidate, target_dir: &Path) -> bool {
    if let Some(path) = &candidate.when_path {
        if !target_dir.join(path).exists() {
            return false;
        }
    }
    if let Some(bin) = &candidate.when_bin {
        if !bin_available(bin) {
            return false;
        }
    }
    true
}

/// Same liveness probe the legacy fzf gate used: run `<bin> --version`.
fn bin_available(bin: &str) -> bool {
    Command::new(bin)
        .arg("--version")
        .output()
        .map_or(false, |output| output.status.success())
}
