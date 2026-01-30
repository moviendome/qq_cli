use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

#[derive(Debug)]
pub struct Anchor;

impl Anchor {
    pub fn detect(current_dir: &Path) -> Option<Box<dyn ProjectTypeCommands>> {
        if current_dir.join("Anchor.toml").exists() {
            Some(Box::new(Anchor))
        } else {
            None
        }
    }
}

impl ProjectTypeCommands for Anchor {
    fn name(&self) -> &'static str {
        "Anchor"
    }
    fn install_command(&self) -> String {
        "anchor build".to_string()
    }

    fn start_command(&self) -> Option<String> {
        Some("anchor localnet".to_string())
    }

    fn test_command(&self) -> Option<String> {
        Some("anchor test".to_string())
    }

    fn deploy_command(&self) -> Option<String> {
        Some("anchor deploy".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_anchor_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Anchor.toml")).unwrap();

        let project_type = Anchor::detect(dir.path());
        assert!(project_type.is_some() && project_type.unwrap().name() == "Anchor");
    }

    #[test]
    fn test_anchor_not_detected_without_toml() {
        let dir = tempdir().unwrap();

        let project_type = Anchor::detect(dir.path());
        assert!(project_type.is_none());
    }
}
