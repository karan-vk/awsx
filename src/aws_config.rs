use dirs::home_dir;
use ini::Ini;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

const AWSX_SELECTED_PROFILE_KEY: &str = "awsx_selected_profile";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsProfileSummary {
    pub name: String,
    pub account_id: Option<String>,
    pub auth_type: String,
    pub region: Option<String>,
    pub details: Vec<String>,
}

#[derive(Debug, Default, Clone)]
struct ProfileRecord {
    values: HashMap<String, String>,
    has_static_credentials: bool,
}

pub fn get_aws_profiles() -> Vec<String> {
    get_aws_profile_summaries()
        .into_iter()
        .map(|profile| profile.name)
        .collect()
}

pub fn get_aws_profile_summaries() -> Vec<AwsProfileSummary> {
    build_profile_summaries(load_profile_records_from_disk())
}

pub fn format_profile_summaries(summaries: &[AwsProfileSummary]) -> String {
    if summaries.is_empty() {
        return "No AWS profiles found in ~/.aws/config or ~/.aws/credentials".to_string();
    }

    let headers = ["PROFILE", "ACCOUNT", "TYPE", "REGION", "DETAILS"];
    let rows: Vec<[String; 5]> = summaries
        .iter()
        .map(|summary| {
            [
                summary.name.clone(),
                summary
                    .account_id
                    .clone()
                    .unwrap_or_else(|| "-".to_string()),
                summary.auth_type.clone(),
                summary.region.clone().unwrap_or_else(|| "-".to_string()),
                if summary.details.is_empty() {
                    "-".to_string()
                } else {
                    summary.details.join(", ")
                },
            ]
        })
        .collect();

    let mut widths = [
        headers[0].len(),
        headers[1].len(),
        headers[2].len(),
        headers[3].len(),
    ];
    for row in &rows {
        for (idx, width) in widths.iter_mut().enumerate() {
            *width = (*width).max(row[idx].len());
        }
    }

    let mut lines = vec![format_profile_row(&headers, &widths)];
    lines.extend(rows.iter().map(|row| format_profile_row(row, &widths)));
    lines.join("\n")
}

pub fn update_default_profile(profile_name: &str) -> Result<(), String> {
    let config_path = aws_shared_file_path("config")
        .ok_or_else(|| "Could not determine home directory.".to_string())?;
    let credentials_path = aws_shared_file_path("credentials")
        .ok_or_else(|| "Could not determine home directory.".to_string())?;

    update_default_profile_at_paths(&config_path, &credentials_path, profile_name)
}

fn update_default_profile_at_paths(
    config_path: &Path,
    credentials_path: &Path,
    profile_name: &str,
) -> Result<(), String> {
    if profile_name == "default" {
        return Ok(());
    }

    let mut config_updated = false;
    let mut creds_updated = false;

    // 1. Update ~/.aws/config
    if let Ok(mut conf) = Ini::load_from_file(config_path) {
        let section_name = format!("profile {}", profile_name);

        if let Some(section) = conf.section(Some(&section_name)) {
            let source_data: HashMap<String, String> = section
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            // Preserve awsx marker before clearing
            let marker = conf
                .section(Some("default"))
                .and_then(|s| s.get(AWSX_SELECTED_PROFILE_KEY))
                .map(|v| v.to_string());

            // Clear entire [default] section to remove stale keys
            conf.delete(Some("default"));

            // Copy all keys from target profile
            for (key, value) in &source_data {
                conf.with_section(Some("default")).set(key, value);
            }

            // Restore awsx marker
            if let Some(marker_value) = marker {
                conf.with_section(Some("default"))
                    .set(AWSX_SELECTED_PROFILE_KEY, marker_value);
            }

            if conf.write_to_file(config_path).is_ok() {
                config_updated = true;
            }
        }
    }

    // 2. Update ~/.aws/credentials
    if let Ok(mut conf) = Ini::load_from_file(credentials_path)
        && let Some(section) = conf.section(Some(profile_name))
    {
        let source_data: HashMap<String, String> = section
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        // Preserve awsx marker before clearing
        let marker = conf
            .section(Some("default"))
            .and_then(|s| s.get(AWSX_SELECTED_PROFILE_KEY))
            .map(|v| v.to_string());

        // Clear entire [default] section to remove stale keys
        conf.delete(Some("default"));

        // Copy all keys from target profile
        for (key, value) in &source_data {
            conf.with_section(Some("default")).set(key, value);
        }

        // Restore awsx marker
        if let Some(marker_value) = marker {
            conf.with_section(Some("default"))
                .set(AWSX_SELECTED_PROFILE_KEY, marker_value);
        }

        if conf.write_to_file(credentials_path).is_ok() {
            creds_updated = true;
        }
    }

    if config_updated || creds_updated {
        Ok(())
    } else {
        Err(format!(
            "Could not find profile '{}' in configuration files to persist.",
            profile_name
        ))
    }
}

