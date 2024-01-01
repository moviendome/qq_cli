mod project_type;
mod command_mapping;

use clap::{App, SubCommand, AppSettings};
use project_type::ProjectType;
use command_mapping::CommandMapping;
use std::env;
use std::process::Command;

fn main() {
    let matches = App::new("QQ CLI")
        .version("0.1")
        .author("Moviendome <estoy@moviendo.me>")
        .about("A CLI to run all")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("install")
            .about("Installs dependencies for the project")
            .alias("i"))
        .subcommand(SubCommand::with_name("migrate")
            .about("Runs database migrations")
            .alias("m"))
        .subcommand(SubCommand::with_name("start")
            .about("Starts the project")
            .alias("s"))
        .subcommand(SubCommand::with_name("test")
            .about("Run tests")
            .alias("t"))
        .after_help("Use 'qq [command]' to execute a command.")
        .help_template("{about}\n\nUSAGE:\n    qq command\n\nCOMMANDS:\n{subcommands}\n\nOPTIONS:\n    -h, --help       Print help information\n    -V, --version    Print version information")

        .get_matches();

    // Get the current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Pass the current directory to the detect method
    let project_type = ProjectType::detect(&current_dir);
    let command_mapping = CommandMapping::for_project_type(&project_type, &current_dir);

    match matches.subcommand() {
        Some(("install", _)) | Some(("i", _)) => {
            if let Some(cmd) = command_mapping.and_then(|m| Some(m.install)) {
                run_command(&cmd);
                println!("\nDependencies installed!\n");
            }
        },
        Some(("migrate", _)) | Some(("m", _)) => {
            if let Some(mapping) = command_mapping {
                if let Some(migrate_cmd) = mapping.migrate {
                    run_command(&migrate_cmd);
                    println!("\nDatabase migrated!\n");
                } else {
                    println!("\n'migrate' command not supported for {}\n", project_type);
                }
            }
        },
        Some(("start", _)) | Some(("s", _)) => {
            if let Some(mapping) = command_mapping {
                if let Some(start_cmd) = mapping.start {
                    run_command(&start_cmd);
                } else {
                    println!("\n'start' command not supported for {}\n", project_type);
                }
            }
        },
        Some(("test", _)) | Some(("t", _)) => {
            if let Some(cmd) = command_mapping.and_then(|m| m.test) {
                run_command(&cmd);
            } else {
                println!("'test' command not supported for {} project type", project_type);
            }
        },
        _ => println!("\nNo valid command was provided. Run qq --help for more information.\n"),
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
