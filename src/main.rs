mod project_type_trait;
mod project_types;

use project_types::nodejs::Nodejs;
use project_types::rails::Rails;
use project_types::rust::Rust;

use clap::{App, AppSettings, SubCommand};
use std::env;
use std::process::Command;

fn main() {
    let matches = App::new("QQ CLI")
        .version("0.1")
        .author("Moviendome <estoy@moviendo.me>")
        .about("A CLI to run all")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs dependencies for the project")
                .alias("i"),
        )
        .subcommand(
            SubCommand::with_name("migrate")
                .about("Runs database migrations")
                .alias("m"),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("Starts the project")
                .alias("s"),
        )
        .subcommand(SubCommand::with_name("test").about("Run tests").alias("t"))
        .after_help("Use 'qq [command]' to execute a command.")
        .get_matches();

    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Detect the project type
    let commands = Rails::detect(&current_dir)
        .or_else(|| Nodejs::detect(&current_dir))
        .or_else(|| Rust::detect(&current_dir))
        .expect("Project type not supported");

    match matches.subcommand() {
        Some(("install", _)) => run_command(&commands.install_command()),
        Some(("migrate", _)) => {
            if let Some(cmd) = commands.migrate_command() {
                run_command(&cmd);
            } else {
                println!("'migrate' command not supported for this project type.");
            }
        }
        Some(("start", _)) => {
            if let Some(cmd) = commands.start_command() {
                run_command(&cmd);
            } else {
                println!("'start' command not supported for this project type.");
            }
        }
        Some(("test", _)) => {
            if let Some(cmd) = commands.test_command() {
                run_command(&cmd);
            } else {
                println!("'test' command not supported for this project type.");
            }
        }
        _ => println!("Command not recognized."),
    }
}

fn run_command(command: &str) {
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed to execute.");
    }
}