pub fn persist_active_profile(profile_name: &str) -> Result<(), String> {
    let config_path = aws_shared_file_path("config")
        .ok_or_else(|| "Could not determine home directory.".to_string())?;
    let credentials_path = aws_shared_file_path("credentials")
        .ok_or_else(|| "Could not determine home directory.".to_string())?;

    persist_active_profile_to_paths(&config_path, &credentials_path, profile_name)
}

pub fn get_persisted_profile() -> Result<Option<String>, String> {
    let Some(config_path) = aws_shared_file_path("config") else {
        return Ok(None);
    };

    let Some(credentials_path) = aws_shared_file_path("credentials") else {
        return Ok(None);
    };

    read_persisted_profile_from_paths(&config_path, &credentials_path, &get_aws_profiles())
}

fn persist_active_profile_to_paths(
    config_path: &Path,
    credentials_path: &Path,
    profile_name: &str,
) -> Result<(), String> {
    let mut config = load_ini_or_create(config_path)?;
    let mut credentials = load_ini_or_create(credentials_path)?;

    set_selected_profile(&mut config, profile_name);
    set_selected_profile(&mut credentials, profile_name);

    write_ini_to_path(config_path, &config, "config")?;
    write_ini_to_path(credentials_path, &credentials, "credentials")
}

fn read_persisted_profile_from_paths(
    config_path: &Path,
    credentials_path: &Path,
    available_profiles: &[String],
) -> Result<Option<String>, String> {
    let credentials_profile =
        load_ini_if_exists(credentials_path)?.and_then(|ini| get_selected_profile(&ini));
    let config_profile =
        load_ini_if_exists(config_path)?.and_then(|ini| get_selected_profile(&ini));
    let candidate = match (credentials_profile, config_profile) {
        (Some(credentials_profile), Some(config_profile))
            if credentials_profile != config_profile =>
        {
            None
        }
        (Some(credentials_profile), Some(_)) => Some(credentials_profile),
        (Some(credentials_profile), None) => Some(credentials_profile),
        (None, Some(config_profile)) => Some(config_profile),
        (None, None) => None,
    };

    Ok(candidate.filter(|profile| available_profiles.contains(profile)))
}

fn load_ini_or_create(path: &Path) -> Result<Ini, String> {
    match Ini::load_from_file(path) {
        Ok(ini) => Ok(ini),
        Err(ini::Error::Io(err)) if err.kind() == ErrorKind::NotFound => Ok(Ini::new()),
        Err(err) => Err(format!("Could not read {}: {err}", path.display())),
    }
}

fn load_ini_if_exists(path: &Path) -> Result<Option<Ini>, String> {
    match Ini::load_from_file(path) {
        Ok(ini) => Ok(Some(ini)),
        Err(ini::Error::Io(err)) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(format!("Could not read {}: {err}", path.display())),
    }
}

fn write_ini_to_path(path: &Path, ini: &Ini, file_label: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Could not create AWS directory for {file_label}: {err}"))?;
    }

    ini.write_to_file(path)
        .map_err(|err| format!("Could not write {file_label}: {err}"))
}

fn set_selected_profile(ini: &mut Ini, profile_name: &str) {
    ini.with_section(Some("default"))
        .set(AWSX_SELECTED_PROFILE_KEY, profile_name);
}

fn get_selected_profile(ini: &Ini) -> Option<String> {
    ini.section(Some("default"))
        .and_then(|section| section.get(AWSX_SELECTED_PROFILE_KEY))
        .and_then(parse_persisted_profile)
}

