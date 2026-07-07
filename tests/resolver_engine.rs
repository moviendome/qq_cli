//! U4: resolution engine tests — acceptance examples, layering, trust gate,
//! config errors, detection order, and differential parity against the
//! legacy chain (which still exists until U5 deletes it).

use qq_cli::config::{allow_project, AllowOutcome, ConfigPaths};
use qq_cli::resolver::{resolve, ResolveOutcome};
use qq_cli::{detect_project, project_command};
use std::fs::{create_dir, create_dir_all, File};
use std::io::Write as _;
use std::path::Path;
use std::sync::Mutex;
use tempfile::{tempdir, TempDir};

static CWD_LOCK: Mutex<()> = Mutex::new(());

fn in_dir<T>(dir: &Path, f: impl FnOnce() -> T) -> T {
    let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let old = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir).unwrap();
    let result = f();
    std::env::set_current_dir(old).unwrap();
    result
}

/// A project dir plus an isolated global config dir (never touches $HOME).
struct Fixture {
    project: TempDir,
    global: TempDir,
}

impl Fixture {
    fn new() -> Fixture {
        Fixture {
            project: tempdir().unwrap(),
            global: tempdir().unwrap(),
        }
    }

    fn paths(&self) -> ConfigPaths {
        ConfigPaths::with_global_dir(self.project.path(), self.global.path())
    }

    fn dir(&self) -> &Path {
        self.project.path()
    }

    fn touch(&self, name: &str) -> &Self {
        File::create(self.dir().join(name)).unwrap();
        self
    }

    fn mkdir(&self, name: &str) -> &Self {
        create_dir(self.dir().join(name)).unwrap();
        self
    }

