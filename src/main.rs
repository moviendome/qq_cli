use clap::{App, SubCommand};
use qq_cli::config::{allow_project, AllowOutcome, ConfigPaths};
use qq_cli::resolver::{resolve, Resolution};
use std::env;
use std::path::Path;
use std::process::{Command, ExitCode};

const LOGO: &str = r#"
 ________  ________           ________  ___       ___
|\   __  \|\   __  \         |\   ____\|\  \     |\  \
\ \  \|\  \ \  \|\  \        \ \  \___|\ \  \    \ \  \
 \ \  \\\  \ \  \\\  \        \ \  \    \ \  \    \ \  \
  \ \  \\\  \ \  \\\  \        \ \  \____\ \  \____\ \  \
   \ \_____  \ \_____  \        \ \_______\ \_______\ \__\
    \|___| \__\|___| \__\        \|_______|\|_______|\|__|
          \|__|     \|__|
"#;

/// The seven standard project commands, in their traditional help order.
const CANONICAL_COMMANDS: [&str; 7] = [
    "install", "migrate", "console", "start", "test", "routes", "deploy",
];

/// (subcommand, shell command, description) — hardcoded and un-overridable.
const GIT_SHORTCUTS: [(&str, &str, &str); 7] = [
    ("g", "git status", "Run git status"),
    ("gl", "git lg", "Run git log"),
    ("gp", "git pull", "Run git pull"),
    ("gP", "git push", "Run git push"),
    ("gm", "git checkout main", "Switch to main branch"),
    ("ga", "git commit --amend --no-edit", "Run git amend"),
    ("gM", "git merge -", "Merge previous branch"),
];

/// Names a config can never claim (R11).
const RESERVED: [&str; 10] = [
    "g", "gl", "gp", "gP", "gm", "ga", "gM", "help", "exit", "allow",
];

fn main() -> ExitCode {
    println!("{}", LOGO);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let paths = ConfigPaths::discover(&current_dir);

    let help_or_version = env::args()
        .nth(1)
        .map_or(false, |a| matches!(a.as_str(), "-h" | "--help" | "-V" | "--version"));

    // --help/--version must work in any directory, so a resolution failure
    // (e.g. malformed config) falls back to the reserved-only surface.
    let outcome = match resolve(&current_dir, &paths) {
        Ok(outcome) => outcome,
        Err(e) => {
            if help_or_version {
                build_app(&[]).get_matches(); // clap renders help/version and exits
                return ExitCode::SUCCESS;
            }
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    // The notice prints before argument parsing so it appears even when the
    // user invokes a config-defined command that isn't registered yet.
    if let Some(path) = &outcome.untrusted_project_config {
        println!(
            "Ignoring unapproved config {} — run 'qq allow' to trust it.",
            path.display()
        );
    }

    let project_commands: Vec<String> = outcome
        .resolution
        .as_ref()
        .map(command_list)
        .unwrap_or_default();
    let matches = build_app(&project_commands).get_matches();

    // `qq allow` works everywhere, including before any type resolves.
    if matches.subcommand_name() == Some("allow") {
        return handle_allow(&paths);
    }

    let resolution = match outcome.resolution {
        Some(resolution) => {
            if let Some(name) = &resolution.display_name {
                println!("Detected: {}", name);
            }
            resolution
        }
        None => {
            println!("Project type found in current directory is not supported.");
            return ExitCode::FAILURE;
        }
    };

    match matches.subcommand_name() {
        None => show_interactive_menu(&resolution, &current_dir, &paths),
        Some(cmd) => execute_command(cmd, &resolution, &current_dir),
    }
}

/// Project commands for this resolution: the standard seven in canonical
/// order first (when available), then config-defined extras; reserved names
/// are filtered out so a config can never shadow them.
fn command_list(resolution: &Resolution) -> Vec<String> {
    let names = resolution.command_names();
    let mut list: Vec<String> = CANONICAL_COMMANDS
        .iter()
        .filter(|c| names.iter().any(|n| n == *c))
        .map(|s| s.to_string())
        .collect();
    for name in names {
        if !CANONICAL_COMMANDS.contains(&name.as_str()) && !RESERVED.contains(&name.as_str()) {
            list.push(name);
        }
    }
    list
}

fn build_app(project_commands: &[String]) -> App<'static> {
    let mut app = App::new("QQ CLI")
        .version("0.5")
        .author("Moviendome <estoy@moviendo.me>");

    // The canonical seven are always parseable — even when no type resolves —
    // so `qq start` in an unrecognized directory and `qq migrate` on a type
    // without migrate reach today's exact messages instead of a clap error.
    for name in CANONICAL_COMMANDS {
        let mut sub = SubCommand::with_name(name).about(about_for(name));
        if let Some(alias) = alias_for(name) {
            sub = sub.alias(alias);
        }
        app = app.subcommand(sub);
    }
    for name in project_commands {
        if !CANONICAL_COMMANDS.contains(&name.as_str()) {
            app = app.subcommand(
                SubCommand::with_name(name.as_str()).about(about_for(name)),
            );
        }
    }
    for (name, _, about) in GIT_SHORTCUTS {
        app = app.subcommand(SubCommand::with_name(name).about(about).alias(name));
    }
    app = app.subcommand(
        SubCommand::with_name("allow").about("Trust this directory's .qq.toml config"),
    );
    app.after_help("Use 'qq [command]' to execute a command.")
}

fn about_for(name: &str) -> &'static str {
    match name {
        "install" => "Installs dependencies for the project",
        "migrate" => "Runs database migrations",
        "console" => "Runs console",
        "start" => "Starts the project",
        "test" => "Run tests",
        "routes" => "Show Routes",
        "deploy" => "Deploy with Kamal",
        _ => "Run a project command",
    }
}

