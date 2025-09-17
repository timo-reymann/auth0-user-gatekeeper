use std::{fs};
use serde::Deserialize;
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
        self.allowed_domains = self.allowed_domains.iter().map(|d| d.to_lowercase()).collect();
        self.allowed_mails = self.allowed_mails.iter().map(|m| m.to_lowercase()).collect();
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmailRequest {
    #[validate(email)]
    pub(crate) email: String,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let mut config: Config = serde_yaml::from_str(&config_str)?;
    config.validate()?;
    config.normalize();
    Ok(config)
}
