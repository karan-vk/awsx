use inquire::Select;

pub fn select_profile(profiles: Vec<String>) -> Option<String> {
    if profiles.is_empty() {
        eprintln!("No AWS profiles found in ~/.aws/config or ~/.aws/credentials");
        return None;
    }

    Select::new("Select AWS Profile:", profiles).prompt().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_profile_empty() {
        let result = select_profile(vec![]);
        assert!(result.is_none());
    }
}
