use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpClientError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Header parse error: {0}")]
    HeaderParse(String),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid PEM: {0}")]
    Pem(String),
    #[error("PKCS8 error: {0}")]
    Pkcs8(String),
    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("Signature error: {0}")]
    Signature(String),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<reqwest::Error> for OpClientError {
    fn from(err: reqwest::Error) -> Self {
        OpClientError::Http(format!("{}", err))
    }
}

pub type Result<T> = std::result::Result<T, OpClientError>;
