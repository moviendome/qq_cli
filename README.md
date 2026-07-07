# QQ CLI

```
 ________  ________           ________  ___       ___
|\   __  \|\   __  \         |\   ____\|\  \     |\  \
\ \  \|\  \ \  \|\  \        \ \  \___|\ \  \    \ \  \
 \ \  \\\  \ \  \\\  \        \ \  \    \ \  \    \ \  \
  \ \  \\\  \ \  \\\  \        \ \  \____\ \  \____\ \  \
   \ \_____  \ \_____  \        \ \_______\ \_______\ \__\
    \|___| \__\|___| \__\        \|_______|\|_______|\|__|
          \|__|     \|__|
```

![GitHub Workflow Status](http://img.shields.io/github/actions/workflow/status/moviendome/qq_cli/rust.yml?branch=main&style=for-the-badge)
![Rust](https://img.shields.io/badge/Made%20with%20Rust-blueviolet.svg?style=for-the-badge&logo=rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

**One command for every project.** QQ CLI detects your project type and runs the right command — no more remembering if it's `bundle install`, `npm install`, or `cargo build`.

## Why

Every framework has its own CLI. Rails has `bin/rails`, Node has `npm`, Rust has `cargo`, Anchor has `anchor`. Switching between projects means switching mental contexts.

QQ gives you a single set of commands (`install`, `start`, `test`, `deploy`) that work across all your projects. It detects the project type automatically and runs the right thing.

## Installation

```bash
git clone https://github.com/moviendome/qq_cli.git
cd qq_cli
make build && make install
```

## Usage

```bash
qq              # Interactive mode with autocomplete
qq start        # Run command directly
qq --help       # Show help
```

## Supported Projects

| Project | Detection | Example Stack |
|---------|-----------|---------------|
| Rails | `Gemfile` | Ruby on Rails |
| Middleman | `Gemfile` + `source/` | Static sites |
| Anchor | `Anchor.toml` | Solana programs |
| Next.js | `package.json` + `next.config.{js,mjs,ts}` | React/Next.js |
| Node.js | `package.json` | Node.js apps |
| Rust | `Cargo.toml` | Rust projects |

Detection runs top-to-bottom — more specific patterns (Middleman, Anchor) are checked before general ones (Rails, Rust).

## Commands

### Project Commands

| Command | Alias | Rails | Middleman | Anchor | Next.js | Node.js | Rust |
|---------|-------|-------|-----------|--------|---------|---------|------|
| `install` | `i` | `bundle install` | `bundle install` | `anchor build` | `npm install` | `npm install` | `cargo build` |
| `start` | `s` | `bin/dev` or `bin/rails s` | `bundle exec middleman serve` | `anchor localnet` | `npm run dev` | `npm start` | `cargo run` |
| `test` | `t` | `bin/rspec` or `bin/rails test` | - | `anchor test` | `npm test` | `npm test` | `cargo test` |
| `console` | `c` | `bin/rails c` | - | - | - | - | - |
| `migrate` | `m` | `bin/rails db:migrate` | - | - | - | - | - |
| `routes` | `r` | `bin/rails routes` | - | - | - | - | - |
| `deploy` | `d` | `kamal deploy` * | `kamal deploy` * | `anchor deploy` | `kamal deploy` * | `kamal deploy` * | `kamal deploy` * |

\* Available when `.kamal` directory exists

### Git Shortcuts

| Command | Action |
|---------|--------|
| `g` | `git status` |
| `gl` | `git log` |
| `gp` | `git pull` |
| `gP` | `git push` |
| `gm` | `git checkout main` |
| `ga` | `git commit --amend --no-edit` |
| `gM` | `git merge -` (merge previous branch) |

## Configuration

QQ reads two optional config files, written in the same format as its built-in project types:

| File | Scope | Trust |
|------|-------|-------|
| `.qq.toml` in the project directory | That project only | Requires one-time `qq allow` |
| `~/.config/qq/config.toml` | Everywhere | Implicitly trusted |

Precedence is merge-based, most explicit wins per command: per-project config > global config > built-in detection — a config that redefines only `test` leaves every other command's built-in behavior intact, and a type you declare beats one detection guessed.

Because a project's `.qq.toml` defines shell commands, QQ ignores it until you approve it once with `qq allow` (re-approval is required whenever the file changes). Git shortcuts and `help`, `exit`, `allow` are reserved — a config can never redefine them.

### Override a command

```toml
# .qq.toml — redefine only `test`; everything else stays built-in
[commands]
test = [{ run = "bin/rails test:system" }]
```

### Add your own commands

```toml
# .qq.toml or ~/.config/qq/config.toml
[commands]
lint = [{ run = "cargo clippy --all-targets" }]
```

New commands show up in `qq` autocomplete and run as `qq lint`.

### Define a whole project type

```toml
# ~/.config/qq/config.toml — detected in any directory with a justfile
[types.just]
name = "Just"
detect = ["justfile"]

[types.just.commands]
install = [{ run = "just setup" }]
test = [{ run = "just test" }]
start = [
    { run = "just dev", when_path = "dev.env" },
    { run = "just run" },
]
```

Candidates are tried in order; the first whose condition passes wins. Conditions are `when_path` (a path exists in the project) and `when_bin` (a binary answers `--version`) — that's the whole language, by design.

## Examples

```bash
# In a Rails project
$ qq start
Detected: Rails
# Runs: bin/dev

# In an Anchor project
$ qq test
Detected: Anchor
# Runs: anchor test

# In a Rust project
$ qq i
Detected: Rust
# Runs: cargo build
```

## Contributing

Built-in project types are TOML files in `src/definitions/` — the same format as user config. Adding support for a new framework means adding one definition file plus a fixture test; no Rust changes needed.

1. Fork the repo
2. Create your branch (`git checkout -b feat/my-feature`)
3. Make your changes and add tests
4. Run `cargo test` to make sure everything passes
5. Commit and open a PR

## Support

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/moviendome)

## License

MIT
