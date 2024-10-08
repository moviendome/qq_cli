use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct Rails;

impl Rails {
    pub fn detect(current_dir: &Path) -> Option<Box<dyn ProjectTypeCommands>> {
        if current_dir.join("Gemfile").exists() {
            Some(Box::new(Rails))
        } else {
            None
        }
    }
}

impl ProjectTypeCommands for Rails {
    fn name(&self) -> &'static str {
        "Rails"
    }

    fn install_command(&self) -> String {
        "bundle install".to_string()
    }

    fn migrate_command(&self) -> Option<String> {
        Some("bin/rails db:migrate".to_string())
    }

    fn console_command(&self) -> Option<String> {
        Some("bin/rails c".to_string())
    }

    fn start_command(&self) -> Option<String> {
        let dev = Path::new("bin/dev");

        if dev.exists() {
            Some("bin/dev".to_string())
        } else {
            Some("bin/rails s".to_string())
        }
    }

    fn test_command(&self) -> Option<String> {
        let test_dir = Path::new("test");
        let spec_dir = Path::new("spec");

        if spec_dir.exists() {
            Some("bin/rspec spec/".to_string())
        } else if test_dir.exists() {
            Some("bin/rails test".to_string())
        } else {
            None
        }
    }

    fn routes_command(&self) -> Option<String> {
        let is_fzf_installed = {
            Command::new("fzf")
                .arg("--version")
                .output()
                .map_or(false, |output| output.status.success())
        };

        if is_fzf_installed {
            Some("bin/rails routes | fzf -e".to_string())
        } else {
            Some("bin/rails routes".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_rails_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Gemfile")).unwrap();

        let project_type = Rails::detect(dir.path()); // This function returns Box<dyn ProjectTypeCommands>
        assert!(project_type.is_some() && project_type.unwrap().name() == "Rails");
    }
}
