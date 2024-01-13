use std::fmt;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum ProjectType {
    NodeJs,
    Rails,
    Rust,
    Unknown,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::NodeJs => write!(f, "Node.js"),
            ProjectType::Rails => write!(f, "Rails"),
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl ProjectType {
    pub fn detect(directory: &Path) -> Self {
        let gemfile_path = directory.join("Gemfile");
        let package_json_path = directory.join("package.json");
        let cargo_path = directory.join("Cargo.lock");

        if gemfile_path.exists() {
            println!("\nRails project detected!\n");
            ProjectType::Rails
        } else if package_json_path.exists() {
            println!("Node project detected!");
            ProjectType::NodeJs
        } else if cargo_path.exists() {
            println!("Rust project detected!");
            ProjectType::Rust
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
    fn test_detect_rust_project() {
        let dir = tempdir().unwrap();
        File::create(dir.path().join("Cargo.lock")).unwrap();

        assert_eq!(ProjectType::detect(dir.path()), ProjectType::Rust);
    }

    #[test]
    fn test_detect_unknown_project() {
        let dir = tempdir().unwrap();

        assert_eq!(ProjectType::detect(dir.path()), ProjectType::Unknown);
    }
}
