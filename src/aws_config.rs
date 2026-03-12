use dirs::home_dir;
use ini::Ini;
use std::collections::HashSet;

pub fn get_aws_profiles() -> Vec<String> {
    let mut profiles = HashSet::new();

    // 1. Try to read ~/.aws/config
    if let Some(mut config_path) = home_dir() {
        config_path.push(".aws");
        config_path.push("config");
        if let Ok(conf) = Ini::load_from_file(config_path) {
            for s in conf.sections().flatten() {
                // Profiles in config are often prefixed with "profile "
                let name = s.strip_prefix("profile ").unwrap_or(s);
                profiles.insert(name.to_string());
            }
        }
    }

    // 2. Try to read ~/.aws/credentials
    if let Some(mut creds_path) = home_dir() {
        creds_path.push(".aws");
        creds_path.push("credentials");
        if let Ok(conf) = Ini::load_from_file(creds_path) {
            for s in conf.sections().flatten() {
                profiles.insert(s.to_string());
            }
        }
    }

    // Always include 'default' if it exists or even if it doesn't, it might be common
    // but we filter empty strings and duplicates
    let mut result: Vec<String> = profiles
        .into_iter()
        .filter(|p| !p.trim().is_empty() && p != "default")
        .collect();
    result.sort();

    // Put 'default' at the beginning
    result.insert(0, "default".to_string());
    result
}

#[cfg(test)]
pub fn parse_ini_content(content: &str) -> Vec<String> {
    let mut profiles = HashSet::new();
    if let Ok(conf) = Ini::load_from_str(content) {
        for s in conf.sections().flatten() {
            let name = s.strip_prefix("profile ").unwrap_or(s);
            if !name.trim().is_empty() {
                profiles.insert(name.to_string());
            }
        }
    }
    let mut v: Vec<String> = profiles.into_iter().collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ini_content_basic() {
        let content = r#"
[default]
aws_access_key_id=123
[profile dev]
region=us-east-1
"#;
        let profiles = parse_ini_content(content);
        assert!(profiles.contains(&"default".to_string()));
        assert!(profiles.contains(&"dev".to_string()));
    }

    #[test]
    fn test_parse_ini_content_no_prefix() {
        let content = r#"
[staging]
region=us-west-2
"#;
        let profiles = parse_ini_content(content);
        assert!(profiles.contains(&"staging".to_string()));
    }

    #[test]
    fn test_parse_ini_content_empty() {
        let profiles = parse_ini_content("");
        assert!(profiles.is_empty());
    }

    #[test]
    fn test_parse_ini_content_malformed() {
        let profiles = parse_ini_content("not ini format");
        assert!(profiles.is_empty());
    }
}
