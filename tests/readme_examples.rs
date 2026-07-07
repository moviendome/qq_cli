//! U6: every TOML block in the README's Configuration section must parse
//! against the real definition schema — the docs never drift from the code.

use qq_cli::definition::parse_definition;

#[test]
fn all_readme_toml_examples_parse() {
    let readme = include_str!("../README.md");
    let mut blocks: Vec<String> = Vec::new();
    let mut current: Option<String> = None;
    for line in readme.lines() {
        match (&mut current, line.trim_start()) {
            (None, l) if l.starts_with("```toml") => current = Some(String::new()),
            (Some(block), "```") => {
                blocks.push(std::mem::take(block));
                current = None;
            }
            (Some(block), _) => {
                block.push_str(line);
                block.push('\n');
            }
            _ => {}
        }
    }
    assert!(
        blocks.len() >= 3,
        "expected the README's three Configuration examples, found {}",
        blocks.len()
    );
    for (i, block) in blocks.iter().enumerate() {
        parse_definition(&format!("README example {}", i + 1), block)
            .unwrap_or_else(|e| panic!("README example {} does not parse: {e}", i + 1));
    }
}
