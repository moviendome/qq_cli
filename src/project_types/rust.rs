use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

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
    fn install_command(&self) -> String {
        "cargo build".to_string()
    }

    fn migrate_command(&self) -> Option<String> {
        None
    }

    fn start_command(&self) -> Option<String> {
        Some("cargo run".to_string())
    }

    fn test_command(&self) -> Option<String> {
        Some("cargo test".to_string())
    }
}
