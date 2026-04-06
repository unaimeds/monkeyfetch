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

    #[error("api_key in config must not be empty")]
    ApiKeyMissing,
}
