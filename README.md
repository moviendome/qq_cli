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

1. Fork the repo
2. Create your branch (`git checkout -b feat/my-feature`)
3. Make your changes and add tests
4. Run `cargo test` to make sure everything passes
5. Commit and open a PR

## Support

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/moviendome)

## License

MIT
