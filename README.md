![GitHub Workflow Status](http://img.shields.io/github/actions/workflow/status/moviendome/qq_cli/rust.yml?branch=main&style=for-the-badge)
![Lua](https://img.shields.io/badge/Made%20with%20Rust-blueviolet.svg?style=for-the-badge&logo=rust)

# QQ CLI

`QQ CLI` is a versatile command-line interface tool designed as a Proof of Concept to automate and simplify tasks in development environments. This first version primarily focuses on Ruby on Rails, Node.js & Rust projects. By intelligently detecting the project type can execute a set of basic commands.

## Features

- **Focused on Ruby on Rails, Node.js & Rust**: Tailored to handle common tasks in Rails, Node.js and Rust projects.
- **Intelligent Project Detection**: Automatically identifies the type of project and executes relevant commands.
- **Basic Command Set**: Supports basic commands like installation, migration, starting servers, and running tests for Rails & Rust projects.
- **Proof of Concept**: Demonstrates the potential for a more extensive tool with broader capabilities in future versions.

## Version 0.1.0

This initial version includes basic functionalities for [Rails](https://rubyonrails.org/), [Node.js](https://nodejs.org/en) and [Rust](https://www.rust-lang.org/) projects:

- Detection of project type.
- Execution of basic commands:
  - `install` (or `i`): Installs dependencies.
  - `migrate` (or `m`): Runs database migrations for Rails projects.
  - `start` (or `s`): Starts the project server.
  - `test` (or `t`): Runs the test suite for Rails projects (Minitest or Rspec), Node.js & Rust.
  - `routes` (or `r`): Show routes for Rails projects (uses fzf if available).

## Future Development

### Enhancing Functionality and User Experience

- **Dynamic Command Execution**: Future versions aim to enhance command execution based on context, allowing for more intelligent and adaptive interactions depending on the project environment and user preferences.

- **Configuration File Support**: To increase flexibility, I plan to implement support for configuration files. This will allow users to customize the behavior according to their specific needs, making the tool more versatile and personalized.

- **Extensibility and Modularity**: A key goal is to design `QQ CLI` with extensibility and modularity in mind. This would enable easy integration of new languages, frameworks, and features, fostering a tool that evolves alongside the ever-changing landscape of software development.

### Leveraging AI and AGI in Development

- **Fast Prototyping with AI Assistance**: Integrating AI-powered tools like [OpenAI ChatGPT](https://chat.openai.com/) and [GitHub Copilot](https://github.com/features/copilot) within Neovim to accelerate development and prototyping.

- **AGI for Automated Code Management**: Exploring AGI's potential in automating code changes, conducting tests and quality assurance, and managing pull requests autonomously.

## Installation

1. Clone the repository:
```bash
git clone https://github.com/moviendome/qq_cli.git
```
   
2. Navigate the project directory:
```bash
cd qq_cli
```

3. Build the project (requires Rust and Cargo):
```bash
cargo build --release
```

4. (Optional) Add the binary to your PATH for easy access:
```bash
sudo cp target/release/qq /usr/local/bin
```

## Usage
Run QQ CLI from the command line within your project directory:

```bash
qq [command]
```

Example for installing dependencies for you project: 
```bash
qq install
```

For more information on each command, use:
```bash
qq --help
```
## Contributing

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/moviendome)

Contributions are welcome! If you have a feature request, bug report, or a pull request, please feel free to contribute.

Fork the repository and create your branch from main.
Make your changes and test them.
Send a pull request with a clear description of your changes.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
