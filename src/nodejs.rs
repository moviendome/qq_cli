use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

pub struct Nodejs;

impl Nodejs {
    pub fn detect(current_dir: &Path) -> Option<Box<dyn ProjectTypeCommands>> {
        if current_dir.join("package.json").exists() {
            Some(Box::new(Nodejs))
        } else {
            None
        }
    }
}

impl ProjectTypeCommands for Nodejs {
    fn install_command(&self) -> String {
        "npm install".to_string()
    }

    fn migrate_command(&self) -> Option<String> {
        None
    }

    fn start_command(&self) -> Option<String> {
        Some("npm start".to_string())
    }

    fn test_command(&self) -> Option<String> {
        None
    }
}