fn format_profile_row(columns: &[impl AsRef<str>], widths: &[usize; 4]) -> String {
    format!(
        "{:<w0$}  {:<w1$}  {:<w2$}  {:<w3$}  {}",
        columns[0].as_ref(),
        columns[1].as_ref(),
        columns[2].as_ref(),
        columns[3].as_ref(),
        columns[4].as_ref(),
        w0 = widths[0],
        w1 = widths[1],
        w2 = widths[2],
        w3 = widths[3],
    )
}

fn load_profile_records_from_disk() -> BTreeMap<String, ProfileRecord> {
    let config = load_ini_from_home("config");
    let credentials = load_ini_from_home("credentials");
    collect_profile_records(config.as_ref(), credentials.as_ref())
}

fn aws_shared_file_path(file_name: &str) -> Option<std::path::PathBuf> {
    let mut path = home_dir()?;
    path.push(".aws");
    path.push(file_name);
    Some(path)
}

fn parse_persisted_profile(contents: &str) -> Option<String> {
    let profile = contents.trim();
    if profile.is_empty() {
        None
    } else {
        Some(profile.to_string())
    }
}

fn load_ini_from_home(file_name: &str) -> Option<Ini> {
    let mut path = home_dir()?;
    path.push(".aws");
    path.push(file_name);
    Ini::load_from_file(path).ok()
}

fn collect_profile_records(
    config: Option<&Ini>,
    credentials: Option<&Ini>,
) -> BTreeMap<String, ProfileRecord> {
    let mut profiles = BTreeMap::new();

    if let Some(config) = config {
        merge_ini_sections(config, true, &mut profiles);
    }

    if let Some(credentials) = credentials {
        merge_ini_sections(credentials, false, &mut profiles);
    }

    profiles
}

fn merge_ini_sections(
    conf: &Ini,
    strip_profile_prefix: bool,
    profiles: &mut BTreeMap<String, ProfileRecord>,
) {
    for section_name in conf.sections().flatten() {
        let Some(profile_name) = normalize_profile_name(section_name, strip_profile_prefix) else {
            continue;
        };

        if profile_name.trim().is_empty() {
            continue;
        }

        let Some(section) = conf.section(Some(section_name)) else {
            continue;
        };

        let record = profiles.entry(profile_name.to_string()).or_default();

        for (key, value) in section.iter() {
            if matches!(
                key,
                "aws_access_key_id" | "aws_secret_access_key" | "aws_session_token"
            ) {
                record.has_static_credentials = true;
            }

            record.values.insert(key.to_string(), value.to_string());
        }
    }
}

fn normalize_profile_name(section_name: &str, strip_profile_prefix: bool) -> Option<&str> {
    if strip_profile_prefix {
        if section_name == "default" {
            Some(section_name)
        } else {
            section_name.strip_prefix("profile ")
        }
    } else {
        Some(section_name)
    }
}

fn build_profile_summaries(records: BTreeMap<String, ProfileRecord>) -> Vec<AwsProfileSummary> {
    let mut summaries: Vec<_> = records
        .keys()
        .map(|profile_name| AwsProfileSummary {
            name: profile_name.clone(),
            account_id: resolve_account_id(profile_name, &records, &mut HashSet::new()),
            auth_type: auth_type_for(records.get(profile_name).expect("profile record exists"))
                .to_string(),
            region: records
                .get(profile_name)
                .and_then(|profile| profile.values.get("region").cloned()),
            details: details_for(records.get(profile_name).expect("profile record exists")),
        })
        .collect();

    summaries.sort_by(
        |left, right| match (left.name.as_str(), right.name.as_str()) {
            ("default", "default") => std::cmp::Ordering::Equal,
            ("default", _) => std::cmp::Ordering::Less,
            (_, "default") => std::cmp::Ordering::Greater,
            _ => left.name.cmp(&right.name),
        },
    );

    summaries
}