fn alias_for(name: &str) -> Option<&'static str> {
    match name {
        "install" => Some("i"),
        "migrate" => Some("m"),
        "console" => Some("c"),
        "start" => Some("s"),
        "test" => Some("t"),
        "routes" => Some("r"),
        "deploy" => Some("d"),
        _ => None,
    }
}

fn handle_allow(paths: &ConfigPaths) -> ExitCode {
    match allow_project(paths) {
        Ok(AllowOutcome::Approved(path)) => {
            println!("Approved {}", path.display());
            ExitCode::SUCCESS
        }
        Ok(AllowOutcome::NothingToApprove) => {
            println!("No .qq.toml found in this directory.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn execute_command(cmd: &str, resolution: &Resolution, dir: &Path) -> ExitCode {
    if let Some((_, shell, _)) = GIT_SHORTCUTS.iter().find(|(name, _, _)| *name == cmd) {
        return run_command(shell);
    }
    match resolution.command(cmd, dir) {
        Some(command) => run_command(&command),
        None => {
            let known = CANONICAL_COMMANDS.contains(&cmd)
                || resolution.command_names().iter().any(|n| n == cmd);
            if known {
                println!("'{}' command not supported for this project type.", cmd);
                ExitCode::SUCCESS
            } else {
                println!("Command not recognized.");
                ExitCode::FAILURE
            }
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

fn show_interactive_menu(resolution: &Resolution, dir: &Path, paths: &ConfigPaths) -> ExitCode {
    let mut suggestions: Vec<String> = command_list(resolution);
    suggestions.extend(GIT_SHORTCUTS.iter().map(|(name, _, _)| name.to_string()));
    suggestions.extend(["allow", "help", "exit"].map(String::from));

    let autocomplete = move |val: &str| -> Result<Vec<String>, inquire::CustomUserError> {
        Ok(suggestions
            .iter()
            .filter(|cmd| cmd.starts_with(val))
            .cloned()
            .collect())
    };

    let cmd = inquire::Text::new("Enter Command: ")
        .with_help_message("Enter a valid command")
        .with_autocomplete(autocomplete)
        .prompt()
        .unwrap_or_else(|_| "exit".to_string());

    match cmd.as_str() {
        "exit" => {
            println!("Exiting...");
            ExitCode::SUCCESS
        }
        "help" => {
            print_help(resolution);
            ExitCode::SUCCESS
        }
        "allow" => handle_allow(paths),
        _ => execute_command(&cmd, resolution, dir),
    }
}

fn print_help(resolution: &Resolution) {
    println!("Available commands:");
    for name in command_list(resolution) {
        println!("  {:<7} - {}", name, about_for(&name));
    }
    for (name, _, about) in GIT_SHORTCUTS {
        println!("  {:<7} - {}", name, about);
    }
    println!("  allow   - Trust this directory's .qq.toml config");
    println!("  exit    - Exit the program");
    println!("  help    - Show this help message");
}
