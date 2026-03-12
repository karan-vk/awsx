use clap::{Parser, Subcommand};

mod aws_config;
mod cli;
mod shell_hooks;

#[derive(Parser)]
#[command(name = "awsx")]
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
    /// Initialize the shell hook (e.g. `awsx init bash`)
    Init {
        #[arg(help = "The shell to generate hooks for (bash, zsh, fish, powershell)")]
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
        // Just running `awsx` without the shell hook will not be able to export the variable.
        // We print a helpful message guiding them to use the hook.
        eprintln!("awsx is designed to be used via its shell hook to export variables.");
        eprintln!();
        eprintln!("To set it up, add the following to your shell configuration:");
        eprintln!("  eval \"$(awsx init zsh)\"               # For Zsh  (~/.zshrc)");
        eprintln!("  eval \"$(awsx init bash)\"              # For Bash (~/.bashrc)");
        eprintln!(
            "  awsx init fish | source                 # For Fish (~/.config/fish/config.fish)"
        );
        eprintln!("  Invoke-Expression (awsx init powershell) # For PowerShell ($PROFILE)");
        eprintln!();
        eprintln!("Once the hook is loaded, simply run `awsx` to select a profile interactively.");
    }
}
