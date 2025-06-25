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
//! ## Quick Start
//!
//! ```rust
//! use open_payments::http_signature::{create_signature_headers, SignOptions};
//! use http::{Request, Method, Uri};
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//!
//! // Create a test request
//! let mut request = Request::new(Some("test body".to_string()));
//! *request.method_mut() = Method::POST;
//! *request.uri_mut() = Uri::from_static("http://example.com/");
//!
//! // Generate a signing key
//! let signing_key = SigningKey::generate(&mut OsRng);
//!
//! // Create signature headers
//! let options = SignOptions::new(&request, &signing_key, "test-key".to_string());
//! let headers = create_signature_headers(options)?;
//!
//! println!("Signature: {}", headers.signature);
//! println!("Signature-Input: {}", headers.signature_input);
//! ```
//!
//! ## Signature Validation
//!
//! ```rust
//! use open_payments::http_signature::{validate_signature, ValidationOptions};
//! use http::{HeaderMap, Request};
//! use ed25519_dalek::VerifyingKey;
//!
//! // Assuming you have a request, headers, and public key
//! let options = ValidationOptions::new(&request, &headers, &public_key);
//! validate_signature(options)?;
//! ```
//!
//! ## Key Management
//!
//! ```rust
//! use open_payments::http_signature::load_or_generate_key;
//!
//! // Load existing key or generate a new one
//! let signing_key = load_or_generate_key("path/to/key.pem")?;
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
pub use self::signatures::{SignOptions, SignatureHeaders, create_signature_headers};
pub use self::utils::load_or_generate_key;
pub use self::validation::{ValidationOptions, validate_signature}; 