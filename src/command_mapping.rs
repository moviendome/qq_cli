use crate::project_type::ProjectType;
use std::path::PathBuf;

pub struct CommandMapping {
    pub install: String,
    pub migrate: Option<String>,
    pub start: Option<String>,
    pub test: Option<String>
}

impl CommandMapping {
    pub fn for_project_type(project_type: &ProjectType, current_dir: &PathBuf) -> Option<Self> {
        match project_type {
            ProjectType::Rails =>  {
                let test_command = determine_rails_test_command(current_dir);

                Some(Self {
                    install: "bundle install".to_string(),
                    migrate: Some("bin/rails db:migrate".to_string()),
                    start: Some("bin/dev".to_string()),
                    test: test_command,
                })
            },
            ProjectType::NodeJs => Some(Self {
                install: "npm install".to_string(),
                migrate: None,
                start: Some("npm start".to_string()),
                test: None,
            }),
            ProjectType::Unknown => None,
        }
    }
}

fn determine_rails_test_command(current_dir: &PathBuf) -> Option<String> {
    let test_dir = current_dir.join("test");
    let spec_dir = current_dir.join("spec");

    if test_dir.exists() {
        Some("bin/rails t".to_string())
    } else if spec_dir.exists() {
        Some("bin/rspec".to_string())
    } else {
        None
    }
}
