use thiserror::Error;

pub type Result<T> = std::result::Result<T, BlindPayError>;

#[derive(Error, Debug)]
pub enum BlindPayError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Missing API key")]
    MissingApiKey,

    #[error("Missing instance ID")]
    MissingInstanceId,

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}
