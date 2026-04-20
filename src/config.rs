use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::error::{AppError, AppResult};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
}

impl Config {
    pub fn from_file(path: &Path) -> AppResult<Self> {
        let mut file = File::open(path).map_err(|_| AppError::ConfigMissing)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn default_path() -> AppResult<PathBuf> {
        let mut path = dirs::config_dir().ok_or(AppError::NoConfigDir)?;
        path.push("monkeyfetch");
        path.push("config.toml");
        Ok(path)
    }

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
