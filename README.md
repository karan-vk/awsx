# awsx

A fast, interactive command-line utility in Rust that allows you to seamlessly switch between AWS profiles. Think of it as `kubectx` with `fzf`, but specifically tailored for AWS credentials and configurations.

It reads directly from `~/.aws/config` and `~/.aws/credentials` and provides a native fuzzy-finding interface built with `inquire`.

## Features

- **Fast & Lightweight:** Built in Rust. Single natively-compiled binary.
- **Native Fuzzy Search:** No need to install `fzf` or other external tools. The interactive prompt is built-in.
- **Secure:** Reads your standard AWS files locally without copying or parsing data elsewhere.
- **Smart Shell Hooks:** Gracefully bypasses UNIX parent-process environment limitations so your exported `AWS_PROFILE` seamlessly updates in your current shell session.

## Installation

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) installed.

### Installing with Homebrew (macOS/Linux)

You can install `awsx` via Homebrew by tapping into your repository (or using the provided formula):

```sh
brew install karan-vijayakumar/tap/awsx
```

### Installing with a Script (Linux/macOS)

For a quick installation without Rust installed, you can use our installation script:

```sh
curl -sSL https://raw.githubusercontent.com/karan-vijayakumar/awsx/main/scripts/install.sh | bash
```

### Installing with Cargo

```sh
cargo install --path .
```

This will build and install the `awsx` binary to `~/.cargo/bin`. Make sure this directory is added to your system's `$PATH`.

## Setup

Because a child process cannot export environment variables to its parent shell, `awsx` uses shell hooks to wrap the command. **You must add the initialization hook to your shell configuration file for `awsx` to work!**

### Zsh
Add this to your `~/.zshrc`:
```sh
eval "$(awsx init zsh)"
```

### Bash
Add this to your `~/.bashrc`:
```sh
eval "$(awsx init bash)"
```

### Fish
Add this to your `~/.config/fish/config.fish`:
```fish
awsx init fish | source
```

*(Note: after adding the line, restart your terminal or `source` the config file)*

## Usage

Simply run:
```sh
awsx
```

Use your arrow keys to select a profile, type to fuzzy-search, and hit `<Enter>` to switch. The selected profile will instantly be applied as your `AWS_PROFILE` in your terminal session!

## Continuous Integration

The project includes a GitHub Actions workflow that:
- Runs `cargo fmt` and `cargo clippy`.
- Executes tests on both **macOS** and **Linux**.
- Automatically builds and packages release binaries for **AMD64** (x86_64) and **ARM64** (aarch64) for both platforms when a new version tag (e.g., `v0.1.0`) is pushed to the repository.

## License
MIT
