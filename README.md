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

Context-aware CLI that runs the right commands for your project type.

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

| Project | Detection |
|---------|-----------|
| Rails | `Gemfile` |
| Middleman | `Gemfile` + `source/` |
| Next.js | `package.json` + `next.config.{js,mjs,ts}` |
| Node.js | `package.json` |
| Rust | `Cargo.toml` |

## Commands

### Project Commands

| Command | Alias | Rails | Middleman | Next.js | Node.js | Rust |
|---------|-------|-------|-----------|---------|---------|------|
| `install` | `i` | `bundle install` | `bundle install` | `npm install` | `npm install` | `cargo build` |
| `start` | `s` | `bin/dev` or `bin/rails s` | `bundle exec middleman serve` | `npm run dev` | `npm start` | `cargo run` |
| `test` | `t` | `bin/rspec` or `bin/rails test` | - | `npm test` | `npm test` | `cargo test` |
| `console` | `c` | `bin/rails c` | - | - | - | - |
| `migrate` | `m` | `bin/rails db:migrate` | - | - | - | - |
| `routes` | `r` | `bin/rails routes` | - | - | - | - |

### Git Commands

| Command | Action |
|---------|--------|
| `g` | `git status` |
| `gl` | `git log` |
| `gp` | `git pull` |
| `gP` | `git push` |
| `gm` | `git checkout main` |
| `ga` | `git commit --amend --no-edit` |

## Examples

```bash
# In a Rails project
$ qq start
Detected: Rails
# Runs: bin/dev

# In a Next.js project
$ qq test
Detected: NextJS
# Runs: npm test

# In a Rust project
$ qq i
Detected: Rust
# Runs: cargo build
```

## Support

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/moviendome)

## License

MIT
