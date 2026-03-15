# 🚀 awsx

**The fastest, simplest way to switch AWS profiles in your terminal.**

`awsx` is a native, interactive command-line utility that lets you switch between AWS profiles with zero friction. If you've ever used `kubectx` or `fzf`, you'll feel right at home.

---

## ✨ Why awsx?

*   **⚡ Blazing Fast:** Built with Rust. It starts instantly and feels snappy.
*   **🪄 Auto-completion:** Native support for Bash, Zsh, Fish, and PowerShell. Type `awsx <TAB>` for a list of profiles.
*   **🏎️ Fast Switching:** Bypass the UI with `awsx <profile-name>` for instant context switching.
*   **🔍 Native Fuzzy Finding:** No need to install `fzf`. Interactive search is built directly into the binary.
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

To print every discovered profile directly to stdout with account and config metadata, run:
```sh
awsx --list
```

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

When you select a profile, the shell wrapper function clears any higher-precedence AWS credential environment variables, then exports `AWS_PROFILE` and `AWS_DEFAULT_PROFILE` to your current session and enables shared config loading with `AWS_SDK_LOAD_CONFIG=1`. That makes the selected profile immediately available to `aws`, `terraform`, `cdk`, and other AWS-aware tools without a manual `export AWS_PROFILE=...` step.

awsx also remembers the last profile you switched to by storing the selected profile in your shared AWS files and restores it in new shell sessions when the hook loads, unless that shell already has `AWS_PROFILE` or `AWS_DEFAULT_PROFILE` set explicitly.

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
