pub fn generate_hook(shell: &str) {
    match shell {
        "bash" | "zsh" => {
            let script = r#"
awsp() {
    if [ "$1" = "init" ]; then
        command awsp init "$2"
        return
    fi
    if [ "$1" = "--select" ]; then
        command awsp --select
        return
    fi

    local profile="$(command awsp --select)"
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
function awsp
    if test "$argv[1]" = "init"
        command awsp init $argv[2]
        return
    end
    if test "$argv[1]" = "--select"
        command awsp --select
        return
    end

    set -l profile (command awsp --select)
    if test -n "$profile"
        set -gx AWS_PROFILE "$profile"
        echo "Switched to AWS profile: $AWS_PROFILE"
    end
end
"#;
            println!("{}", script.trim());
        }
        _ => {
            eprintln!("Unsupported shell: {}. Supported shells are bash, zsh, fish.", shell);
            std::process::exit(1);
        }
    }
}
