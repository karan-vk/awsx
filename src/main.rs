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
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the shell hook (e.g. `awsx init bash`)
    Init {
        #[arg(help = "The shell to generate hooks for (bash, zsh, fish, powershell)")]
        shell: String,
    },
    /// Internal command to perform the switch (used by shell hook)
    #[command(hide = true)]
    Switch {
        /// Optional profile to switch to without interactive prompt
        profile: Option<String>,
    },
    /// Internal command to get profiles for autocomplete
    #[command(hide = true)]
    ListProfiles,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Init { shell }) => {
            shell_hooks::generate_hook(&shell);
        }
        Some(Commands::ListProfiles) => {
            let profiles = aws_config::get_aws_profiles();
            for p in profiles {
                println!("{}", p);
            }
        }
        Some(Commands::Switch { profile }) => {
            let profiles = aws_config::get_aws_profiles();

            if let Some(target) = profile {
                // Validate if the requested profile exists
                if profiles.contains(&target) {
                    println!("{}", target);
                } else {
                    eprintln!("Error: Profile '{}' not found.", target);
                }
            } else {
                // Interactive mode
                if let Some(selected) = cli::select_profile(profiles) {
                    println!("{}", selected);
                }
            }
        }
        None => {
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
            eprintln!(
                "Once the hook is loaded, simply run `awsx` to select a profile interactively, or `awsx <profile>` to switch directly."
            );
        }
    }
}
