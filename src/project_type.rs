use std::path::Path;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ProjectType {
    Rails,
    NodeJs,
    Unknown,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Rails => write!(f, "Rails"),
            ProjectType::NodeJs => write!(f, "Node.js"),
            ProjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl ProjectType {
    pub fn detect(directory: &Path) -> Self {
        let gemfile_path = directory.join("Gemfile");
        let package_json_path = directory.join("package.json");

        if gemfile_path.exists() {
            println!("\nRails project detected!\n");
            ProjectType::Rails
        } else if package_json_path.exists() {
            println!("Node project detected!");
            ProjectType::NodeJs
        } else {
            println!("\nNo project detected :(\n");
            ProjectType::Unknown
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
        
        assert_eq!(ProjectType::detect(dir.path()), ProjectType::Rails);
    }

    #[test]
    fn test_detect_nodejs_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("package.json")).unwrap();
        
        assert_eq!(ProjectType::detect(dir.path()), ProjectType::NodeJs);
    }

    #[test]
    fn test_detect_unknown_project() {
        let dir = tempdir().unwrap();
        
        assert_eq!(ProjectType::detect(dir.path()), ProjectType::Unknown);
    }
}