fn resolve_account_id(
    profile_name: &str,
    records: &BTreeMap<String, ProfileRecord>,
    visited: &mut HashSet<String>,
) -> Option<String> {
    if !visited.insert(profile_name.to_string()) {
        return None;
    }

    let profile = records.get(profile_name)?;

    explicit_account_id(profile).or_else(|| {
        profile
            .values
            .get("source_profile")
            .and_then(|source_profile| resolve_account_id(source_profile, records, visited))
    })
}

fn explicit_account_id(profile: &ProfileRecord) -> Option<String> {
    profile
        .values
        .get("sso_account_id")
        .or_else(|| profile.values.get("aws_account_id"))
        .filter(|value| is_account_id(value))
        .cloned()
        .or_else(|| {
            profile
                .values
                .get("role_arn")
                .and_then(|arn| extract_account_id_from_arn(arn))
        })
        .or_else(|| {
            profile
                .values
                .get("mfa_serial")
                .and_then(|arn| extract_account_id_from_arn(arn))
        })
}

fn extract_account_id_from_arn(arn: &str) -> Option<String> {
    let account_id = arn.split(':').nth(4)?;
    if is_account_id(account_id) {
        Some(account_id.to_string())
    } else {
        None
    }
}

fn is_account_id(value: &str) -> bool {
    value.len() == 12 && value.chars().all(|ch| ch.is_ascii_digit())
}

fn auth_type_for(profile: &ProfileRecord) -> &'static str {
    if profile.values.contains_key("sso_account_id")
        || profile.values.contains_key("sso_role_name")
        || profile.values.contains_key("sso_start_url")
        || profile.values.contains_key("sso_session")
    {
        "sso"
    } else if profile.values.contains_key("role_arn") {
        "role"
    } else if profile.values.contains_key("credential_process") {
        "process"
    } else if profile.has_static_credentials {
        "static"
    } else if profile.values.contains_key("source_profile") {
        "source"
    } else {
        "config"
    }
}

fn details_for(profile: &ProfileRecord) -> Vec<String> {
    let mut details = Vec::new();

    if let Some(role_arn) = profile.values.get("role_arn") {
        let role_label = role_arn
            .split(':')
            .nth(5)
            .and_then(|resource| resource.strip_prefix("role/"))
            .unwrap_or(role_arn);
        details.push(format!("role={role_label}"));
    }

    if let Some(source_profile) = profile.values.get("source_profile") {
        details.push(format!("source={source_profile}"));
    }

    if let Some(sso_role_name) = profile.values.get("sso_role_name") {
        details.push(format!("sso_role={sso_role_name}"));
    }

    if let Some(output) = profile.values.get("output") {
        details.push(format!("output={output}"));
    }

    if profile.values.contains_key("credential_process") {
        details.push("credential_process".to_string());
    }

    if profile.values.contains_key("mfa_serial") {
        details.push("mfa".to_string());
    }

    if profile.has_static_credentials {
        details.push("static_creds".to_string());
    }

    details
}

