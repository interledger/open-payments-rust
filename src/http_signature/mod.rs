//! # HTTP Signature Utilities
//!
//! This module provides utilities for creating and validating HTTP message signatures
//! according to the [HTTP Message Signatures](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-message-signatures) specification.
//! It supports Ed25519 signing and is designed for use with the Open Payments protocol.
//!
//! ## Features
//!
//! - **Signature Creation**: Create HTTP message signatures with Ed25519 keys
//! - **Signature Validation**: Validate incoming HTTP message signatures
//! - **JWK Support**: Generate and handle JSON Web Keys
//! - **Key Management**: Load keys from files or generate new ones
//!
//! ## Signature Creation and Validation
//!
//! ```rust
//! use open_payments::http_signature::{create_signature_headers, validate_signature, SignOptions, ValidationOptions, HttpSignatureError};
//! use http::{Request, Method, Uri, HeaderMap};
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut request = Request::new(Some("test body".to_string()));
//!     *request.method_mut() = Method::POST;
//!     *request.uri_mut() = Uri::from_static("https://ilp.rafiki.money/incoming-payments");
//!
//!     let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
//!     let options = SignOptions::new(&request, &signing_key, "test-key".to_string());
//!     let signature_headers = create_signature_headers(options)?;
//!
//!     let mut headers = HeaderMap::new();
//!     headers.insert("Signature", signature_headers.signature.parse().map_err(|_| HttpSignatureError::Validation("Invalid signature".to_string()))?);
//!     headers.insert(
//!         "Signature-Input",
//!         signature_headers.signature_input.parse().map_err(|_| HttpSignatureError::Validation("Invalid signature input".to_string()))?,
//!     );
//!     let public_key = signing_key.verifying_key();
//!     let options = ValidationOptions::new(&request, &headers, &public_key);
//!     validate_signature(options)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Key Management
//!
//! ```rust,no_run
//! use open_payments::http_signature::load_or_generate_key;
//! use std::path::Path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Note: This will create the file if it doesn't exist
//!     let signing_key = load_or_generate_key(Path::new("test-key.pem"))?;
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All functions return a [`Result<T, HttpSignatureError>`] which provides detailed
//! error information for different failure scenarios including:
//!
//! - Invalid key formats
//! - Signature verification failures
//! - Missing required headers
//! - Base64 decoding errors
//!
//! ## Module Structure
//!
//! - [`signatures`] - Core signature creation functionality
//! - [`validation`] - Signature validation utilities
//! - [`jwk`] - JSON Web Key generation and handling
//! - [`utils`] - Key management utilities
//! - [`error`] - Error types and handling

pub mod error;
pub mod jwk;
pub mod signatures;
pub mod utils;
pub mod validation;

pub use self::error::{HttpSignatureError, Result};
pub use self::jwk::{Jwk, JwkError};
pub use self::signatures::{create_signature_headers, SignOptions, SignatureHeaders};
pub use self::utils::load_or_generate_key;
pub use self::validation::{validate_signature, ValidationOptions};
