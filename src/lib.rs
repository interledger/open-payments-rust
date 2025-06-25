//! # Open Payments Rust Client
//!
//! A Rust client library for the Open Payments protocol, providing types and HTTP signatures utilities for building Open Payments applications.
//!
//! ## Features
//!
//! - **Types**: Complete type definitions for all Open Payments resources and operations
//! - **HTTP Client**: Async HTTP client with authentication and signature support
//! - **HTTP Signatures**: Utilities for creating and validating HTTP message signatures
//! - **Snippets**: Ready-to-use code examples for common operations (optional feature)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use open_payments::client::{AuthenticatedClient, ClientConfig, AuthenticatedResources, UnauthenticatedResources};
//! use open_payments::types::{IncomingPayment, OutgoingPayment};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // In a real application, you would use actual file paths
//!     let config = ClientConfig {
//!         private_key_path: "path/to/private-key.pem".into(),
//!         key_id: "my-key-id".to_string(),
//!         jwks_path: Some("path/to/jwks.json".into()),
//!     };
//!
//!     // This would fail in a real scenario if the files don't exist
//!     // but demonstrates the API usage
//!     let client = AuthenticatedClient::new(config)?;
//!
//!     // Use the client to interact with Open Payments resources
//!     let wallet_address_url = "https://rafiki.money/alice";
//!     let wallet_address = client.wallet_address().get(wallet_address_url).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Modules
//!
//! - [`client`] - HTTP client for making unauthenticated and authenticated requests to Open Payments servers
//! - [`types`] - Type definitions for all Open Payments resources and operations
//! - [`http_signature`] - Utilities for HTTP message signature creation and validation
//! - \[`snippets`\] - Code examples for common operations (requires `snippets` feature)
//!
//! ## Cargo Features
//!
//! - `snippets` - Enables snippet binaries with example code for common operations
//!
//! ## Examples
//!
//! ### Creating an Incoming Payment
//!
//! ```rust,no_run
//! use open_payments::client::{AuthenticatedClient, AuthenticatedResources};
//! use open_payments::types::{Amount, resource::CreateIncomingPaymentRequest};
//! use chrono::{Duration, Utc};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // In a real application, you would use actual file paths
//!     let config = open_payments::client::ClientConfig {
//!         private_key_path: "path/to/private-key.pem".into(),
//!         key_id: "my-key-id".to_string(),
//!         jwks_path: Some("path/to/jwks.json".into()),
//!     };
//!     
//!     // This would fail in a real scenario if the files don't exist
//!     // but demonstrates the API usage
//!     let client = AuthenticatedClient::new(config)?;
//!     
//!     let request = CreateIncomingPaymentRequest {
//!         wallet_address: "https://rafiki.money/alice".to_string(),
//!         incoming_amount: Some(Amount {
//!             value: "1000".to_string(),
//!             asset_code: "EUR".to_string(),
//!             asset_scale: 2,
//!         }),
//!         expires_at: Some(Utc::now() + Duration::hours(1)),
//!         metadata: None,
//!     };
//!
//!     let resource_server_url = "https://ilp.rafiki.money";
//!     let access_token = "your-access-token";
//!     
//!     let payment = client
//!         .incoming_payments()
//!         .create(&resource_server_url, &request, Some(&access_token))
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### HTTP Signature Creation
//!
//! ```rust
//! use open_payments::http_signature::{create_signature_headers, SignOptions};
//! use http::{Request, Method, Uri};
//! use ed25519_dalek::SigningKey;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut request = Request::new(Some("test body".to_string()));
//!     *request.method_mut() = Method::POST;
//!     *request.uri_mut() = Uri::from_static("https://ilp.rafiki.money/incoming-payments");
//!
//!     let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
//!     let options = SignOptions::new(&request, &signing_key, "test-key".to_string());
//!     let headers = create_signature_headers(options)?;
//!
//!     println!("Signature: {}", headers.signature);
//!     println!("Signature-Input: {}", headers.signature_input);
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

pub mod client;
pub mod types;
pub mod http_signature;
#[cfg(feature = "snippets")]
pub mod snippets;

// Re-export everything public from client at the crate root
pub use client::*; 