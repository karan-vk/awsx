use clap::{Parser, Subcommand};

mod aws_config;
mod cli;
mod shell_hooks;

#[derive(Parser)]
#[command(name = "awsp")]
#[command(about = "Interactive AWS profile switcher", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Internal flag used by the shell hook to trigger the interactive selection
    #[arg(long, hide = true)]
    select: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the shell hook (e.g. `awsp init bash`)
    Init {
        #[arg(help = "The shell to generate hooks for (bash, zsh, fish)")]
        shell: String,
    },
}

fn main() {
    let args = Cli::parse();

    if let Some(Commands::Init { shell }) = args.command {
        shell_hooks::generate_hook(&shell);
        return;
    }

    if args.select {
        let profiles = aws_config::get_aws_profiles();
        if let Some(selected) = cli::select_profile(profiles) {
            println!("{}", selected);
        }
    } else {
        // Just running `awsp` without the shell hook will not be able to export the variable.
        // We print a helpful message guiding them to use the hook.
        eprintln!("awsp is designed to be used via its shell hook to export variables.");
        eprintln!();
        eprintln!("To set it up, add the following to your shell configuration:");
        eprintln!("  eval \"$(awsp init zsh)\"     # For Zsh  (~/.zshrc)");
        eprintln!("  eval \"$(awsp init bash)\"    # For Bash (~/.bashrc)");
        eprintln!("  awsp init fish | source       # For Fish (~/.config/fish/config.fish)");
        eprintln!();
        eprintln!("Once the hook is loaded, simply run `awsp` to select a profile interactively.");
    }
}
