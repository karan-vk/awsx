# Contributing to awsx

First off, thank you for considering contributing to `awsx`! It's people like you that make the open-source community such an amazing place to learn, inspire, and create.

## 🚀 Getting Started

1.  **Fork the repository** on GitHub.
2.  **Clone your fork locally**:
    ```bash
    git clone https://github.com/YOUR_USERNAME/awsx.git
    cd awsx
    ```
3.  **Ensure you have the Rust toolchain installed** ([rustup](https://rustup.rs/) is recommended).

## 🛠️ Development Workflow

*   **Create a branch** for your feature or bug fix:
    ```bash
    git checkout -b feature/my-new-feature
    ```
*   **Write code** and ensure tests pass:
    ```bash
    cargo run -- select
    cargo test
    ```
*   **Format and lint** your code before committing:
    ```bash
    cargo fmt
    cargo clippy -- -D warnings
    ```

## 📝 Submitting a Pull Request

*   Push your branch to your fork.
*   Open a Pull Request against the `main` branch.
*   Describe your changes clearly and link any relevant issues.
*   Our CI will automatically run tests and lint checks. Please ensure they pass!

Thank you for contributing! 💖
