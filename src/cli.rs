use inquire::Select;

pub fn select_profile(profiles: Vec<String>) -> Option<String> {
    if profiles.is_empty() {
        eprintln!("No AWS profiles found in ~/.aws/config or ~/.aws/credentials");
        return None;
    }

    let default_profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| String::new());

    // We can pre-select the current profile if it's in the list
    let starting_cursor = profiles
        .iter()
        .position(|p| p == &default_profile)
        .unwrap_or_default();

    let ans = Select::new("Select AWS Profile:", profiles)
        .with_starting_cursor(starting_cursor)
        .prompt();

    ans.ok()
}
