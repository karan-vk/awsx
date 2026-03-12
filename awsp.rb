class Awsp < Formula
  desc "Interactive AWS profile switcher"
  homepage "https://github.com/karan-vijayakumar/awsp"
  url "https://github.com/karan-vijayakumar/awsp/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256" # Run 'shasum -a 256 v0.1.0.tar.gz' to get this
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "awsp", shell_output("#{bin}/awsp --help")
  end

  def caveats
    <<~EOS
      To use awsp, you need to add the shell hook to your configuration:

      Zsh:
        echo 'eval "$(awsp init zsh)"' >> ~/.zshrc

      Bash:
        echo 'eval "$(awsp init bash)"' >> ~/.bashrc

      Fish:
        echo 'awsp init fish | source' >> ~/.config/fish/config.fish
    EOS
  end
end
