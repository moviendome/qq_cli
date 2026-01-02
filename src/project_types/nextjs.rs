use crate::project_type_trait::ProjectTypeCommands;
use std::path::Path;

#[derive(Debug)]
pub struct NextJs;

impl NextJs {
    pub fn detect(current_dir: &Path) -> Option<Box<dyn ProjectTypeCommands>> {
        let has_package_json = current_dir.join("package.json").exists();
        let has_next_config = current_dir.join("next.config.js").exists()
            || current_dir.join("next.config.mjs").exists()
            || current_dir.join("next.config.ts").exists();

        if has_package_json && has_next_config {
            Some(Box::new(NextJs))
        } else {
            None
        }
    }
}

impl ProjectTypeCommands for NextJs {
    fn name(&self) -> &'static str {
        "NextJS"
    }

    fn install_command(&self) -> String {
        "npm install".to_string()
    }

    fn start_command(&self) -> Option<String> {
        Some("npm run dev".to_string())
    }

    fn test_command(&self) -> Option<String> {
        Some("npm test".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_detect_nextjs_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();
        File::create(dir.path().join("next.config.js")).unwrap();

        let project_type = NextJs::detect(dir.path());
        assert!(project_type.is_some() && project_type.unwrap().name() == "NextJS");
    }

    #[test]
    fn test_detect_nextjs_with_mjs_config() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();
        File::create(dir.path().join("next.config.mjs")).unwrap();

        let project_type = NextJs::detect(dir.path());
        assert!(project_type.is_some() && project_type.unwrap().name() == "NextJS");
    }

    #[test]
    fn test_no_detect_without_next_config() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();

        let project_type = NextJs::detect(dir.path());
        assert!(project_type.is_none());
    }
}
