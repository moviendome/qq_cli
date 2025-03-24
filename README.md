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

## Automate and Simplify Tasks in Development Environments Executing Context-Aware Commands

QQ CLI is a powerful, intelligent command-line interface that simplifies your development workflow across multiple project types. With automatic project detection and intuitive commands, QQ CLI streamlines common development tasks for Rails, Node.js, Middleman, and Rust projects.

## ✨ Features

- **Smart Project Detection** - Automatically identifies your project type and adapts commands accordingly
- **Framework Support** - Seamlessly works with Ruby on Rails, Node.js, Middleman, and Rust projects
- **Interactive Mode** - Enjoy an intuitive CLI experience with command suggestions and autocompletion
- **Git Integration** - Execute common git commands without switching contexts
- **Unified Command Structure** - Use consistent commands across different project types

## 🚀 Quick Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `qq install` | `qq i` | Install project dependencies |
| `qq migrate` | `qq m` | Run database migrations |
| `qq console` | `qq c` | Launch interactive console |
| `qq start` | `qq s` | Start development server |
| `qq test` | `qq t` | Run test suite |
| `qq routes` | `qq r` | Display application routes |
| `qq g` | | Run git status |
| `qq gl` | | Run git log |
| `qq gp` | | Run git pull |
| `qq gP` | | Run git push |
| `qq gm` | | Switch to main branch |
| `qq ga` | | Amend last commit |

## 🛠️ Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/moviendome/qq_cli.git

# Navigate to project directory
cd qq_cli

# Build with Cargo
cargo build --release or make build

# Add to your PATH (optional)
sudo cp target/release/qq /usr/local/bin or make install
```

## 📖 Usage

Simply run `qq` in your project directory to launch the interactive menu, or use specific commands:

```bash
# Interactive mode
qq

# Direct command
qq install

# Get help
qq --help
```

## 🔮 Roadmap

- **Expanded Framework Support** - Adding support for more languages and frameworks
- **Custom Command Configuration** - Define your own commands via configuration files

## 🤝 Contributing

Contributions are welcome and appreciated! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## ❤️ Support

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/moviendome)

If you find QQ CLI useful, consider supporting its development!

## 📜 License

QQ CLI is available under the MIT License. See the [LICENSE](LICENSE) file for more information.
