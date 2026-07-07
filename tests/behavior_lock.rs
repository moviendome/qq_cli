//! Behavior-lock suite (U1): characterizes the command resolution behavior
//! established before the config-first cutover. The U5 repoint swapped the
//! resolution API (legacy chain → resolver) with zero assertion changes.
//!
//! The legacy conditional gates were cwd-relative, so those cases ran inside
//! a serialized chdir block; the resolver is dir-parameterized, and the
//! chdir wrapper is kept so the locked assertions run under both models.

use qq_cli::config::ConfigPaths;
use qq_cli::resolver::resolve;
use std::fs::{create_dir, File};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use tempfile::tempdir;

static CWD_LOCK: Mutex<()> = Mutex::new(());

/// Run `f` with the process cwd set to `dir`, serialized across test threads.
fn in_dir<T>(dir: &Path, f: impl FnOnce() -> T) -> T {
    let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let old = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir).unwrap();
    let result = f();
    std::env::set_current_dir(old).unwrap();
    result
}

fn resolved(dir: &Path, command: &str) -> Option<String> {
    let global = tempdir().unwrap(); // isolated: never reads the real HOME
    let paths = ConfigPaths::with_global_dir(dir, global.path());
    resolve(dir, &paths)
        .unwrap()
        .resolution
        .expect("fixture should detect a project type")
        .command(command, dir)
}

fn detected_name(dir: &Path) -> Option<String> {
    let global = tempdir().unwrap();
    let paths = ConfigPaths::with_global_dir(dir, global.path());
    resolve(dir, &paths)
        .unwrap()
        .resolution
        .and_then(|r| r.display_name)
}

/// Same liveness probe the Rails routes command uses.
fn fzf_available() -> bool {
    Command::new("fzf")
        .arg("--version")
        .output()
        .map_or(false, |output| output.status.success())
}

// ---------------------------------------------------------------- fixtures

fn rails_fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("Gemfile")).unwrap();
    dir
}

fn middleman_fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("Gemfile")).unwrap();
    create_dir(dir.path().join("source")).unwrap();
    dir
}

fn anchor_fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("Anchor.toml")).unwrap();
    dir
}

fn rust_fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("Cargo.toml")).unwrap();
    dir
}

fn nextjs_fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();
    File::create(dir.path().join("next.config.js")).unwrap();
    dir
}

fn nodejs_fixture() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("package.json")).unwrap();
    dir
}

// ------------------------------------------------- unconditional commands

#[test]
fn rails_unconditional_commands() {
    let dir = rails_fixture();
    assert_eq!(resolved(dir.path(), "install").as_deref(), Some("bundle install"));
    assert_eq!(resolved(dir.path(), "migrate").as_deref(), Some("bin/rails db:migrate"));
    assert_eq!(resolved(dir.path(), "console").as_deref(), Some("bin/rails c"));
}

#[test]
fn middleman_commands() {
    let dir = middleman_fixture();
    assert_eq!(resolved(dir.path(), "install").as_deref(), Some("bundle install"));
    assert_eq!(resolved(dir.path(), "start").as_deref(), Some("bundle exec middleman serve"));
    assert_eq!(resolved(dir.path(), "migrate"), None);
    assert_eq!(resolved(dir.path(), "console"), None);
    assert_eq!(resolved(dir.path(), "test"), None);
    assert_eq!(resolved(dir.path(), "routes"), None);
}

#[test]
fn anchor_commands() {
    let dir = anchor_fixture();
    assert_eq!(resolved(dir.path(), "install").as_deref(), Some("anchor build"));
    assert_eq!(resolved(dir.path(), "start").as_deref(), Some("anchor localnet"));
    assert_eq!(resolved(dir.path(), "test").as_deref(), Some("anchor test"));
    assert_eq!(resolved(dir.path(), "migrate"), None);
    assert_eq!(resolved(dir.path(), "console"), None);
    assert_eq!(resolved(dir.path(), "routes"), None);
}

#[test]
fn rust_commands() {
    let dir = rust_fixture();
    assert_eq!(resolved(dir.path(), "install").as_deref(), Some("cargo build"));
    assert_eq!(resolved(dir.path(), "start").as_deref(), Some("cargo run"));
    assert_eq!(resolved(dir.path(), "test").as_deref(), Some("cargo test"));
    assert_eq!(resolved(dir.path(), "migrate"), None);
    assert_eq!(resolved(dir.path(), "console"), None);
    assert_eq!(resolved(dir.path(), "routes"), None);
}

#[test]
fn nextjs_commands() {
    let dir = nextjs_fixture();
    assert_eq!(resolved(dir.path(), "install").as_deref(), Some("npm install"));
    assert_eq!(resolved(dir.path(), "start").as_deref(), Some("npm run dev"));
    assert_eq!(resolved(dir.path(), "test").as_deref(), Some("npm test"));
}

#[test]
fn nodejs_commands() {
    let dir = nodejs_fixture();
    assert_eq!(resolved(dir.path(), "install").as_deref(), Some("npm install"));
    assert_eq!(resolved(dir.path(), "start").as_deref(), Some("npm start"));
    assert_eq!(resolved(dir.path(), "test").as_deref(), Some("npm test"));
}

