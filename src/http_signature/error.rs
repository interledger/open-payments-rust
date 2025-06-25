//! # HTTP Signature Error Types
//!
//! This module defines the error types used throughout the HTTP signature functionality.
//! All signature operations return a [`Result<T, HttpSignatureError>`] which provides
//! detailed error information for different failure scenarios.
//!
//! ## Error Categories
//!
//! - **I/O Errors**: File system and network I/O problems
//! - **Parsing Errors**: Key format and data parsing failures
//! - **Cryptographic Errors**: Signature creation and validation issues
//! - **JWK Errors**: JSON Web Key format and processing problems
//!
//! ## Example Usage
//!
//! ```rust
//! use open_payments::http_signature::{HttpSignatureError, Result};
//!
//! fn handle_signature_error(result: Result<()>) {
//!     match result {
//!         Ok(()) => println!("Signature operation successful"),
//!         Err(HttpSignatureError::Io(err)) => eprintln!("I/O error: {}", err),
//!         Err(HttpSignatureError::Signature(msg)) => eprintln!("Signature error: {}", msg),
//!         Err(HttpSignatureError::Validation(msg)) => eprintln!("Validation error: {}", msg),
//!         Err(e) => eprintln!("Unexpected error: {:?}", e),
//!     }
//! }
//! ```

use base64::DecodeError;
use std::io;
use thiserror::Error;

/// Error type for HTTP signature operations.
///
/// This enum provides detailed error information for different types of failures
/// that can occur during HTTP signature creation, validation, and key management.
/// Each variant includes context-specific error messages to help with debugging
/// and error handling.
///
/// ## Error Variants
///
/// - `Io` - File system and I/O errors
/// - `Base64` - Base64 encoding/decoding errors
/// - `Pem` - PEM format parsing errors
/// - `Pkcs8` - PKCS8 key format errors
/// - `InvalidPrivateKeyLength` - Private key length validation errors
/// - `Utf8` - UTF-8 encoding/decoding errors
/// - `Jwk` - JSON Web Key format errors
/// - `Signature` - Signature creation and verification errors
/// - `Validation` - Signature validation and verification errors
/// - `Other` - Miscellaneous errors
#[derive(Debug, Error)]
pub enum HttpSignatureError {
    /// File system and I/O errors.
    ///
    /// This error is automatically converted from `std::io::Error` and occurs
    /// when the signature module cannot read key files or perform other I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    /// Base64 encoding or decoding errors.
    ///
    /// This error is automatically converted from `base64::DecodeError` and occurs
    /// when the signature module cannot decode Base64-encoded data such as signatures,
    /// keys, or other encoded content.
    #[error("Base64 decode error: {0}")]
    Base64(#[from] DecodeError),
    
    /// PEM format parsing errors.
    ///
    /// Occurs when the signature module cannot parse PEM-encoded private keys or certificates.
    /// This includes malformed PEM files, unsupported PEM types, or invalid PEM structure.
    #[error("PEM parse error: {0}")]
    Pem(String),
    
    /// PKCS8 key format errors.
    ///
    /// Occurs when the signature module cannot parse PKCS8-encoded private keys.
    /// This includes unsupported key algorithms, malformed key data, or invalid
    /// PKCS8 structure.
    #[error("PKCS8 parse error: {0}")]
    Pkcs8(String),
    
    /// Private key length validation errors.
    ///
    /// Occurs when a private key has an invalid length for the expected algorithm.
    /// For Ed25519 keys, this typically means the key is not exactly 32 bytes long.
    #[error("Invalid private key length")]
    InvalidPrivateKeyLength,
    
    /// UTF-8 encoding or decoding errors.
    ///
    /// This error is automatically converted from `std::string::FromUtf8Error` and occurs
    /// when the signature module cannot convert byte data to UTF-8 strings, typically
    /// when processing key files or signature data.
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    
    /// JSON Web Key format errors.
    ///
    /// Occurs when there are issues with JWK format, parsing, or processing.
    /// This includes malformed JWK JSON, unsupported key types, or invalid
    /// JWK structure.
    #[error("JWK error: {0}")]
    Jwk(String),
    
    /// Signature creation and verification errors.
    ///
    /// Occurs when there are issues with HTTP message signature creation or verification.
    /// This includes key loading failures, signature generation errors, or algorithm
    /// compatibility issues.
    #[error("Signature error: {0}")]
    Signature(String),
    
    /// Signature validation and verification errors.
    ///
    /// Occurs when signature validation fails, including expired signatures,
    /// invalid signature formats, or verification failures against public keys.
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Miscellaneous errors that don't fit into other categories.
    ///
    /// This variant is used for errors that are specific to the HTTP signature
    /// implementation or other unexpected issues that don't fall into the standard
    /// error categories.
    #[error("Other: {0}")]
    Other(String),
}

/// Result type for HTTP signature operations.
///
/// This is a type alias for `Result<T, HttpSignatureError>` that provides a convenient
/// way to handle signature operation results with detailed error information.
pub type Result<T> = std::result::Result<T, HttpSignatureError>;
