use std::io;

use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("api_key in config must not be empty")]
    ApiKeyMissing,

    #[error("could not determine config directory for this platform")]
    NoConfigDir,

    #[error("could not read config file, is it missing?")]
    ConfigMissing,
}
