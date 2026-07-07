pub mod config;
pub mod definition;
pub mod definitions;
pub mod project_type_trait;
pub mod project_types;
pub mod resolver;
pub mod utils;

use project_type_trait::ProjectTypeCommands;
use std::path::Path;

/// Detect the project type for a directory using the current hardcoded chain,
/// most-specific-first: Middleman, Rails, Anchor, Rust, NextJs, Nodejs.
pub fn detect_project(current_dir: &Path) -> Option<Box<dyn ProjectTypeCommands>> {
    project_types::middleman::Middleman::detect(current_dir)
        .or_else(|| project_types::rails::Rails::detect(current_dir))
        .or_else(|| project_types::anchor::Anchor::detect(current_dir))
        .or_else(|| project_types::rust::Rust::detect(current_dir))
        .or_else(|| project_types::nextjs::NextJs::detect(current_dir))
        .or_else(|| project_types::nodejs::Nodejs::detect(current_dir))
}

/// Map a project command name to its resolved command string, mirroring the
/// dispatch in main.rs. `install` is always Some; the rest follow the trait.
/// Note: conditional commands (Rails start/test, default deploy) evaluate
/// their gates against the process working directory, not a parameter.
pub fn project_command(commands: &dyn ProjectTypeCommands, name: &str) -> Option<String> {
    match name {
        "install" => Some(commands.install_command()),
        "migrate" => commands.migrate_command(),
        "console" => commands.console_command(),
        "start" => commands.start_command(),
        "test" => commands.test_command(),
        "routes" => commands.routes_command(),
        "deploy" => commands.deploy_command(),
        _ => None,
    }
}
