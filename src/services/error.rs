use thiserror::Error;

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Platform not supported")]
    UnsupportedPlatform,
    #[error("Process {0} not found")]
    ProcessNotFound(String),
}
