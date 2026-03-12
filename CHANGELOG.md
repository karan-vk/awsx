# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - Initial Release
### Added
- **Native fuzzy finder**: In-terminal, fast interactive AWS profile switching avoiding `fzf` bounds via `inquire`.
- **AWS Configuration parsing**: Securely reading from standard `~/.aws/config` and `credentials` profiles natively.
- **Smart Shell Hooks**: Automated shell hook generation preventing dirty subprocess environment hacking for:
  - Zsh
  - Bash
  - Fish
  - PowerShell
- **Multi-architecture release binaries**: Automated CI pipeline distributing static Linux + standard MacOS + standard Windows builds (AMD64 & ARM64 supported on all platforms).
