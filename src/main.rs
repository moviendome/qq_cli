mod project_type_trait;
mod project_types;
mod utils;

use project_types::middleman::Middleman;
use project_types::nodejs::Nodejs;
use project_types::rails::Rails;
use project_types::rust::Rust;

use clap::{App, SubCommand};
use std::env;
use std::process::Command;

fn main() {
    let app = App::new("QQ CLI")
        .version("0.1")
        .author("Moviendome <estoy@moviendo.me>")
        .about("A CLI to run all")
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
            SubCommand::with_name("console")
                .about("Runs console")
                .alias("c"),
        )
        .subcommand(
            SubCommand::with_name("start")
                .about("Starts the project")
                .alias("s"),
        )
        .subcommand(SubCommand::with_name("test").about("Run tests").alias("t"))
        .subcommand(
            SubCommand::with_name("routes")
                .about("Show Routes")
                .alias("r"),
        )
        .subcommand(
            SubCommand::with_name("g")
                .about("Run git status")
                .alias("g"),
        )
        .subcommand(SubCommand::with_name("gl").about("Run git log").alias("gl"))
        .subcommand(
            SubCommand::with_name("gp")
                .about("Run git pull")
                .alias("gp"),
        )
        .subcommand(
            SubCommand::with_name("gP")
                .about("Run git push")
                .alias("gP"),
        )
        .subcommand(
            SubCommand::with_name("gm")
                .about("Run git push")
                .alias("g,"),
        )
        .subcommand(
            SubCommand::with_name("ga")
                .about("Run git amend")
                .alias("ga"),
        )
        .after_help("Use 'qq [command]' to execute a command.");

    let matches = app.get_matches();

    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Detect the project type
    let project_type = Middleman::detect(&current_dir)
        .or_else(|| Rails::detect(&current_dir))
        .or_else(|| Rust::detect(&current_dir))
        .or_else(|| Nodejs::detect(&current_dir));

    let commands = match project_type {
        Some(pt) => pt,
        None => {
            println!("Project type found in current directory is not supported.");
            return; // or use std::process::exit(1) for an error exit code
        }
    };

    // If no subcommand was provided, show the interactive menu
    if matches.subcommand_name().is_none() {
        show_interactive_menu(commands);
        return;
    }

    match matches.subcommand() {
        // For Project Types
        Some(("install", _)) => run_command(&commands.install_command()),
        Some(("migrate", _)) => {
            if let Some(cmd) = commands.migrate_command() {
                run_command(&cmd);
            } else {
                println!("'migrate' command not supported for this project type.");
            }
        }
        Some(("console", _)) => {
            if let Some(cmd) = commands.console_command() {
                run_command(&cmd);
            } else {
                println!("'console' command not supported for this project type.");
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
        Some(("routes", _)) => {
            if let Some(cmd) = commands.routes_command() {
                run_command(&cmd);
            } else {
                println!("'routes' command not supported for this project type.");
            }
        }
        // For Git
        Some(("g", _)) => run_command("git status"),
        Some(("gl", _)) => run_command("git lg"),
        Some(("gp", _)) => run_command("git pull"),
        Some(("gP", _)) => run_command("git push"),
        Some(("gm", _)) => run_command("git checkout main"),
        Some(("ga", _)) => run_command("git commit --amend --no-edit"),
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

fn show_interactive_menu(commands: Box<dyn project_type_trait::ProjectTypeCommands>) {
    //    let logo = r#"
    //  ___   ___    ___ _    ___
    // / _ \ / _ \  / __| |  |_ _|
    //| (_) | (_) || (__| |__ | |
    // \__\_\\__\_\ \___|____|___|
    //
    //"#;

    let logo = r#"
 ________  ________           ________  ___       ___     
|\   __  \|\   __  \         |\   ____\|\  \     |\  \    
\ \  \|\  \ \  \|\  \        \ \  \___|\ \  \    \ \  \   
 \ \  \\\  \ \  \\\  \        \ \  \    \ \  \    \ \  \  
  \ \  \\\  \ \  \\\  \        \ \  \____\ \  \____\ \  \ 
   \ \_____  \ \_____  \        \ \_______\ \_______\ \__\
    \|___| \__\|___| \__\        \|_______|\|_______|\|__|
          \|__|     \|__|                                 
"#;

    println!("{}", logo);

    let cmd = inquire::Text::new("Enter Command: ")
        .with_help_message("Enter a valid command")
        .with_autocomplete(&utils::suggester)
        .prompt()
        .unwrap_or_else(|_| "exit".to_string());

    match cmd.as_str() {
        "install" => run_command(&commands.install_command()),
        "migrate" => {
            if let Some(cmd) = commands.migrate_command() {
                run_command(&cmd);
            } else {
                println!("'migrate' command not supported for this project type.");
            }
        }
        "console" => {
            if let Some(cmd) = commands.console_command() {
                run_command(&cmd);
            } else {
                println!("'console' command not supported for this project type.");
            }
        }
        "start" => {
            if let Some(cmd) = commands.start_command() {
                run_command(&cmd);
            } else {
                println!("'start' command not supported for this project type.");
            }
        }
        "test" => {
            if let Some(cmd) = commands.test_command() {
                run_command(&cmd);
            } else {
                println!("'test' command not supported for this project type.");
            }
        }
        "routes" => {
            if let Some(cmd) = commands.routes_command() {
                run_command(&cmd);
            } else {
                println!("'routes' command not supported for this project type.");
            }
        }
        "g" => run_command("git status"),
        "gl" => run_command("git lg"),
        "gp" => run_command("git pull"),
        "gP" => run_command("git push"),
        "gm" => run_command("git checkout main"),
        "ga" => run_command("git commit --amend --no-edit"),
        "exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        "help" => {
            println!("Available commands:");
            println!("  install - Installs dependencies for the project");
            println!("  migrate - Runs database migrations");
            println!("  console - Runs console");
            println!("  start   - Starts the project");
            println!("  test    - Run tests");
            println!("  routes  - Show Routes");
            println!("  g       - Run git status");
            println!("  gl      - Run git log");
            println!("  gp      - Run git pull");
            println!("  gP      - Run git push");
            println!("  gm      - Run git checkout main");
            println!("  ga      - Run git commit --amend --no-edit");
            println!("  exit    - Exit the program");
            println!("  help    - Show this help message");
        }
        _ => println!("Command not recognized."),
    }
}
