use clap::{Parser, Subcommand};

mod aws_config;
mod cli;
mod shell_hooks;

#[derive(Parser)]
#[command(name = "awsx")]
#[command(about = "Interactive AWS profile switcher", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
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
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run(args: Cli) -> Result<(), String> {
    match args.command {
        Some(Commands::Init { shell }) => {
            shell_hooks::generate_hook(&shell);
            Ok(())
        }
        Some(Commands::ListProfiles) => {
            let profiles = aws_config::get_aws_profiles();
            for p in profiles {
                println!("{}", p);
            }
            Ok(())
        }
        Some(Commands::Switch { profile }) => {
            let profiles = aws_config::get_aws_profiles();

            if let Some(target) = profile {
                if profiles.contains(&target) {
                    let _ = aws_config::update_default_profile(&target);
                    println!("{}", target);
                    Ok(())
                } else {
                    Err(format!("Error: Profile '{}' not found.", target))
                }
            } else {
                // Interactive mode
                if let Some(selected) = cli::select_profile(profiles) {
                    let _ = aws_config::update_default_profile(&selected);
                    println!("{}", selected);
                }
                Ok(())
            }
        }
        None => {
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
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::parse_from(["awsx", "init", "bash"]);
        assert!(matches!(cli.command, Some(Commands::Init { .. })));

        if let Some(Commands::Init { shell }) = cli.command {
            assert_eq!(shell, "bash");
        }
    }

    #[test]
    fn test_switch_cmd_parsing() {
        let cli = Cli::parse_from(["awsx", "switch", "my-profile"]);
        assert!(matches!(
            cli.command,
            Some(Commands::Switch { profile: Some(_) })
        ));

        if let Some(Commands::Switch { profile }) = cli.command {
            assert_eq!(profile, Some("my-profile".to_string()));
        }
    }

    #[test]
    fn test_list_profiles_parsing() {
        let cli = Cli::parse_from(["awsx", "list-profiles"]);
        assert_eq!(cli.command, Some(Commands::ListProfiles));
    }
}