#[cfg(test)]
pub fn parse_ini_content(content: &str) -> Vec<String> {
    if let Ok(conf) = Ini::load_from_str(content) {
        return collect_profile_records(Some(&conf), None)
            .into_keys()
            .collect();
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_profile_summaries(config: &str, credentials: &str) -> Vec<AwsProfileSummary> {
        let config_ini = Ini::load_from_str(config).ok();
        let credentials_ini = Ini::load_from_str(credentials).ok();
        build_profile_summaries(collect_profile_records(
            config_ini.as_ref(),
            credentials_ini.as_ref(),
        ))
    }

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
        let credentials = r#"
[staging]
region=us-west-2
"#;
        let summaries = parse_profile_summaries("", credentials);
        assert_eq!(summaries[0].name, "staging");
    }

    #[test]
    fn test_parse_ini_content_ignores_non_profile_config_sections() {
        let content = r#"
[sso-session corp]
sso_start_url=https://example.awsapps.com/start

[profile dev]
region=us-west-2
"#;

        let profiles = parse_ini_content(content);
        assert_eq!(profiles, vec!["dev".to_string()]);
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

    #[test]
    fn test_parse_persisted_profile_trims_value() {
        assert_eq!(parse_persisted_profile("meta\n"), Some("meta".to_string()));
        assert_eq!(parse_persisted_profile("   \n"), None);
    }

    #[test]
    fn test_persist_and_read_active_profile_from_path() {
        let path = std::env::temp_dir().join(format!(
            "awsx-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let config_file = path.join(".aws").join("config");
        let credentials_file = path.join(".aws").join("credentials");
        let available_profiles = vec!["meta".to_string(), "dev".to_string()];

        persist_active_profile_to_paths(&config_file, &credentials_file, "meta")
            .expect("persist should succeed");
        let persisted =
            read_persisted_profile_from_paths(&config_file, &credentials_file, &available_profiles)
                .expect("read should succeed")
                .expect("profile should exist");

        assert_eq!(persisted, "meta");

        let credentials = Ini::load_from_file(&credentials_file).expect("credentials should exist");
        assert_eq!(get_selected_profile(&credentials).as_deref(), Some("meta"));

        fs::remove_dir_all(path).expect("cleanup should succeed");
    }

    #[test]
    fn test_persisted_profile_ignores_missing_profile() {
        let path = std::env::temp_dir().join(format!(
            "awsx-test-missing-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let config_file = path.join(".aws").join("config");
        let credentials_file = path.join(".aws").join("credentials");
        let available_profiles = vec!["dev".to_string()];

        persist_active_profile_to_paths(&config_file, &credentials_file, "meta")
            .expect("persist should succeed");
        let persisted =
            read_persisted_profile_from_paths(&config_file, &credentials_file, &available_profiles)
                .expect("read should succeed");

        assert_eq!(persisted, None);

        fs::remove_dir_all(path).expect("cleanup should succeed");
    }

    #[test]
    fn test_persisted_profile_ignores_diverged_markers() {
        let path = std::env::temp_dir().join(format!(
            "awsx-test-diverged-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let config_file = path.join(".aws").join("config");
        let credentials_file = path.join(".aws").join("credentials");
        let available_profiles = vec!["meta".to_string(), "dev".to_string()];

        persist_active_profile_to_paths(&config_file, &credentials_file, "meta")
            .expect("persist should succeed");

        let mut credentials =
            Ini::load_from_file(&credentials_file).expect("credentials should exist");
        set_selected_profile(&mut credentials, "dev");
        write_ini_to_path(&credentials_file, &credentials, "credentials")
            .expect("credentials write should succeed");

        let persisted =
            read_persisted_profile_from_paths(&config_file, &credentials_file, &available_profiles)
                .expect("read should succeed");

        assert_eq!(persisted, None);

        fs::remove_dir_all(path).expect("cleanup should succeed");
    }

    #[test]
    fn test_profile_summaries_ignore_non_profile_config_sections() {
        let config = r#"
[sso-session corp]
sso_start_url=https://example.awsapps.com/start

[profile dev]
region=us-west-2
"#;

        let summaries = parse_profile_summaries(config, "");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "dev");
    }

    #[test]
    fn test_profile_summaries_include_account_region_and_details() {
        let config = r#"
[profile default]
region=us-east-1
output=json

[profile sso-dev]
region=us-west-2
sso_account_id=123456789012
sso_role_name=DeveloperAccess

[profile prod-admin]
role_arn=arn:aws:iam::210987654321:role/AdminAccess
source_profile=base
region=eu-central-1

[profile chained]
source_profile=prod-admin
"#;
        let credentials = r#"
[base]
aws_access_key_id=AKIAEXAMPLE
aws_secret_access_key=secret
"#;

        let summaries = parse_profile_summaries(config, credentials);

        assert_eq!(summaries[0].name, "default");
        assert_eq!(summaries[0].auth_type, "config");
        assert_eq!(summaries[0].region.as_deref(), Some("us-east-1"));
        assert!(summaries[0].details.contains(&"output=json".to_string()));

        let sso = summaries
            .iter()
            .find(|summary| summary.name == "sso-dev")
            .expect("sso-dev summary should exist");
        assert_eq!(sso.account_id.as_deref(), Some("123456789012"));
        assert_eq!(sso.auth_type, "sso");
        assert_eq!(sso.region.as_deref(), Some("us-west-2"));
        assert!(
            sso.details
                .contains(&"sso_role=DeveloperAccess".to_string())
        );

        let prod = summaries
            .iter()
            .find(|summary| summary.name == "prod-admin")
            .expect("prod-admin summary should exist");
        assert_eq!(prod.account_id.as_deref(), Some("210987654321"));
        assert_eq!(prod.auth_type, "role");
        assert!(prod.details.contains(&"role=AdminAccess".to_string()));
        assert!(prod.details.contains(&"source=base".to_string()));

        let chained = summaries
            .iter()
            .find(|summary| summary.name == "chained")
            .expect("chained summary should exist");
        assert_eq!(chained.account_id.as_deref(), Some("210987654321"));
        assert_eq!(chained.auth_type, "source");
    }

    #[test]
    fn test_profile_summaries_handle_source_profile_cycles() {
        let config = r#"
[profile alpha]
source_profile=beta

[profile beta]
source_profile=alpha
"#;

        let summaries = parse_profile_summaries(config, "");
        let alpha = summaries
            .iter()
            .find(|summary| summary.name == "alpha")
            .expect("alpha summary should exist");

        assert_eq!(alpha.account_id, None);
    }

    #[test]
    fn test_update_default_profile_clears_stale_keys() {
        let path = std::env::temp_dir().join(format!(
            "awsx-test-stale-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let config_file = path.join(".aws").join("config");
        let credentials_file = path.join(".aws").join("credentials");

        // Create config with SSO profile in [default] and a static-creds profile
        fs::create_dir_all(config_file.parent().unwrap()).unwrap();
        let mut config = Ini::new();
        config
            .with_section(Some("default"))
            .set("region", "us-east-1")
            .set("sso_account_id", "111111111111")
            .set("sso_role_name", "OldRole")
            .set("sso_start_url", "https://old.example.com")
            .set(AWSX_SELECTED_PROFILE_KEY, "sso-old");
        config
            .with_section(Some("profile static-prof"))
            .set("region", "us-west-2")
            .set("output", "json");
        config.write_to_file(&config_file).unwrap();

        // Create credentials with a static profile
        let mut creds = Ini::new();
        creds
            .with_section(Some("static-prof"))
            .set("aws_access_key_id", "AKIA123")
            .set("aws_secret_access_key", "secret123");
        creds.write_to_file(&credentials_file).unwrap();

        // Switch to static-prof
        update_default_profile_at_paths(&config_file, &credentials_file, "static-prof")
            .expect("update should succeed");

        // Verify config [default] has NO stale SSO keys
        let updated_config = Ini::load_from_file(&config_file).unwrap();
        let default_section = updated_config.section(Some("default")).unwrap();
        assert_eq!(default_section.get("region"), Some("us-west-2"));
        assert_eq!(default_section.get("output"), Some("json"));
        assert!(default_section.get("sso_account_id").is_none());
        assert!(default_section.get("sso_role_name").is_none());
        assert!(default_section.get("sso_start_url").is_none());
        // awsx marker should be preserved
        assert_eq!(
            default_section.get(AWSX_SELECTED_PROFILE_KEY),
            Some("sso-old")
        );

        // Verify credentials [default] has the static creds
        let updated_creds = Ini::load_from_file(&credentials_file).unwrap();
        let default_creds = updated_creds.section(Some("default")).unwrap();
        assert_eq!(default_creds.get("aws_access_key_id"), Some("AKIA123"));
        assert_eq!(
            default_creds.get("aws_secret_access_key"),
            Some("secret123")
        );

        fs::remove_dir_all(path).expect("cleanup should succeed");
    }

    #[test]
    fn test_format_profile_summaries_renders_table() {
        let summaries = vec![AwsProfileSummary {
            name: "dev".to_string(),
            account_id: Some("123456789012".to_string()),
            auth_type: "sso".to_string(),
            region: Some("us-west-2".to_string()),
            details: vec!["sso_role=DeveloperAccess".to_string()],
        }];

        let rendered = format_profile_summaries(&summaries);

        assert!(rendered.contains("PROFILE"));
        assert!(rendered.contains("ACCOUNT"));
        assert!(rendered.contains("dev"));
        assert!(rendered.contains("123456789012"));
        assert!(rendered.contains("sso_role=DeveloperAccess"));
    }
}
