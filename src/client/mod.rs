//! # Open Payments HTTP Client
//!
//! This module provides HTTP client functionality for interacting with Open Payments servers.
//! It includes both authenticated and unauthenticated clients, along with utilities for
//! making requests, handling authentication, and managing resources.
//!
//! ## Client Types
//!
//! - [`AuthenticatedClient`] - Client for making authenticated requests with HTTP signatures
//! - [`UnauthenticatedClient`] - Client for making unauthenticated requests to public endpoints
//!
//! ## Configuration
//!
//! - [`ClientConfig`] - Configuration for authenticated clients including private key and key ID
//!
//! ## Resource APIs
//!
//! The client provides access to different Open Payments resources through dedicated APIs:
//!
//! - **Wallet Address**: [`mod@wallet_address`] - Get wallet address information
//! - **Incoming Payments**: [`mod@payments`] - Create and manage incoming payments
//! - **Outgoing Payments**: [`mod@payments`] - Create and manage outgoing payments
//! - **Quotes**: [`mod@quotes`] - Create and retrieve payment quotes
//! - **Grants**: [`mod@grant`] - Request and manage access tokens
//! - **Tokens**: [`mod@token`] - Manage access tokens
//!
//! ## Example Usage
//!
//! ```rust
//! use open_payments::client::{AuthenticatedClient, ClientConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ClientConfig {
//!         private_key_path: "path/to/private-key.pem".into(),
//!         key_id: "my-key-id".to_string(),
//!         jwks_path: Some("path/to/jwks.json".into()),
//!     };
//!
//!     let client = AuthenticatedClient::new(config)?;
//!
//!     // Get wallet address
//!     let wallet_address = client.wallet_address().get("https://rafiki.money/alice").await?;
//!
//!     // Create an incoming payment
//!     let resource_server = "https://ilp.rafiki.money";
//!     let access_token = "your-access-token";
//!     
//!     let payment = client
//!         .incoming_payments()
//!         .create(resource_server, &payment_request, Some(access_token))
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All client operations return a [`Result<T, OpClientError>`] where `OpClientError` provides
//! detailed error information for different failure scenarios.

pub mod api;
pub mod core;
pub mod config;
pub mod error;
pub mod grant;
pub mod payments;
pub mod quotes;
pub mod request;
pub mod token;
pub mod utils;
pub mod wallet_address;

pub use api::{AuthenticatedResources, UnauthenticatedResources};
pub use core::{AuthenticatedClient, UnauthenticatedClient};
pub use config::ClientConfig;
pub use error::{OpClientError, Result};
pub use core::{AuthenticatedOpenPaymentsClient, UnauthenticatedOpenPaymentsClient, BaseClient}; 