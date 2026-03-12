# 🚀 awsx

**The fastest, simplest way to switch AWS profiles in your terminal.**

`awsx` is a native, interactive command-line utility that lets you switch between AWS profiles with zero friction. If you've ever used `kubectx` or `fzf`, you'll feel right at home.

---

## ✨ Why awsx?

*   **⚡ Blazing Fast:** Built with Rust. It starts instantly and feels snappy.
*   **🔍 Native Fuzzy Finding:** No need to install `fzf`. Interactive search is built directly into the binary.
*   **🛠️ Zero Dependencies:** It reads your standard `~/.aws/config` and `config` files and just works.
*   **📦 Portable:** Statically linked binaries available for Linux (musl), macOS (Intel & Silicon), and Windows.
*   **🔒 Secure:** Your credentials never leave your machine. `awsx` only reads what's already there.

---

## 🚀 Quick Start in 60 Seconds

### 1. Install it
#### **Via Homebrew (macOS/Linux)**
```sh
brew install karan-vk/tap/awsx
```

#### **Via One-Liner Script (Linux/macOS)**
```sh
curl -sSL https://raw.githubusercontent.com/karan-vk/awsx/main/scripts/install.sh | bash
```

### 2. Add the Hook
Since a CLI can't change your shell's environment variables directly, `awsx` needs a small helper in your shell config.

| Shell | Command to add to your config file |
| :--- | :--- |
| **Zsh** | `echo 'eval "$(awsx init zsh)"' >> ~/.zshrc` |
| **Bash** | `echo 'eval "$(awsx init bash)"' >> ~/.bashrc` |
| **Fish** | `echo 'awsx init fish \| source' >> ~/.config/fish/config.fish` |
| **PowerShell** | `echo 'Invoke-Expression (awsx init powershell)' >> $PROFILE` |

*Restart your terminal or source your config file after adding this.*

### 3. Use it!
Just type `awsx` anywhere:
```sh
$ awsx
? Select AWS Profile:
> staging
  production-read-only
  personal-dev-account
```
**Type to filter**, use arrow keys to navigate, and hit `Enter` to switch. 

---

## 🛠️ Advanced Installation

### From Source (Requires Rust)
If you prefer to build it yourself:
```sh
git clone https://github.com/karan-vk/awsx.git
cd awsx
cargo install --path .
```

---

## ⚙️ How it works
`awsx` parses your `~/.aws/config` and `~/.aws/credentials` libraries. It automatically deduplicates profiles and handles the `[profile name]` format correctly.

When you select a profile, the shell wrapper function exports `AWS_PROFILE` to your current session, making it immediately available for `aws`, `terraform`, `cdk`, and other AWS-aware tools.

---

## 🚥 Continuous Integration
We take stability seriously. Every change is:
*   Checked for code style (`cargo fmt`).
*   Linted for best practices (`clippy`).
*   Tested across **macOS**, **Linux**, and **Windows**.
*   Built for **AMD64** and **ARM64** architectures.

---

## 📜 License
MIT © Karan Vijayakumar. See [LICENSE](LICENSE) for details.

---

**Star ⭐ the repo if you find this useful!**
