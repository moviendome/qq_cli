//! Config discovery and the trust gate (R13). Two user layers exist: a
//! per-project `.qq.toml` in the target directory and a global
//! `$XDG_CONFIG_HOME/qq/config.toml`. The per-project file is honored only
//! after a one-time `qq allow` approval, recorded as (absolute path, content
//! hash) in the global qq directory — approval goes stale when the file's
//! content changes. The global file is user-owned and implicitly trusted.

use crate::definition::{parse_definition, DefinitionError, DefinitionFile};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const PROJECT_CONFIG_NAME: &str = ".qq.toml";

pub struct ConfigPaths {
    pub project_config: PathBuf,
    pub global_config: PathBuf,
    pub trust_store: PathBuf,
}

impl ConfigPaths {
    pub fn discover(target_dir: &Path) -> ConfigPaths {
        let global_dir = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| !p.as_os_str().is_empty())
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("qq");
        Self::with_global_dir(target_dir, &global_dir)
    }

    /// Explicit global directory — used by tests to avoid touching $HOME.
    pub fn with_global_dir(target_dir: &Path, global_dir: &Path) -> ConfigPaths {
        ConfigPaths {
            project_config: target_dir.join(PROJECT_CONFIG_NAME),
            global_config: global_dir.join("config.toml"),
            trust_store: global_dir.join("trusted.toml"),
        }
    }
}

pub enum ProjectConfig {
    /// No `.qq.toml` in the target directory — not an error.
    Missing,
    /// Present and well-formed, but not (or no longer) approved.
    Untrusted { path: PathBuf },
    /// Present, well-formed, and approved for its current content.
    Trusted { file: DefinitionFile },
}

/// Load the global config; a missing file yields an empty definition.
pub fn load_global(paths: &ConfigPaths) -> Result<DefinitionFile, DefinitionError> {
    if !paths.global_config.exists() {
        return Ok(DefinitionFile::default());
    }
    let content = read_config(&paths.global_config)?;
    parse_definition(&paths.global_config.display().to_string(), &content)
}

/// Load the per-project config through the trust gate. Malformed files error
/// regardless of trust state (R7 — never silent), untrusted files are
/// reported so the CLI can print the notice.
pub fn load_project(paths: &ConfigPaths) -> Result<ProjectConfig, DefinitionError> {
    if !paths.project_config.exists() {
        return Ok(ProjectConfig::Missing);
    }
    let content = read_config(&paths.project_config)?;
    let file = parse_definition(&paths.project_config.display().to_string(), &content)?;
    if is_approved(paths, &paths.project_config, &content) {
        Ok(ProjectConfig::Trusted { file })
    } else {
        Ok(ProjectConfig::Untrusted {
            path: paths.project_config.clone(),
        })
    }
}

pub enum AllowOutcome {
    Approved(PathBuf),
    NothingToApprove,
}

/// `qq allow`: approve the target directory's `.qq.toml` at its current
/// content. Malformed files are refused with the parse error.
pub fn allow_project(paths: &ConfigPaths) -> Result<AllowOutcome, DefinitionError> {
    if !paths.project_config.exists() {
        return Ok(AllowOutcome::NothingToApprove);
    }
    let content = read_config(&paths.project_config)?;
    parse_definition(&paths.project_config.display().to_string(), &content)?;

    let key = approval_key(&paths.project_config);
    let mut store = read_trust_store(&paths.trust_store);
    store.approvals.insert(key, content_hash(&content));
    write_trust_store(&paths.trust_store, &store)?;
    Ok(AllowOutcome::Approved(paths.project_config.clone()))
}

#[derive(Default, Serialize, Deserialize)]
struct TrustStore {
    #[serde(default)]
    approvals: BTreeMap<String, String>,
}

fn is_approved(paths: &ConfigPaths, config_path: &Path, content: &str) -> bool {
    let store = read_trust_store(&paths.trust_store);
    store.approvals.get(&approval_key(config_path)).map(String::as_str)
        == Some(content_hash(content).as_str())
}

fn approval_key(config_path: &Path) -> String {
    fs::canonicalize(config_path)
        .unwrap_or_else(|_| config_path.to_path_buf())
        .display()
        .to_string()
}

/// FNV-1a 64 — deterministic across builds and Rust versions, which the
/// std hasher does not guarantee for a persisted approval store.
fn content_hash(content: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in content.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn read_trust_store(path: &Path) -> TrustStore {
    fs::read_to_string(path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or_default()
}

fn write_trust_store(path: &Path, store: &TrustStore) -> Result<(), DefinitionError> {
    let serialized = toml::to_string(store).map_err(|e| DefinitionError {
        source: path.display().to_string(),
        message: e.to_string(),
    })?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| io_error(path, &e))?;
    }
    fs::write(path, serialized).map_err(|e| io_error(path, &e))
}

fn read_config(path: &Path) -> Result<String, DefinitionError> {
    fs::read_to_string(path).map_err(|e| io_error(path, &e))
}

fn io_error(path: &Path, error: &std::io::Error) -> DefinitionError {
    DefinitionError {
        source: path.display().to_string(),
        message: error.to_string(),
    }
}