// ------------------------------------------------ cwd-gated conditionals

#[test]
fn rails_start_prefers_bin_dev_when_present() {
    let dir = rails_fixture();
    create_dir(dir.path().join("bin")).unwrap();
    File::create(dir.path().join("bin/dev")).unwrap();
    let cmd = in_dir(dir.path(), || resolved(dir.path(), "start"));
    assert_eq!(cmd.as_deref(), Some("bin/dev"));
}

#[test]
fn rails_start_falls_back_to_rails_s() {
    let dir = rails_fixture();
    let cmd = in_dir(dir.path(), || resolved(dir.path(), "start"));
    assert_eq!(cmd.as_deref(), Some("bin/rails s"));
}

#[test]
fn rails_test_prefers_rspec_with_spec_dir() {
    let dir = rails_fixture();
    create_dir(dir.path().join("spec")).unwrap();
    create_dir(dir.path().join("test")).unwrap();
    let cmd = in_dir(dir.path(), || resolved(dir.path(), "test"));
    assert_eq!(cmd.as_deref(), Some("bin/rspec spec/"));
}

#[test]
fn rails_test_uses_rails_test_with_test_dir_only() {
    let dir = rails_fixture();
    create_dir(dir.path().join("test")).unwrap();
    let cmd = in_dir(dir.path(), || resolved(dir.path(), "test"));
    assert_eq!(cmd.as_deref(), Some("bin/rails test"));
}

#[test]
fn rails_test_unsupported_without_spec_or_test() {
    let dir = rails_fixture();
    let cmd = in_dir(dir.path(), || resolved(dir.path(), "test"));
    assert_eq!(cmd, None);
}

#[test]
fn rails_routes_follows_fzf_availability() {
    let dir = rails_fixture();
    let expected = if fzf_available() {
        "bin/rails routes | fzf -e"
    } else {
        "bin/rails routes"
    };
    assert_eq!(resolved(dir.path(), "routes").as_deref(), Some(expected));
}

#[test]
fn deploy_is_kamal_gated_for_every_defaulting_type() {
    // The `.kamal` deploy gate is a TRAIT DEFAULT: no per-type module mentions
    // deploy except Anchor's override. All five defaulting types are locked.
    let fixtures: Vec<(&str, tempfile::TempDir)> = vec![
        ("Rails", rails_fixture()),
        ("Middleman", middleman_fixture()),
        ("Rust", rust_fixture()),
        ("NextJS", nextjs_fixture()),
        ("Nodejs", nodejs_fixture()),
    ];
    for (name, dir) in &fixtures {
        let without = in_dir(dir.path(), || resolved(dir.path(), "deploy"));
        assert_eq!(without, None, "{name}: deploy without .kamal");

        create_dir(dir.path().join(".kamal")).unwrap();
        let with = in_dir(dir.path(), || resolved(dir.path(), "deploy"));
        assert_eq!(with.as_deref(), Some("kamal deploy"), "{name}: deploy with .kamal");
    }
}

#[test]
fn anchor_deploy_ignores_kamal_gate() {
    let dir = anchor_fixture();
    let without = in_dir(dir.path(), || resolved(dir.path(), "deploy"));
    assert_eq!(without.as_deref(), Some("anchor deploy"));

    create_dir(dir.path().join(".kamal")).unwrap();
    let with = in_dir(dir.path(), || resolved(dir.path(), "deploy"));
    assert_eq!(with.as_deref(), Some("anchor deploy"));
}

// -------------------------------------------------------- detection order

#[test]
fn gemfile_with_source_dir_is_middleman_not_rails() {
    let dir = middleman_fixture();
    assert_eq!(detected_name(dir.path()).as_deref(), Some("Middleman"));
}

#[test]
fn anchor_toml_with_cargo_toml_is_anchor_not_rust() {
    let dir = anchor_fixture();
    File::create(dir.path().join("Cargo.toml")).unwrap();
    assert_eq!(detected_name(dir.path()).as_deref(), Some("Anchor"));
}

#[test]
fn package_json_with_next_config_is_nextjs_not_nodejs() {
    let dir = nextjs_fixture();
    assert_eq!(detected_name(dir.path()).as_deref(), Some("NextJS"));
}

#[test]
fn unrecognized_directory_detects_nothing() {
    let dir = tempdir().unwrap();
    assert!(detected_name(dir.path()).is_none());
}

// -------------------------------------- CLI directory-independence (today)

#[test]
fn help_succeeds_in_unrecognized_directory() {
    let dir = tempdir().unwrap();
    let global = tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_qq"))
        .arg("--help")
        .current_dir(dir.path())
        .env("XDG_CONFIG_HOME", global.path())
        .output()
        .unwrap();
    assert!(output.status.success(), "qq --help must succeed anywhere");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE"), "help text should render");
}

#[test]
fn version_succeeds_in_unrecognized_directory() {
    let dir = tempdir().unwrap();
    let global = tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_qq"))
        .arg("--version")
        .current_dir(dir.path())
        .env("XDG_CONFIG_HOME", global.path())
        .output()
        .unwrap();
    assert!(output.status.success(), "qq --version must succeed anywhere");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.4"), "version should print");
}
