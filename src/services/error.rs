use std::{fmt::Display, sync::PoisonError};
use thiserror::Error;

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Request failed: {0}")]
    Request(reqwest::Error),
    #[error("Platform not supported")]
    UnsupportedPlatform,
    #[error(r#"Process "{0}" not found"#)]
    ProcessNotFound(String),
    #[error("Failed to aquire lock: {0}")]
    LockPoisoned(String),
}

impl From<reqwest::Error> for ServiceError {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

impl<T> From<PoisonError<T>> for ServiceError
where
    T: Display,
{
    fn from(err: PoisonError<T>) -> Self {
        let message = format!("{}", err.into_inner());

        Self::LockPoisoned(message)
    }
}
