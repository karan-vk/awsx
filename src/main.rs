use clap::{Parser, Subcommand};

mod aws_config;
mod cli;
mod shell_hooks;

#[derive(Parser)]
#[command(name = "awsx")]
#[command(about = "Interactive AWS profile switcher", long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[arg(long = "list", help = "List available AWS profiles with metadata")]
    pub list: bool,

    /// Profile name to switch to directly (e.g., `awsx dev`)
    pub profile: Option<String>,

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
    #[command(hide = true)]
    CurrentProfile,
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn perform_switch(profile_name: &str) -> Result<(), String> {
    aws_config::update_default_profile(profile_name)?;
    aws_config::persist_active_profile(profile_name)?;
    Ok(())
}

pub fn run(args: Cli) -> Result<(), String> {
    if args.list && args.command.is_some() {
        return Err("--list cannot be combined with subcommands.".to_string());
    }

    if args.list {
        let profiles = aws_config::get_aws_profile_summaries();
        println!("{}", aws_config::format_profile_summaries(&profiles));
        return Ok(());
    }

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
        Some(Commands::CurrentProfile) => {
            if let Some(profile) = aws_config::get_persisted_profile()? {
                println!("{}", profile);
            }
            Ok(())
        }
        Some(Commands::Switch { profile }) => {
            let profiles = aws_config::get_aws_profiles();

            if let Some(target) = profile {
                if profiles.contains(&target) {
                    perform_switch(&target)?;
                    println!("{}", target);
                    Ok(())
                } else {
                    Err(format!("Error: Profile '{}' not found.", target))
                }
            } else {
                // Interactive mode
                if let Some(selected) = cli::select_profile(profiles) {
                    perform_switch(&selected)?;
                    println!("{}", selected);
                }
                Ok(())
            }
        }
        None => {
            if let Some(target) = args.profile {
                let profiles = aws_config::get_aws_profiles();
                if profiles.contains(&target) {
                    perform_switch(&target)?;
                    eprintln!("Switched to AWS profile: {}", target);
                    Ok(())
                } else {
                    Err(format!("Profile '{}' not found.", target))
                }
            } else {
                // Interactive mode (no shell hook needed)
                let profiles = aws_config::get_aws_profiles();
                if let Some(selected) = cli::select_profile(profiles) {
                    perform_switch(&selected)?;
                    eprintln!("Switched to AWS profile: {}", selected);
                }
                Ok(())
            }
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
        assert!(!cli.list);

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
    fn test_direct_profile_parsing() {
        let cli = Cli::parse_from(["awsx", "dev"]);
        assert_eq!(cli.command, None);
        assert_eq!(cli.profile, Some("dev".to_string()));
    }

    #[test]
    fn test_no_args_parsing() {
        let cli = Cli::parse_from(["awsx"]);
        assert_eq!(cli.command, None);
        assert_eq!(cli.profile, None);
        assert!(!cli.list);
    }

    #[test]
    fn test_list_profiles_parsing() {
        let cli = Cli::parse_from(["awsx", "list-profiles"]);
        assert_eq!(cli.command, Some(Commands::ListProfiles));
        assert!(!cli.list);
    }

    #[test]
    fn test_current_profile_parsing() {
        let cli = Cli::parse_from(["awsx", "current-profile"]);
        assert_eq!(cli.command, Some(Commands::CurrentProfile));
        assert!(!cli.list);
    }

    #[test]
    fn test_list_flag_parsing() {
        let cli = Cli::parse_from(["awsx", "--list"]);
        assert!(cli.list);
        assert_eq!(cli.command, None);
    }

    #[test]
    fn test_list_flag_conflicts_with_subcommands() {
        // With args_conflicts_with_subcommands, clap rejects this at parse time
        let result = Cli::try_parse_from(["awsx", "--list", "init", "bash"]);
        assert!(result.is_err());
    }
}
