use ini::Ini;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn get_credentials_path() -> PathBuf {
    if let Ok(val) = env::var("AWS_SHARED_CREDENTIALS_FILE") {
        PathBuf::from(val)
    } else {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        path.push(".aws");
        path.push("credentials");
        path
    }
}

fn get_config_path() -> PathBuf {
    if let Ok(val) = env::var("AWS_CONFIG_FILE") {
        PathBuf::from(val)
    } else {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        path.push(".aws");
        path.push("config");
        path
    }
}

pub fn get_aws_profiles() -> Vec<String> {
    let mut profiles = HashSet::new();

    // parse credentials
    if let Ok(conf) = Ini::load_from_file(get_credentials_path()) {
        extract_profiles_from_ini(&conf, &mut profiles);
    }

    // parse config
    if let Ok(conf) = Ini::load_from_file(get_config_path()) {
        extract_profiles_from_ini(&conf, &mut profiles);
    }

    let mut profile_list: Vec<String> = profiles.into_iter().collect();
    profile_list.sort();

    // Ensure "default" is always at the top if it exists, otherwise sort alphabetically
    if let Some(pos) = profile_list.iter().position(|x| x == "default") {
        profile_list.remove(pos);
        profile_list.insert(0, "default".to_string());
    }

    profile_list
}

fn extract_profiles_from_ini(conf: &Ini, profiles: &mut HashSet<String>) {
    for (sec, _) in conf.iter() {
        if let Some(s) = sec {
            let profile_name = s.strip_prefix("profile ").unwrap_or(s);
            profiles.insert(profile_name.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_profiles() {
        let creds_str = r#"
[default]
aws_access_key_id=FOO
[dev]
aws_access_key_id=BAR
"#;
        let config_str = r#"
[default]
region=us-east-1
[profile dev]
region=us-west-2
[profile staging]
region=eu-central-1
"#;
        let mut profiles = HashSet::new();

        let creds_ini = Ini::load_from_str(creds_str).unwrap();
        extract_profiles_from_ini(&creds_ini, &mut profiles);

        let config_ini = Ini::load_from_str(config_str).unwrap();
        extract_profiles_from_ini(&config_ini, &mut profiles);

        let mut actual: Vec<String> = profiles.into_iter().collect();
        actual.sort();

        assert_eq!(actual, vec!["default", "dev", "staging"]);
    }
}