    fn project_config(&self, content: &str) -> &Self {
        let mut f = File::create(self.dir().join(".qq.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        self
    }

    fn approved_project_config(&self, content: &str) -> &Self {
        self.project_config(content);
        match allow_project(&self.paths()).unwrap() {
            AllowOutcome::Approved(_) => {}
            AllowOutcome::NothingToApprove => panic!("expected a config to approve"),
        }
        self
    }

    fn global_config(&self, content: &str) -> &Self {
        create_dir_all(self.global.path()).unwrap();
        let mut f = File::create(self.global.path().join("config.toml")).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        self
    }

    fn resolve(&self) -> ResolveOutcome {
        resolve(self.dir(), &self.paths()).unwrap()
    }

    fn command(&self, name: &str) -> Option<String> {
        self.resolve()
            .resolution
            .expect("expected a resolution")
            .command(name, self.dir())
    }
}

// ------------------------------------------------------ acceptance examples

#[test]
fn ae1_partial_override_keeps_builtin_fallback() {
    // Rails project, config redefines only `test` — `start` still resolves
    // to the built-in Rails behavior (bin/dev fallback chain).
    let fx = Fixture::new();
    fx.touch("Gemfile")
        .approved_project_config("[commands]\ntest = [{ run = \"bin/rails test:system\" }]\n");

    assert_eq!(fx.command("test").as_deref(), Some("bin/rails test:system"));
    assert_eq!(fx.command("start").as_deref(), Some("bin/rails s"));

    fx.mkdir("bin");
    fx.touch("bin/dev");
    assert_eq!(fx.command("start").as_deref(), Some("bin/dev"));
}

#[test]
fn ae2_declared_type_beats_detection_with_builtin_fallback() {
    // Cargo.toml present, but the config declares a custom type: the custom
    // type wins; a command it omits falls back to the shadowed built-in.
    let fx = Fixture::new();
    fx.touch("Cargo.toml").approved_project_config(
        "[types.mytool]\nname = \"MyTool\"\n\n[types.mytool.commands]\nstart = [{ run = \"make serve\" }]\n",
    );

    let outcome = fx.resolve();
    let resolution = outcome.resolution.unwrap();
    assert_eq!(resolution.display_name.as_deref(), Some("MyTool"));
    assert_eq!(resolution.command("start", fx.dir()).as_deref(), Some("make serve"));
    // `install` omitted by the custom type — falls back to Rust's built-in.
    assert_eq!(resolution.command("install", fx.dir()).as_deref(), Some("cargo build"));
}

#[test]
fn ae3_fallback_list_resolves_rails_start_without_bin_dev() {
    let fx = Fixture::new();
    fx.touch("Gemfile");
    assert_eq!(fx.command("start").as_deref(), Some("bin/rails s"));
}

#[test]
fn ae5_trust_gate_blocks_until_allowed_and_stales_on_change() {
    let fx = Fixture::new();
    fx.touch("Gemfile")
        .project_config("[commands]\ntest = [{ run = \"evil test\" }]\n");

    // Unapproved: notice surfaced, override ignored, built-in behavior runs.
    let outcome = fx.resolve();
    assert!(outcome.untrusted_project_config.is_some());
    let resolution = outcome.resolution.unwrap();
    assert_eq!(resolution.display_name.as_deref(), Some("Rails"));
    assert_eq!(resolution.command("test", fx.dir()), None); // no spec/ or test/

    // Approved: override applies.
    allow_project(&fx.paths()).unwrap();
    let outcome = fx.resolve();
    assert!(outcome.untrusted_project_config.is_none());
    assert_eq!(fx.command("test").as_deref(), Some("evil test"));

    // Content change invalidates the approval.
    fx.project_config("[commands]\ntest = [{ run = \"even more evil\" }]\n");
    let outcome = fx.resolve();
    assert!(outcome.untrusted_project_config.is_some());
    assert_eq!(fx.command("test"), None);
}

// ------------------------------------------------------------ layering

#[test]
fn per_project_beats_global_beats_nothing() {
    let fx = Fixture::new();
    fx.touch("Cargo.toml")
        .global_config("[commands]\nlint = [{ run = \"cargo clippy\" }]\n");
    assert_eq!(fx.command("lint").as_deref(), Some("cargo clippy"));

    fx.approved_project_config("[commands]\nlint = [{ run = \"cargo clippy --all\" }]\n");
    assert_eq!(fx.command("lint").as_deref(), Some("cargo clippy --all"));
}

#[test]
fn global_type_shadowing_merges_per_command() {
    // Global config overrides one command of the built-in rails type; the
    // rest of rails' commands keep working (per-command merge by type key).
    let fx = Fixture::new();
    fx.touch("Gemfile")
        .global_config("[types.rails.commands]\ntest = [{ run = \"bin/custom-test\" }]\n");

    assert_eq!(fx.command("test").as_deref(), Some("bin/custom-test"));
    assert_eq!(fx.command("install").as_deref(), Some("bundle install"));
    assert_eq!(fx.command("start").as_deref(), Some("bin/rails s"));
}

#[test]
fn typeless_commands_work_in_unrecognized_directory() {
    let fx = Fixture::new();
    fx.approved_project_config("[commands]\nbuild = [{ run = \"make\" }]\n");

    let resolution = fx.resolve().resolution.unwrap();
    assert_eq!(resolution.display_name, None);
    assert_eq!(resolution.command("build", fx.dir()).as_deref(), Some("make"));
}

#[test]
fn unrecognized_directory_without_config_resolves_nothing() {
    let fx = Fixture::new();
    assert!(fx.resolve().resolution.is_none());
}

// ------------------------------------------------------------- errors (R7)

#[test]
fn malformed_project_config_errors_naming_the_file() {
    let fx = Fixture::new();
    fx.touch("Gemfile").project_config("not [valid toml");
    let err = resolve(fx.dir(), &fx.paths()).unwrap_err();
    assert!(err.to_string().contains(".qq.toml"));
}

#[test]
fn malformed_global_config_errors_naming_the_file() {
    let fx = Fixture::new();
    fx.touch("Gemfile").global_config("[commands\nbroken");
    let err = resolve(fx.dir(), &fx.paths()).unwrap_err();
    assert!(err.to_string().contains("config.toml"));
}

#[test]
fn unreadable_present_project_config_errors_not_skips() {
    // A directory named .qq.toml is present but unreadable as a file.
    let fx = Fixture::new();
    fx.touch("Gemfile").mkdir(".qq.toml");
    let err = resolve(fx.dir(), &fx.paths()).unwrap_err();
    assert!(err.to_string().contains(".qq.toml"));
}

#[test]
fn allow_refuses_malformed_config() {
    let fx = Fixture::new();
    fx.project_config("broken = [");
    assert!(allow_project(&fx.paths()).is_err());
}

#[test]
fn allow_without_config_reports_nothing_to_approve() {
    let fx = Fixture::new();
    assert!(matches!(
        allow_project(&fx.paths()).unwrap(),
        AllowOutcome::NothingToApprove
    ));
}

// --------------------------------------------- detection order (from U3)

#[test]
fn detection_order_matches_legacy_chain() {
    let cases: Vec<(Fixture, &str)> = vec![
        (
            {
                let fx = Fixture::new();
                fx.touch("Gemfile").mkdir("source");
                fx
            },
            "Middleman",
        ),
        (
            {
                let fx = Fixture::new();
                fx.touch("Anchor.toml").touch("Cargo.toml");
                fx
            },
            "Anchor",
        ),
        (
            {
                let fx = Fixture::new();
                fx.touch("package.json").touch("next.config.js");
                fx
            },
            "NextJS",
        ),
        (
            {
                let fx = Fixture::new();
                fx.touch("package.json").touch("next.config.ts");
                fx
            },
            "NextJS",
        ),
        (
            {
                let fx = Fixture::new();
                fx.touch("package.json");
                fx
            },
            "Nodejs",
        ),
    ];
    for (fx, expected) in cases {
        let resolution = fx.resolve().resolution.unwrap();
        assert_eq!(resolution.display_name.as_deref(), Some(expected));
    }
}

// ------------------------------- differential parity vs the legacy chain

/// For every fixture type and every project command, the resolver must
/// produce exactly what the legacy chain produces — active falsification of
/// parity while both mechanisms exist (legacy is deleted in U5).
#[test]
fn differential_parity_legacy_vs_resolver() {
    const COMMANDS: [&str; 7] = ["install", "migrate", "console", "start", "test", "routes", "deploy"];

    let mut fixtures: Vec<(&str, Fixture)> = Vec::new();

    let plain = |markers: &[&str], dirs: &[&str]| {
        let fx = Fixture::new();
        for m in markers {
            fx.touch(m);
        }
        for d in dirs {
            fx.mkdir(d);
        }
        fx
    };

    fixtures.push(("middleman", plain(&["Gemfile"], &["source"])));
    fixtures.push(("rails-bare", plain(&["Gemfile"], &[])));
    fixtures.push(("rails-full", {
        let fx = plain(&["Gemfile"], &["spec", "test", "bin", ".kamal"]);
        fx.touch("bin/dev");
        fx
    }));
    fixtures.push(("rails-testdir", plain(&["Gemfile"], &["test"])));
    fixtures.push(("anchor", plain(&["Anchor.toml"], &[])));
    fixtures.push(("anchor-kamal", plain(&["Anchor.toml"], &[".kamal"])));
    fixtures.push(("rust", plain(&["Cargo.toml"], &[])));
    fixtures.push(("rust-kamal", plain(&["Cargo.toml"], &[".kamal"])));
    fixtures.push(("nextjs", plain(&["package.json", "next.config.js"], &[])));
    fixtures.push(("nodejs", plain(&["package.json"], &[".kamal"])));

    for (label, fx) in &fixtures {
        let legacy_project = detect_project(fx.dir()).expect(label);
        let resolution = fx.resolve().resolution.expect(label);

        assert_eq!(
            resolution.display_name.as_deref(),
            Some(legacy_project.name()),
            "{label}: detected type"
        );

        for command in COMMANDS {
            let legacy = in_dir(fx.dir(), || project_command(&*legacy_project, command));
            let new = resolution.command(command, fx.dir());
            assert_eq!(new, legacy, "{label}: `{command}` must match legacy");
        }
    }
}
