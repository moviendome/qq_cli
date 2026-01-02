mod project_type_trait;
mod project_types;
mod utils;

use project_types::middleman::Middleman;
use project_types::nextjs::NextJs;
use project_types::nodejs::Nodejs;
use project_types::rails::Rails;
use project_types::rust::Rust;

use clap::{App, SubCommand};
use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
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

    let app = App::new("QQ CLI")
        .version("0.3")
        .author("Moviendome <estoy@moviendo.me>")
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
            SubCommand::with_name("deploy")
                .about("Deploy with Kamal")
                .alias("d"),
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
                .about("Switch to main branch")
                .alias("gm"),
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
        .or_else(|| NextJs::detect(&current_dir))
        .or_else(|| Nodejs::detect(&current_dir));

    let commands = match project_type {
        Some(pt) => {
            println!("Detected: {}", pt.name());
            pt
        }
        None => {
            println!("Project type found in current directory is not supported.");
            return ExitCode::FAILURE;
        }
    };

    // If no subcommand was provided, show the interactive menu
    if matches.subcommand_name().is_none() {
        return show_interactive_menu(commands);
    }

    let cmd_name = matches.subcommand_name().unwrap();
    execute_command(cmd_name, &*commands)
}

fn execute_command(cmd: &str, commands: &dyn project_type_trait::ProjectTypeCommands) -> ExitCode {
    match cmd {
        "install" => run_command(&commands.install_command()),
        "migrate" => run_optional_command(commands.migrate_command(), "migrate"),
        "console" => run_optional_command(commands.console_command(), "console"),
        "start" => run_optional_command(commands.start_command(), "start"),
        "test" => run_optional_command(commands.test_command(), "test"),
        "routes" => run_optional_command(commands.routes_command(), "routes"),
        "deploy" => run_optional_command(commands.deploy_command(), "deploy"),
        "g" => run_command("git status"),
        "gl" => run_command("git lg"),
        "gp" => run_command("git pull"),
        "gP" => run_command("git push"),
        "gm" => run_command("git checkout main"),
        "ga" => run_command("git commit --amend --no-edit"),
        _ => {
            println!("Command not recognized.");
            ExitCode::FAILURE
        }
    }
}

fn run_optional_command(cmd: Option<String>, name: &str) -> ExitCode {
    match cmd {
        Some(c) => run_command(&c),
        None => {
            println!("'{}' command not supported for this project type.", name);
            ExitCode::SUCCESS
        }
    }
}

fn run_command(command: &str) -> ExitCode {
    match Command::new("sh").arg("-c").arg(command).status() {
        Ok(status) => {
            if status.success() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(status.code().unwrap_or(1) as u8)
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn show_interactive_menu(commands: Box<dyn project_type_trait::ProjectTypeCommands>) -> ExitCode {
    let cmd = inquire::Text::new("Enter Command: ")
        .with_help_message("Enter a valid command")
        .with_autocomplete(&utils::suggester)
        .prompt()
        .unwrap_or_else(|_| "exit".to_string());

    match cmd.as_str() {
        "exit" => {
            println!("Exiting...");
            ExitCode::SUCCESS
        }
        "help" => {
            print_help();
            ExitCode::SUCCESS
        }
        _ => execute_command(&cmd, &*commands),
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  install - Installs dependencies for the project");
    println!("  migrate - Runs database migrations");
    println!("  console - Runs console");
    println!("  start   - Starts the project");
    println!("  test    - Run tests");
    println!("  routes  - Show Routes");
    println!("  deploy  - Deploy with Kamal");
    println!("  g       - Run git status");
    println!("  gl      - Run git log");
    println!("  gp      - Run git pull");
    println!("  gP      - Run git push");
    println!("  gm      - Switch to main branch");
    println!("  ga      - Run git commit --amend --no-edit");
    println!("  exit    - Exit the program");
    println!("  help    - Show this help message");
}
