use notify::{Event, RecursiveMode, Watcher};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use validator::Validate;

fn empty_string_vector() -> Vec<String> {
    let empty_vec: Vec<String> = Vec::new();
    empty_vec
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct Config {
    #[serde(default = "empty_string_vector")]
    pub(crate) allowed_domains: Vec<String>,
    #[serde(default = "empty_string_vector")]
    pub(crate) allowed_mails: Vec<String>,
    pub(crate) token: String,
}

impl Config {
    pub fn normalize(&mut self) {
        self.allowed_domains = self
            .allowed_domains
            .iter()
            .map(|d| d.to_lowercase())
            .collect();
        self.allowed_mails = self
            .allowed_mails
            .iter()
            .map(|m| m.to_lowercase())
            .collect();
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmailRequest {
    #[validate(email)]
    pub(crate) email: String,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    let config_str = fs::read_to_string(path)?;
    let mut config: Config = serde_yaml::from_str(&config_str)?;
    config.validate()?;
    config.normalize();
    Ok(config)
}

pub async fn watch_config_updates(
    path: &Path,
    sender: Sender<Result<Config, Box<dyn std::error::Error + Send + Sync>>>,
) {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    for _ in rx {
        sender.send(load_config(path.to_str().unwrap())).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn empty_string_vector_returns_empty_vec() {
        let v = empty_string_vector();
        assert!(v.is_empty());
    }

    #[test]
    fn config_normalize_lowercases_domains_and_mails() {
        let mut cfg = Config {
            allowed_domains: vec!["EXAMPLE.COM".into(), "Sub.Domain.Org".into()],
            allowed_mails: vec!["User@Example.COM".into(), "ADMIN@ORG.NET".into()],
            token: "secret".into(),
        };

        cfg.normalize();

        assert_eq!(cfg.allowed_domains, vec!["example.com", "sub.domain.org"]);
        assert_eq!(cfg.allowed_mails, vec!["user@example.com", "admin@org.net"]);
    }

    #[test]
    fn load_config_reads_and_normalizes_yaml() {
        // YAML with mixed-case values to verify normalization and defaults handling
        let yaml = r#"
allowed_domains:
  - ExAmPlE.com
  - API.Service.IO
allowed_mails:
  - USER@One.Org
token: SUPER_TOKEN
"#;

        let mut tmp = NamedTempFile::new().expect("tmp file");
        write!(tmp, "{yaml}").unwrap();

        let loaded = load_config(tmp.path().to_str().unwrap()).expect("config loads");
        assert_eq!(loaded.token, "SUPER_TOKEN");
        assert_eq!(
            loaded.allowed_domains,
            vec!["example.com", "api.service.io"]
        );
        assert_eq!(loaded.allowed_mails, vec!["user@one.org"]);
    }

    #[test]
    fn load_config_applies_defaults_when_missing() {
        // Missing allowed_domains and allowed_mails should default to empty vectors
        let yaml = r#"
token: t123
"#;
        let mut tmp = NamedTempFile::new().expect("tmp file");
        write!(tmp, "{yaml}").unwrap();

        let loaded = load_config(tmp.path().to_str().unwrap()).expect("config loads with defaults");
        assert_eq!(loaded.token, "t123");
        assert!(loaded.allowed_domains.is_empty());
        assert!(loaded.allowed_mails.is_empty());
    }

    #[test]
    fn email_request_validate_accepts_valid_email() {
        let req = EmailRequest {
            email: "user@example.com".into(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn email_request_validate_rejects_invalid_email() {
        let req = EmailRequest {
            email: "not-an-email".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn load_config_errors_on_missing_file() {
        let err = load_config("/path/does/not/exist.yaml").unwrap_err();
        // It should be some IO error; just assert that we got an error
        let _ = format!("{err}"); // ensure it’s Display-able
    }

    #[test]
    fn load_config_errors_on_invalid_yaml() {
        let yaml = "token: [unterminated"; // invalid YAML
        let mut tmp = NamedTempFile::new().expect("tmp file");
        write!(tmp, "{yaml}").unwrap();

        let err = load_config(tmp.path().to_str().unwrap()).unwrap_err();
        let _ = format!("{err}");
    }
}
