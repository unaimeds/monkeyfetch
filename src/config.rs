use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::errors::AppResult;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn from_file(path: &str) -> AppResult<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}