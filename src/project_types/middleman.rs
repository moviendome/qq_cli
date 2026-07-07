use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

#[derive(Debug)]
pub struct Middleman;

impl Middleman {
    pub fn detect(current_dir: &Path) -> Option<Box<dyn ProjectTypeCommands>> {
        if current_dir.join("Gemfile").exists() && current_dir.join("source").exists() {
            Some(Box::new(Middleman))
        } else {
            None
        }
    }
}

impl ProjectTypeCommands for Middleman {
    fn name(&self) -> &'static str {
        "Middleman"
    }

    fn install_command(&self) -> String {
        "bundle install".to_string()
    }

    fn start_command(&self) -> Option<String> {
        Some("bundle exec middleman serve".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, File};
    use tempfile::tempdir;

    #[test]
    fn test_detect_middleman_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Gemfile")).unwrap();
        create_dir(dir.path().join("source")).unwrap();

        let project_type = Middleman::detect(dir.path());
        assert!(project_type.is_some() && project_type.unwrap().name() == "Middleman");
    }

    #[test]
    fn test_middleman_not_detected_without_source_dir() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Gemfile")).unwrap();

        let project_type = Middleman::detect(dir.path());
        assert!(project_type.is_none());
    }
}
