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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::File;
//     use tempfile::tempdir;
//
//     #[test]
//     fn test_detect_middleman_project() {
//         let dir = tempdir().unwrap();
//         File::create(dir.path().join("Gemfile")).unwrap();
//
//         let project_type = Middleman::detect(dir.path()); // This function returns Box<dyn ProjectTypeCommands>
//         assert!(project_type.is_some() && project_type.unwrap().name() == "Middleman");
//     }
// }
