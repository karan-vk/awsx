pub fn generate_hook(shell: &str) {
    match shell {
        "bash" | "zsh" => {
            let script = r#"
awsx() {
    if [ "$1" = "init" ]; then
        command awsx init "$2"
        return
    fi
    if [ "$1" = "--select" ]; then
        command awsx --select
        return
    fi

    local profile="$(command awsx --select)"
    if [ -n "$profile" ]; then
        export AWS_PROFILE="$profile"
        echo "Switched to AWS profile: $AWS_PROFILE"
    fi
}
"#;
            println!("{}", script.trim());
        }
        "fish" => {
            let script = r#"
function awsx
    if test "$argv[1]" = "init"
        command awsx init $argv[2]
        return
    end
    if test "$argv[1]" = "--select"
        command awsx --select
        return
    end

    set -l profile (command awsx --select)
    if test -n "$profile"
        set -gx AWS_PROFILE "$profile"
        echo "Switched to AWS profile: $AWS_PROFILE"
    end
end
"#;
            println!("{}", script.trim());
        }
        "powershell" => {
            let script = r#"
function awsx {
    if ($args[0] -eq "init") {
        command awsx init $args[1]
        return
    }
    if ($args[0] -eq "--select") {
        command awsx --select
        return
    }

    $profile = (command awsx --select)
    if ($profile) {
        $env:AWS_PROFILE = $profile
        Write-Host "Switched to AWS profile: $profile"
    }
}
"#;
            println!("{}", script.trim());
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
