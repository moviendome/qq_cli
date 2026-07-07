//! Built-in project types, embedded at compile time from `src/definitions/`.
//! Dropping a new `.toml` file into that directory is the entire source-side
//! cost of a new built-in type — nothing here enumerates the files.

use crate::definition::{parse_definition, DefinitionError, TypeDef};
use include_dir::{include_dir, Dir};

static DEFINITIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/definitions");

/// Parse every embedded definition file. Returns (type key, definition)
/// pairs sorted by priority (lower first — the detection order).
pub fn builtin_types() -> Result<Vec<(String, TypeDef)>, DefinitionError> {
    let mut types: Vec<(String, TypeDef)> = Vec::new();
    for file in DEFINITIONS_DIR.files() {
        let source = format!("built-in {}", file.path().display());
        let content = file.contents_utf8().ok_or_else(|| DefinitionError {
            source: source.clone(),
            message: "not valid UTF-8".to_string(),
        })?;
        let parsed = parse_definition(&source, content)?;
        if !parsed.commands.is_empty() {
            return Err(DefinitionError {
                source,
                message: "built-in definitions must declare [types.*], not a top-level [commands] block".to_string(),
            });
        }
        types.extend(parsed.types);
    }
    types.sort_by_key(|(_, t)| t.priority);
    Ok(types)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_embedded_definitions_parse_and_all_six_types_load() {
        let types = builtin_types().expect("every embedded definition must parse");
        let keys: Vec<&str> = types.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(
            keys,
            vec!["middleman", "rails", "anchor", "rust", "nextjs", "nodejs"],
            "priority order must reproduce the legacy detection chain"
        );
    }

    #[test]
    fn priorities_are_unique() {
        let types = builtin_types().unwrap();
        let priorities: HashSet<i64> = types.iter().map(|(_, t)| t.priority).collect();
        assert_eq!(priorities.len(), types.len());
    }

    #[test]
    fn every_type_has_a_display_name_and_install() {
        for (key, t) in builtin_types().unwrap() {
            assert!(t.name.is_some(), "{key}: display name required");
            assert!(t.commands.contains_key("install"), "{key}: install required");
        }
    }

    #[test]
    fn non_anchor_definitions_carry_kamal_gated_deploy() {
        // The legacy trait DEFAULT gave every type a .kamal-gated deploy;
        // only Anchor overrode it. The definitions must not drop it.
        for (key, t) in builtin_types().unwrap() {
            let deploy = t.commands.get("deploy").unwrap_or_else(|| panic!("{key}: deploy missing"));
            if key == "anchor" {
                assert_eq!(deploy.len(), 1);
                assert_eq!(deploy[0].run, "anchor deploy");
                assert!(deploy[0].when_path.is_none(), "anchor deploy is unconditional");
            } else {
                assert_eq!(deploy.len(), 1, "{key}: single gated deploy candidate");
                assert_eq!(deploy[0].run, "kamal deploy", "{key}");
                assert_eq!(deploy[0].when_path.as_deref(), Some(".kamal"), "{key}");
            }
        }
    }
}
