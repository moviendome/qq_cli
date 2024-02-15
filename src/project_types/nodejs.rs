use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

#[derive(Debug)]
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
    fn name(&self) -> &'static str {
        "Nodejs"
    }
    fn install_command(&self) -> String {
        "npm install".to_string()
    }

    fn migrate_command(&self) -> Option<String> {
        None
    }

    fn console_command(&self) -> Option<String> {
        None
    }

    fn start_command(&self) -> Option<String> {
        Some("npm start".to_string())
    }

    fn test_command(&self) -> Option<String> {
        None
    }

    fn routes_command(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_nodejs_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();

        let project_type = Nodejs::detect(dir.path()); // This function returns Box<dyn ProjectTypeCommands>
        assert!(project_type.is_some() && project_type.unwrap().name() == "Nodejs");
    }
}
