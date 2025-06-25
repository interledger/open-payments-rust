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
//! ```rust,no_run
//! use open_payments::client::{AuthenticatedClient, ClientConfig, AuthenticatedResources, UnauthenticatedResources};
//! use open_payments::types::{GrantRequest, AccessTokenRequest, AccessItem, IncomingPaymentAction, CreateIncomingPaymentRequest};
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
//!     // Example of how to use the client (would require actual server)
//!     let wallet_address = client.wallet_address().get("https://rafiki.money/alice").await?;
//!
//!     // Example of how to request a grant
//!     let grant_request = GrantRequest {
//!         access_token: AccessTokenRequest {
//!             access: vec![AccessItem::IncomingPayment {
//!                 actions: vec![IncomingPaymentAction::Create, IncomingPaymentAction::Read],
//!                 identifier: None,
//!             }],
//!         },
//!         client: "https://rafiki.money/alice".to_string(),
//!         interact: None,
//!     };
//! 
//!     let access_token = client.grant().request(&wallet_address.auth_server, &grant_request).await?;
//!
//!     // Example of creating a payment request
//!     let resource_server = "https://ilp.rafiki.money";
//!     let access_token = "your-access-token";
//!     let payment_request = CreateIncomingPaymentRequest {
//!         wallet_address: wallet_address.id,
//!         incoming_amount: None,
//!         expires_at: None,
//!         metadata: None,
//!     };
//!     
//!     // This would make actual HTTP requests in a real scenario
//!     let payment = client
//!          .incoming_payments()
//!          .create(resource_server, &payment_request, Some(access_token))
//!          .await?;
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
pub mod config;
pub mod core;
pub mod error;
pub mod grant;
pub mod payments;
pub mod quotes;
pub mod request;
pub mod token;
pub mod utils;
pub mod wallet_address;

pub use api::{AuthenticatedResources, UnauthenticatedResources};
pub use config::ClientConfig;
pub use core::{AuthenticatedClient, UnauthenticatedClient};
pub use core::{AuthenticatedOpenPaymentsClient, BaseClient, UnauthenticatedOpenPaymentsClient};
pub use error::{OpClientError, Result};
