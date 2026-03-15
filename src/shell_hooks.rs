pub fn generate_hook(shell: &str) {
    println!("{}", get_hook_script(shell));
}

pub fn get_hook_script(shell: &str) -> String {
    match shell {
        "bash" => {
            let script = r#"
_awsx_apply_profile() {
    local profile="$1"
    unset AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_SESSION_TOKEN AWS_SECURITY_TOKEN
    unset AWS_ACCESS_KEY AWS_SECRET_KEY AWS_DELEGATION_TOKEN AWS_CREDENTIAL_EXPIRATION
    export AWS_PROFILE="$profile"
    export AWS_DEFAULT_PROFILE="$profile"
    export AWS_SDK_LOAD_CONFIG=1
}
_awsx_restore_profile() {
    if [ -n "${AWS_PROFILE:-}" ] || [ -n "${AWS_DEFAULT_PROFILE:-}" ]; then
        return
    fi

    local profile="$(command awsx current-profile 2>/dev/null)"
    if [ -n "$profile" ]; then
        _awsx_apply_profile "$profile"
    fi
}
_awsx_restore_profile
awsx() {
    if [ "$#" -eq 0 ]; then
        local profile="$(command awsx switch)"
        if [ -n "$profile" ]; then
            _awsx_apply_profile "$profile"
            echo "Switched to AWS profile: $AWS_PROFILE"
        fi
        return
    fi
    
    if [ "$1" = "init" ] || [ "$1" = "list-profiles" ] || [ "$1" = "current-profile" ] || [ "$1" = "--list" ] || [ "$1" = "-h" ] || [ "$1" = "--help" ] || [ "$1" = "-V" ] || [ "$1" = "--version" ]; then
        command awsx "$@"
        return
    fi

    if [ "$#" -ne 1 ]; then
        command awsx "$@"
        return
    fi
    
    local profile="$(command awsx switch "$1")"
    if [ -n "$profile" ]; then
        _awsx_apply_profile "$profile"
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
_awsx_apply_profile() {
    local profile="$1"
    unset AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_SESSION_TOKEN AWS_SECURITY_TOKEN
    unset AWS_ACCESS_KEY AWS_SECRET_KEY AWS_DELEGATION_TOKEN AWS_CREDENTIAL_EXPIRATION
    export AWS_PROFILE="$profile"
    export AWS_DEFAULT_PROFILE="$profile"
    export AWS_SDK_LOAD_CONFIG=1
}
_awsx_restore_profile() {
    if [ -n "${AWS_PROFILE:-}" ] || [ -n "${AWS_DEFAULT_PROFILE:-}" ]; then
        return
    fi

    local profile="$(command awsx current-profile 2>/dev/null)"
    if [ -n "$profile" ]; then
        _awsx_apply_profile "$profile"
    fi
}
_awsx_restore_profile
awsx() {
    if [ "$#" -eq 0 ]; then
        local profile="$(command awsx switch)"
        if [ -n "$profile" ]; then
            _awsx_apply_profile "$profile"
            echo "Switched to AWS profile: $AWS_PROFILE"
        fi
        return
    fi
    
    if [[ "$1" == "init" || "$1" == "list-profiles" || "$1" == "current-profile" || "$1" == "--list" || "$1" == "-h" || "$1" == "--help" || "$1" == "-V" || "$1" == "--version" ]]; then
        command awsx "$@"
        return
    fi

    if [ "$#" -ne 1 ]; then
        command awsx "$@"
        return
    fi
    
    local profile="$(command awsx switch "$1")"
    if [ -n "$profile" ]; then
        _awsx_apply_profile "$profile"
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
function __awsx_apply_profile --argument profile
    set -e AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY AWS_SESSION_TOKEN AWS_SECURITY_TOKEN
    set -e AWS_ACCESS_KEY AWS_SECRET_KEY AWS_DELEGATION_TOKEN AWS_CREDENTIAL_EXPIRATION
    set -gx AWS_PROFILE "$profile"
    set -gx AWS_DEFAULT_PROFILE "$profile"
    set -gx AWS_SDK_LOAD_CONFIG 1
end
function __awsx_restore_profile
    if set -q AWS_PROFILE
        return
    end
    if set -q AWS_DEFAULT_PROFILE
        return
    end

    set -l profile (command awsx current-profile 2>/dev/null)
    if test -n "$profile"
        __awsx_apply_profile "$profile"
    end
end
__awsx_restore_profile
function awsx
    if test (count $argv) -eq 0
        set -l profile (command awsx switch)
        if test -n "$profile"
            __awsx_apply_profile "$profile"
            echo "Switched to AWS profile: $AWS_PROFILE"
        end
        return
    end

    if contains -- $argv[1] "init" "list-profiles" "current-profile" "--list" "-h" "--help" "-V" "--version"
        command awsx $argv
        return
    end

    if test (count $argv) -ne 1
        command awsx $argv
        return
    end

    set -l profile (command awsx switch $argv[1])
    if test -n "$profile"
        __awsx_apply_profile "$profile"
        echo "Switched to AWS profile: $AWS_PROFILE"
    end
end
complete -c awsx -f -a "(command awsx list-profiles)"
"#;
            script.trim().to_string()
        }
        "powershell" => {
            let script = r#"
function Invoke-AwsxBinary {
    param([Parameter(ValueFromRemainingArguments = $true)][string[]]$AwsxArgs)
    $awsxBinary = (Get-Command awsx -CommandType Application | Select-Object -First 1 -ExpandProperty Source)
    & $awsxBinary @AwsxArgs
}
function Set-AwsxProfile($profile) {
    Remove-Item Env:AWS_ACCESS_KEY_ID -ErrorAction SilentlyContinue
    Remove-Item Env:AWS_SECRET_ACCESS_KEY -ErrorAction SilentlyContinue
    Remove-Item Env:AWS_SESSION_TOKEN -ErrorAction SilentlyContinue
    Remove-Item Env:AWS_SECURITY_TOKEN -ErrorAction SilentlyContinue
    Remove-Item Env:AWS_ACCESS_KEY -ErrorAction SilentlyContinue
    Remove-Item Env:AWS_SECRET_KEY -ErrorAction SilentlyContinue
    Remove-Item Env:AWS_DELEGATION_TOKEN -ErrorAction SilentlyContinue
    Remove-Item Env:AWS_CREDENTIAL_EXPIRATION -ErrorAction SilentlyContinue
    $env:AWS_PROFILE = $profile
    $env:AWS_DEFAULT_PROFILE = $profile
    $env:AWS_SDK_LOAD_CONFIG = "1"
}
if (-not $env:AWS_PROFILE -and -not $env:AWS_DEFAULT_PROFILE) {
    $persistedProfile = (Invoke-AwsxBinary current-profile 2>$null)
    if ($persistedProfile) {
        Set-AwsxProfile $persistedProfile
    }
}
function awsx {
    if ($args.Count -eq 0) {
        $profile = (Invoke-AwsxBinary switch)
        if ($profile) {
            Set-AwsxProfile $profile
            Write-Host "Switched to AWS profile: $profile"
        }
        return
    }

    $cmd = $args[0]
    if ($cmd -eq "init" -or $cmd -eq "list-profiles" -or $cmd -eq "current-profile" -or $cmd -eq "--list" -or $cmd -eq "-h" -or $cmd -eq "--help" -or $cmd -eq "-V" -or $cmd -eq "--version") {
        Invoke-AwsxBinary @args
        return
    }

    if ($args.Count -ne 1) {
        Invoke-AwsxBinary @args
        return
    }

    $profile = (Invoke-AwsxBinary switch $cmd)
    if ($profile) {
        Set-AwsxProfile $profile
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

    fn assert_common_profile_activation(hook: &str) {
        assert!(hook.contains("AWS_PROFILE"));
        assert!(hook.contains("AWS_DEFAULT_PROFILE"));
        assert!(hook.contains("AWS_SDK_LOAD_CONFIG"));
        assert!(hook.contains("AWS_ACCESS_KEY_ID"));
        assert!(hook.contains("AWS_SECRET_ACCESS_KEY"));
        assert!(hook.contains("--list"));
        assert!(hook.contains("current-profile"));
        assert!(hook.contains("-ne 1") || hook.contains("Count -ne 1"));
    }

    #[test]
    fn test_bash_hook() {
        let hook = get_hook_script("bash");
        assert!(hook.contains("awsx()"));
        assert!(hook.contains("complete -F _awsx_completions awsx"));
        assert_common_profile_activation(&hook);
    }

    #[test]
    fn test_zsh_hook() {
        let hook = get_hook_script("zsh");
        assert!(hook.contains("awsx()"));
        assert!(hook.contains("compdef _awsx_completions awsx"));
        assert_common_profile_activation(&hook);
    }

    #[test]
    fn test_fish_hook() {
        let hook = get_hook_script("fish");
        assert!(hook.contains("function awsx"));
        assert!(hook.contains("complete -c awsx"));
        assert_common_profile_activation(&hook);
    }

    #[test]
    fn test_powershell_hook() {
        let hook = get_hook_script("powershell");
        assert!(hook.contains("function awsx {"));
        assert!(hook.contains("Register-ArgumentCompleter"));
        assert_common_profile_activation(&hook);
    }
}
