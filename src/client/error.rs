//! # Open Payments Client Error Types
//!
//! This module defines the error types used throughout the Open Payments client.
//! All client operations return a [`Result<T, OpClientError>`] which provides
//! detailed error information for different failure scenarios.
//!
//! ## Error Categories
//!
//! - **HTTP Errors**: Network and HTTP protocol errors
//! - **Parsing Errors**: Header, URL, and data parsing failures
//! - **Cryptographic Errors**: Key and signature-related issues
//! - **I/O Errors**: File system and network I/O problems
//!
//! ## Example Usage
//!
//! ```rust
//! use open_payments::client::{OpClientError, Result};
//!
//! fn handle_client_error(result: Result<()>) {
//!     match result {
//!         Ok(()) => println!("Operation successful"),
//!         Err(OpClientError::Http(msg)) => eprintln!("HTTP error: {}", msg),
//!         Err(OpClientError::Signature(msg)) => eprintln!("Signature error: {}", msg),
//!         Err(OpClientError::Other(msg)) => eprintln!("Other error: {}", msg),
//!         Err(e) => eprintln!("Unexpected error: {:?}", e),
//!     }
//! }
//! ```

use thiserror::Error;

/// Error type for Open Payments client operations.
///
/// This enum provides detailed error information for different types of failures
/// that can occur during client operations. Each variant includes context-specific
/// error messages to help with debugging and error handling.
///
/// ## Error Variants
///
/// - `Http` - Network and HTTP protocol errors
/// - `HeaderParse` - HTTP header parsing failures
/// - `Serde` - JSON serialization/deserialization errors
/// - `Io` - File system and I/O errors
/// - `Pem` - PEM format parsing errors
/// - `Pkcs8` - PKCS8 key format errors
/// - `Base64` - Base64 encoding/decoding errors
/// - `Url` - URL parsing errors
/// - `Signature` - Cryptographic signature errors
/// - `Other` - Miscellaneous errors
#[derive(Debug, Error)]
pub enum OpClientError {
    /// HTTP protocol or network-related errors.
    ///
    /// This includes connection failures, timeout errors, and HTTP status code errors.
    /// The error message provides details about the specific HTTP issue.
    #[error("HTTP error: {0}")]
    Http(String),

    /// HTTP header parsing errors.
    ///
    /// Occurs when the client cannot parse required HTTP headers such as
    /// `Content-Type`, `Authorization`, or custom headers.
    #[error("Header parse error: {0}")]
    HeaderParse(String),

    /// JSON serialization or deserialization errors.
    ///
    /// This error is automatically converted from `serde_json::Error` and occurs
    /// when the client cannot serialize request data or deserialize response data.
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    /// File system and I/O errors.
    ///
    /// This error is automatically converted from `std::io::Error` and occurs
    /// when the client cannot read key files or perform other I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// PEM format parsing errors.
    ///
    /// Occurs when the client cannot parse PEM-encoded private keys or certificates.
    /// This includes malformed PEM files or unsupported PEM types.
    #[error("Invalid PEM: {0}")]
    Pem(String),

    /// PKCS8 key format errors.
    ///
    /// Occurs when the client cannot parse PKCS8-encoded private keys.
    /// This includes unsupported key algorithms or malformed key data.
    #[error("PKCS8 error: {0}")]
    Pkcs8(String),

    /// Base64 encoding or decoding errors.
    ///
    /// This error is automatically converted from `base64::DecodeError` and occurs
    /// when the client cannot decode Base64-encoded data such as signatures or keys.
    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),

    /// URL parsing errors.
    ///
    /// This error is automatically converted from `url::ParseError` and occurs
    /// when the client cannot parse URLs for wallet addresses or API endpoints.
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    /// Cryptographic signature errors.
    ///
    /// Occurs when there are issues with HTTP message signature creation or validation.
    /// This includes key loading failures, signature generation errors, and validation failures.
    #[error("Signature error: {0}")]
    Signature(String),

    /// Miscellaneous errors that don't fit into other categories.
    ///
    /// This variant is used for unexpected errors that don't fall into the standard error categories.
    #[error("Other error: {0}")]
    Other(String),
}

impl From<reqwest::Error> for OpClientError {
    /// Converts reqwest HTTP errors to `OpClientError::Http`.
    ///
    /// This implementation allows the client to automatically convert reqwest errors
    /// into the unified error type, providing consistent error handling across the library.
    fn from(err: reqwest::Error) -> Self {
        OpClientError::Http(format!("{err}"))
    }
}

/// Result type for Open Payments client operations.
///
/// This is a type alias for `Result<T, OpClientError>` that provides a convenient
/// way to handle client operation results with detailed error information.
pub type Result<T> = std::result::Result<T, OpClientError>;
