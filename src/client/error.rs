//! # Open Payments Client Error Types
//!
//! This module defines the error types used throughout the Open Payments client.
//! All client operations return a [`Result<T, OpClientError>`] which provides
//! detailed error information for different failure scenarios.
//!
//! ## Error Structure
//!
//! - `description` - Human-readable error message
//! - `validationErrors` - Optional list of validation error messages
//! - `status` - HTTP status code (only for HTTP errors)
//! - `code` - Error code (only for HTTP errors)
//! - `details` - Additional error details as key-value pairs
//!
//! ## Example Usage
//!
//! ```rust
//! use open_payments::client::{OpClientError, Result};
//!
//! fn handle_client_error(result: Result<()>) {
//!     match result {
//!         Ok(()) => println!("Operation successful"),
//!         Err(e) => {
//!             eprintln!("Error: {}", e.description);
//!             if let Some(status) = e.status {
//!                 eprintln!("Status: {}", status);
//!             }
//!             if let Some(code) = e.code {
//!                 eprintln!("Code: {}", code);
//!             }
//!             if let Some(validation_errors) = e.validation_errors {
//!                 for error in validation_errors {
//!                     eprintln!("Validation error: {}", error);
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use std::collections::HashMap;
use thiserror::Error;

/// Error type for Open Payments client operations.
///
/// ## Fields
///
/// - `description` - Human-readable error message
/// - `validation_errors` - Optional list of validation error messages
/// - `status` - HTTP status code (only relevant for HTTP errors)
/// - `code` - Error code (only relevant for HTTP errors)
/// - `details` - Additional error details as key-value pairs
#[derive(Debug, Error)]
pub struct OpClientError {
    /// Human-readable error description.
    pub description: String,

    /// Optional list of validation error messages.
    pub validation_errors: Option<Vec<String>>,

    /// HTTP status (only relevant for HTTP errors).
    pub status: Option<String>,

    /// Error code (only relevant for HTTP errors).
    pub code: Option<u16>,

    /// Additional error details as key-value pairs.
    pub details: Option<HashMap<String, serde_json::Value>>,
}

impl OpClientError {
    /// Creates a new HTTP error with status code and optional error code.
    pub fn http(description: impl Into<String>, status: Option<String>, code: Option<u16>) -> Self {
        Self {
            description: description.into(),
            validation_errors: None,
            status,
            code,
            details: None,
        }
    }

    /// Creates a new validation error with a list of validation messages.
    pub fn validation(description: impl Into<String>, validation_errors: Vec<String>) -> Self {
        Self {
            description: description.into(),
            validation_errors: Some(validation_errors),
            status: None,
            code: None,
            details: None,
        }
    }

    /// Creates a new general error without HTTP-specific fields.
    pub fn other(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            validation_errors: None,
            status: None,
            code: None,
            details: None,
        }
    }

    /// Adds additional details to the error.
    pub fn with_details(mut self, details: HashMap<String, serde_json::Value>) -> Self {
        self.details = Some(details);
        self
    }

    /// Adds a single detail key-value pair to the error.
    pub fn with_detail(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        if self.details.is_none() {
            self.details = Some(HashMap::new());
        }
        if let Some(ref mut details) = self.details {
            details.insert(key.into(), value);
        }
        self
    }
}

impl std::fmt::Display for OpClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)?;

        if let Some(status) = &self.status {
            write!(f, " (Status: {status})")?;
        }

        if let Some(code) = &self.code {
            write!(f, " (Code: {code})")?;
        }

        if let Some(validation_errors) = &self.validation_errors {
            write!(f, " [Validation errors: {}]", validation_errors.join(", "))?;
        }

        Ok(())
    }
}

impl From<reqwest::Error> for OpClientError {
    fn from(err: reqwest::Error) -> Self {
        let status = err.status().map(|s| s.to_string());
        let code = err.status().map(|s| s.as_u16());
        let description = format!("HTTP error: {err}");

        Self {
            description,
            validation_errors: None,
            status,
            code,
            details: None,
        }
    }
}

impl From<serde_json::Error> for OpClientError {
    fn from(err: serde_json::Error) -> Self {
        Self::other(format!("JSON serialization/deserialization error: {err}"))
    }
}

impl From<std::io::Error> for OpClientError {
    fn from(err: std::io::Error) -> Self {
        Self::other(format!("I/O error: {err}"))
    }
}

impl From<base64::DecodeError> for OpClientError {
    fn from(err: base64::DecodeError) -> Self {
        Self::other(format!("Base64 decoding error: {err}"))
    }
}

impl From<url::ParseError> for OpClientError {
    fn from(err: url::ParseError) -> Self {
        Self::other(format!("URL parsing error: {err}"))
    }
}

impl OpClientError {
    pub fn header_parse(description: impl Into<String>) -> Self {
        Self::other(format!("Header parse error: {}", description.into()))
    }

    pub fn pem(description: impl Into<String>) -> Self {
        Self::other(format!("Invalid PEM: {}", description.into()))
    }

    pub fn pkcs8(description: impl Into<String>) -> Self {
        Self::other(format!("PKCS8 error: {}", description.into()))
    }

    pub fn signature(description: impl Into<String>) -> Self {
        Self::other(format!("Signature error: {}", description.into()))
    }
}

impl From<url::ParseError> for Box<OpClientError> {
    fn from(err: url::ParseError) -> Self {
        Box::new(OpClientError::from(err))
    }
}

impl From<reqwest::Error> for Box<OpClientError> {
    fn from(err: reqwest::Error) -> Self {
        Box::new(OpClientError::from(err))
    }
}

impl From<serde_json::Error> for Box<OpClientError> {
    fn from(err: serde_json::Error) -> Self {
        Box::new(OpClientError::from(err))
    }
}

impl From<std::io::Error> for Box<OpClientError> {
    fn from(err: std::io::Error) -> Self {
        Box::new(OpClientError::from(err))
    }
}

impl From<base64::DecodeError> for Box<OpClientError> {
    fn from(err: base64::DecodeError) -> Self {
        Box::new(OpClientError::from(err))
    }
}

/// Result type for Open Payments client operations.
///
/// This is a type alias for `Result<T, Box<OpClientError>>` that provides a convenient
/// way to handle client operation results with detailed error information.
pub type Result<T> = std::result::Result<T, Box<OpClientError>>;
