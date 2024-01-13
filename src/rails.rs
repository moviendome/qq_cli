use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

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
    fn install_command(&self) -> String {
        "bundle install".to_string()
    }

    fn migrate_command(&self) -> Option<String> {
        Some("bin/rails db:migrate".to_string())
    }

    fn start_command(&self) -> Option<String> {
        Some("bin/rails server".to_string())
    }

    fn test_command(&self) -> Option<String> {
        // Here, you should decide how to determine if the project uses RSpec or Minitest.
        // For example, checking for a 'spec' directory for RSpec:
        let test_dir = Path::new("test");
        let spec_dir = Path::new("spec");

        if spec_dir.exists() {
            Some("bin/rspec".to_string())
        } else if test_dir.exists() {
            Some("bin/rails test".to_string())
        } else {
            None
        }
    }
}
