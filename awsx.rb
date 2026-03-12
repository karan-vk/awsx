class Awsx < Formula
  desc "Interactive AWS profile switcher"
  homepage "https://github.com/karan-vk/awsx"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/karan-vk/awsx/releases/download/v#{version}/awsx-macos-amd64.tar.gz"
      sha256 "REPLACE_WITH_MACOS_AMD64_SHA256"
    elsif Hardware::CPU.arm?
      url "https://github.com/karan-vk/awsx/releases/download/v#{version}/awsx-macos-arm64.tar.gz"
      sha256 "REPLACE_WITH_MACOS_ARM64_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/karan-vk/awsx/releases/download/v#{version}/awsx-linux-amd64.tar.gz"
      sha256 "REPLACE_WITH_LINUX_AMD64_SHA256"
    elsif Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url "https://github.com/karan-vk/awsx/releases/download/v#{version}/awsx-linux-arm64.tar.gz"
      sha256 "REPLACE_WITH_LINUX_ARM64_SHA256"
    end
  end

  def install
    bin.install "awsx"
  end

  test do
    assert_match "awsx", shell_output("#{bin}/awsx --help")
  end

  def caveats
    <<~EOS
      To use awsx, you need to add the shell hook to your configuration:

      Zsh:
        echo 'eval "$(awsx init zsh)"' >> ~/.zshrc

      Bash:
        echo 'eval "$(awsx init bash)"' >> ~/.bashrc

      Fish:
        echo 'awsx init fish | source' >> ~/.config/fish/config.fish
    EOS
  end
end
