/// Open Payments Client Library
///
/// Provides asynchronous methods to interact with Open Payments endpoints
/// (wallet address resolution, GNAP grant negotiation, payments, quotes, etc.).
/// It uses HTTP signatures (via http-signature-utils) and manages keys internally.
pub mod api;
pub mod client;
pub mod config;
pub mod grant;
pub mod payments;
pub mod quotes;
pub mod request;
pub mod token;
pub mod types;
pub mod utils;
pub mod wallet_address;

pub use api::{AuthenticatedResources, UnauthenticatedResources};
pub use client::{AuthenticatedClient, UnauthenticatedClient};
pub use config::ClientConfig;
pub use types::*;