use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

#[derive(Debug)]
pub struct Rust;

impl Rust {
    pub fn detect(current_dir: &Path) -> Option<Box<dyn ProjectTypeCommands>> {
        if current_dir.join("Cargo.lock").exists() {
            Some(Box::new(Rust))
        } else {
            None
        }
    }
}

impl ProjectTypeCommands for Rust {
    fn name(&self) -> &'static str {
        "Rust"
    }
    fn install_command(&self) -> String {
        "cargo build".to_string()
    }

    fn migrate_command(&self) -> Option<String> {
        None
    }

    fn console_command(&self) -> Option<String> {
        None
    }

    fn start_command(&self) -> Option<String> {
        Some("cargo run".to_string())
    }

    fn test_command(&self) -> Option<String> {
        Some("cargo test".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_rust_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Cargo.lock")).unwrap();

        let project_type = Rust::detect(dir.path()); // This function returns Box<dyn ProjectTypeCommands>
        assert!(project_type.is_some() && project_type.unwrap().name() == "Rust");
    }
}
