use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::error::{AppError, AppResult};

/// Contains all config options that can be set in the file.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
}

impl Config {
    /// Tries to parse toml file for given path into new Config instance.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the config file.
    pub fn from_file(path: &str) -> AppResult<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Checks if all necessary config fields were set.
    /// Returns user-friendly error message otherwise.
    pub fn validate(&self) -> AppResult<()> {
        if self.api_key.trim().is_empty() {
            return Err(AppError::ApiKeyMissing);
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "https://api.monkeytype.com".into(),
            api_key: String::new(),
        }
    }
}
