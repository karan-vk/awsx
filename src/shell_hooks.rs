pub fn generate_hook(shell: &str) {
    println!("{}", get_hook_script(shell));
}

pub fn get_hook_script(shell: &str) -> String {
    match shell {
        "bash" => {
            let script = r#"
awsx() {
    if [ "$#" -eq 0 ]; then
        local profile="$(command awsx switch)"
        if [ -n "$profile" ]; then
            export AWS_PROFILE="$profile"
            echo "Switched to AWS profile: $AWS_PROFILE"
        fi
        return
    fi
    
    if [ "$1" = "init" ] || [ "$1" = "list-profiles" ] || [ "$1" = "-h" ] || [ "$1" = "--help" ] || [ "$1" = "-V" ] || [ "$1" = "--version" ]; then
        command awsx "$@"
        return
    fi
    
    local profile="$(command awsx switch "$1")"
    if [ -n "$profile" ]; then
        export AWS_PROFILE="$profile"
        echo "Switched to AWS profile: $AWS_PROFILE"
    fi
}
_awsx_completions() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local profiles="$(command awsx list-profiles)"
    COMPREPLY=( $(compgen -W "${profiles}" -- "${cur}") )
}
complete -F _awsx_completions awsx
"#;
            script.trim().to_string()
        }
        "zsh" => {
            let script = r#"
awsx() {
    if [ "$#" -eq 0 ]; then
        local profile="$(command awsx switch)"
        if [ -n "$profile" ]; then
            export AWS_PROFILE="$profile"
            echo "Switched to AWS profile: $AWS_PROFILE"
        fi
        return
    fi
    
    if [[ "$1" == "init" || "$1" == "list-profiles" || "$1" == "-h" || "$1" == "--help" || "$1" == "-V" || "$1" == "--version" ]]; then
        command awsx "$@"
        return
    fi
    
    local profile="$(command awsx switch "$1")"
    if [ -n "$profile" ]; then
        export AWS_PROFILE="$profile"
        echo "Switched to AWS profile: $AWS_PROFILE"
    fi
}
_awsx_completions() {
    local -a profiles
    profiles=("${(@f)$(command awsx list-profiles)}")
    compadd -a profiles
}
compdef _awsx_completions awsx
"#;
            script.trim().to_string()
        }
        "fish" => {
            let script = r#"
function awsx
    if test (count $argv) -eq 0
        set -l profile (command awsx switch)
        if test -n "$profile"
            set -gx AWS_PROFILE "$profile"
            echo "Switched to AWS profile: $AWS_PROFILE"
        end
        return
    end

    if contains -- $argv[1] "init" "list-profiles" "-h" "--help" "-V" "--version"
        command awsx $argv
        return
    end

    set -l profile (command awsx switch $argv[1])
    if test -n "$profile"
        set -gx AWS_PROFILE "$profile"
        echo "Switched to AWS profile: $AWS_PROFILE"
    end
end
complete -c awsx -f -a "(command awsx list-profiles)"
"#;
            script.trim().to_string()
        }
        "powershell" => {
            let script = r#"
function awsx {
    if ($args.Count -eq 0) {
        $profile = (command awsx switch)
        if ($profile) {
            $env:AWS_PROFILE = $profile
            Write-Host "Switched to AWS profile: $profile"
        }
        return
    }

    $cmd = $args[0]
    if ($cmd -eq "init" -or $cmd -eq "list-profiles" -or $cmd -eq "-h" -or $cmd -eq "--help" -or $cmd -eq "-V" -or $cmd -eq "--version") {
        command awsx $args
        return
    }

    $profile = (command awsx switch $cmd)
    if ($profile) {
        $env:AWS_PROFILE = $profile
        Write-Host "Switched to AWS profile: $profile"
    }
}
Register-ArgumentCompleter -Native -CommandName awsx -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $profiles = awsx list-profiles
    $profiles | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_)
    }
}
"#;
            script.trim().to_string()
        }
        _ => {
            eprintln!(
                "Unsupported shell: {}. Supported shells are bash, zsh, fish, powershell.",
                shell
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bash_hook() {
        let hook = get_hook_script("bash");
        assert!(hook.contains("awsx()"));
        assert!(hook.contains("complete -F _awsx_completions awsx"));
    }

    #[test]
    fn test_zsh_hook() {
        let hook = get_hook_script("zsh");
        assert!(hook.contains("awsx()"));
        assert!(hook.contains("compdef _awsx_completions awsx"));
    }

    #[test]
    fn test_fish_hook() {
        let hook = get_hook_script("fish");
        assert!(hook.contains("function awsx"));
        assert!(hook.contains("complete -c awsx"));
    }

    #[test]
    fn test_powershell_hook() {
        let hook = get_hook_script("powershell");
        assert!(hook.contains("function awsx {"));
        assert!(hook.contains("Register-ArgumentCompleter"));
    }
}
