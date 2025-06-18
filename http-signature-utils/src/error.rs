use thiserror::Error;
use std::io;
use base64::DecodeError;

#[derive(Debug, Error)]
pub enum HttpSignatureError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Base64 decode error: {0}")]
    Base64(#[from] DecodeError),
    #[error("PEM parse error: {0}")]
    Pem(String),
    #[error("PKCS8 parse error: {0}")]
    Pkcs8(String),
    #[error("Invalid private key length")]
    InvalidPrivateKeyLength,
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("JWK error: {0}")]
    Jwk(String),
    #[error("Signature error: {0}")]
    Signature(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Other: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, HttpSignatureError>; 