//! U5: end-to-end tests of the built binary — dynamic command surface,
//! reserved-name collisions, trust-gate notice, and the qq allow flow.
//! Every invocation isolates XDG_CONFIG_HOME so the real HOME is untouched.

use std::fs::File;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::{tempdir, TempDir};

struct Cli {
    project: TempDir,
    global: TempDir,
}

impl Cli {
    fn new() -> Cli {
        Cli {
            project: tempdir().unwrap(),
            global: tempdir().unwrap(),
        }
    }

    fn dir(&self) -> &Path {
        self.project.path()
    }

    fn touch(&self, name: &str) -> &Self {
        File::create(self.dir().join(name)).unwrap();
        self
    }

    fn config(&self, content: &str) -> &Self {
        let mut f = File::create(self.dir().join(".qq.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        self
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_qq"))
            .args(args)
            .current_dir(self.dir())
            .env("XDG_CONFIG_HOME", self.global.path())
            .output()
            .unwrap()
    }

    fn stdout(&self, args: &[&str]) -> String {
        String::from_utf8_lossy(&self.run(args).stdout).into_owned()
    }
}

#[test]
fn ae5_end_to_end_allow_then_override_applies() {
    let cli = Cli::new();
    cli.touch("Gemfile")
        .config("[commands]\ntest = [{ run = \"echo trusted-ok\" }]\n");

    // Before approval: notice printed, override ignored (rails-bare test is
    // unsupported), exit is still success per the not-supported contract.
    let before = cli.run(&["test"]);
    let stdout = String::from_utf8_lossy(&before.stdout);
    assert!(stdout.contains("Ignoring unapproved config"), "notice expected:\n{stdout}");
    assert!(stdout.contains("'test' command not supported"), "override must not run:\n{stdout}");
    assert!(!stdout.contains("trusted-ok"));
    assert!(before.status.success());

    // Approve, then the override runs.
    let allow = cli.stdout(&["allow"]);
    assert!(allow.contains("Approved"), "allow should confirm:\n{allow}");
    let after = cli.stdout(&["test"]);
    assert!(after.contains("trusted-ok"), "override should run:\n{after}");
    assert!(!after.contains("Ignoring unapproved config"));
}

#[test]
fn unapproved_config_command_still_surfaces_the_notice() {
    // `qq hello` before approval: the subcommand isn't registered (clap
    // rejects it), but the notice must still tell the user why.
    let cli = Cli::new();
    cli.touch("Cargo.toml")
        .config("[commands]\nhello = [{ run = \"echo hi\" }]\n");
    let output = cli.run(&["hello"]);
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Ignoring unapproved config"), "notice missing:\n{stdout}");
    assert!(!stdout.contains("hi\n"), "unapproved command must not run");
}

#[test]
fn ae4_config_added_command_runs_as_direct_subcommand() {
    let cli = Cli::new();
    cli.touch("Cargo.toml")
        .config("[commands]\nlint = [{ run = \"echo lint-ran\" }]\n");
    cli.run(&["allow"]);

    let stdout = cli.stdout(&["lint"]);
    assert!(stdout.contains("lint-ran"), "config command must dispatch:\n{stdout}");
}

#[test]
fn reserved_g_cannot_be_overridden_by_config() {
    let cli = Cli::new();
    cli.touch("Cargo.toml")
        .config("[commands]\ng = [{ run = \"echo HACKED\" }]\n");
    cli.run(&["allow"]);

    let stdout = cli.stdout(&["g"]);
    assert!(!stdout.contains("HACKED"), "git shortcut must win the collision:\n{stdout}");
}

#[test]
fn unrecognized_dir_without_config_fails_with_current_message() {
    let cli = Cli::new();
    for args in [&["start"][..], &["g"][..]] {
        let output = cli.run(args);
        assert!(!output.status.success(), "{args:?} should fail");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Project type found in current directory is not supported."),
            "{args:?}:\n{stdout}"
        );
    }
}

#[test]
fn malformed_config_errors_and_names_the_file() {
    let cli = Cli::new();
    cli.touch("Cargo.toml").config("not [valid toml");
    let output = cli.run(&["start"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(".qq.toml"), "error must name the file:\n{stderr}");
}

#[test]
fn allow_without_config_reports_and_succeeds() {
    let cli = Cli::new();
    let output = cli.run(&["allow"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No .qq.toml found"));
}

#[test]
fn detected_line_prints_for_builtin_types() {
    let cli = Cli::new();
    cli.touch("Cargo.toml");
    // `test` maps to `cargo test` inside an empty fixture; it will fail as a
    // command, but the Detected line must print first.
    let stdout = cli.stdout(&["--help"]);
    assert!(stdout.contains("USAGE"));
    let run = cli.stdout(&["migrate"]);
    assert!(run.contains("Detected: Rust"), "Detected line:\n{run}");
    assert!(run.contains("'migrate' command not supported"));
}
