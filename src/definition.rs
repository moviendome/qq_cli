//! Definition schema: the single format for built-in project types and user
//! config files (per-project and global). A file may declare named types
//! (detection markers + commands) and/or a typeless `[commands]` override
//! block. Candidates form an ordered fallback list; the first one whose gate
//! passes wins. Gates are the format's ceiling: a path-exists check
//! (`when_path`) or a binary-availability check (`when_bin`) — no general
//! conditional syntax.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DefinitionFile {
    /// Typeless command overrides — apply to whatever type resolves.
    #[serde(default)]
    pub commands: BTreeMap<String, Vec<Candidate>>,
    /// Named type definitions, keyed by type key (e.g. "rails").
    #[serde(default)]
    pub types: BTreeMap<String, TypeDef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDef {
    /// Display name ("Detected: {name}"). Defaults to the type key.
    pub name: Option<String>,
    /// Detection order among types in the same layer: lower runs earlier.
    #[serde(default)]
    pub priority: i64,
    /// Markers that must ALL exist in the target directory.
    #[serde(default)]
    pub detect: Vec<String>,
    /// Markers of which AT LEAST ONE must exist (empty = no constraint).
    #[serde(default)]
    pub detect_any: Vec<String>,
    #[serde(default)]
    pub commands: BTreeMap<String, Vec<Candidate>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    /// The shell command to run.
    pub run: String,
    /// Gate: relative path that must exist in the target directory.
    pub when_path: Option<String>,
    /// Gate: binary that must respond successfully to `<bin> --version`
    /// (the same liveness probe the legacy fzf check used).
    pub when_bin: Option<String>,
}

/// Parse failure carrying the source label (file path or built-in name).
#[derive(Debug)]
pub struct DefinitionError {
    pub source: String,
    pub message: String,
}

impl fmt::Display for DefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid definition in {}: {}", self.source, self.message)
    }
}

impl std::error::Error for DefinitionError {}

pub fn parse_definition(source: &str, content: &str) -> Result<DefinitionFile, DefinitionError> {
    toml::from_str(content).map_err(|e| DefinitionError {
        source: source.to_string(),
        message: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_type_definition() {
        let toml = r#"
            [types.rails]
            name = "Rails"
            priority = 20
            detect = ["Gemfile"]

            [types.rails.commands]
            install = [{ run = "bundle install" }]
            start = [
                { run = "bin/dev", when_path = "bin/dev" },
                { run = "bin/rails s" },
            ]
            routes = [
                { run = "bin/rails routes | fzf -e", when_bin = "fzf" },
                { run = "bin/rails routes" },
            ]
        "#;
        let parsed = parse_definition("rails.toml", toml).unwrap();
        let rails = &parsed.types["rails"];
        assert_eq!(rails.name.as_deref(), Some("Rails"));
        assert_eq!(rails.priority, 20);
        assert_eq!(rails.detect, vec!["Gemfile"]);
        assert_eq!(rails.commands["start"].len(), 2);
        assert_eq!(rails.commands["start"][0].when_path.as_deref(), Some("bin/dev"));
        assert_eq!(rails.commands["routes"][0].when_bin.as_deref(), Some("fzf"));
    }

    #[test]
    fn parses_commands_only_config() {
        let toml = r#"
            [commands]
            test = [{ run = "make check" }]
            lint = [{ run = "cargo clippy" }]
        "#;
        let parsed = parse_definition(".qq.toml", toml).unwrap();
        assert!(parsed.types.is_empty());
        assert_eq!(parsed.commands["test"][0].run, "make check");
        assert_eq!(parsed.commands["lint"][0].run, "cargo clippy");
    }

    #[test]
    fn parses_partial_type_override() {
        let toml = r#"
            [types.rails.commands]
            test = [{ run = "bin/rails test:all" }]
        "#;
        let parsed = parse_definition("config.toml", toml).unwrap();
        let rails = &parsed.types["rails"];
        assert!(rails.detect.is_empty());
        assert_eq!(rails.commands["test"][0].run, "bin/rails test:all");
    }

    #[test]
    fn parses_empty_file() {
        let parsed = parse_definition("empty.toml", "").unwrap();
        assert!(parsed.commands.is_empty());
        assert!(parsed.types.is_empty());
    }

    #[test]
    fn parses_type_without_commands() {
        let toml = r#"
            [types.marker]
            detect = ["marker.txt"]
        "#;
        let parsed = parse_definition("x.toml", toml).unwrap();
        assert!(parsed.types["marker"].commands.is_empty());
    }

    #[test]
    fn parses_all_gated_candidate_list() {
        let toml = r#"
            [commands]
            fmt = [
                { run = "cargo +nightly fmt", when_bin = "rustup" },
                { run = "cargo fmt", when_path = "Cargo.toml" },
            ]
        "#;
        let parsed = parse_definition("x.toml", toml).unwrap();
        assert_eq!(parsed.commands["fmt"].len(), 2);
    }

    #[test]
    fn syntax_error_names_the_source() {
        let err = parse_definition(".qq.toml", "not [valid toml").unwrap_err();
        assert_eq!(err.source, ".qq.toml");
        assert!(err.to_string().contains(".qq.toml"));
    }

    #[test]
    fn unknown_key_is_rejected() {
        let toml = r#"
            [types.rails]
            detekt = ["Gemfile"]
        "#;
        let err = parse_definition("typo.toml", toml).unwrap_err();
        assert!(err.message.contains("detekt") || err.message.contains("unknown"));
    }

    #[test]
    fn wrong_value_type_is_a_clear_error() {
        let toml = r#"
            [types.rails]
            detect = "Gemfile"
        "#;
        let err = parse_definition("bad.toml", toml).unwrap_err();
        assert_eq!(err.source, "bad.toml");
        assert!(!err.message.is_empty());
    }
}
