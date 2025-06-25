//! # Open Payments Types
//!
//! This module contains all the type definitions for the Open Payments protocol.
//! These types represent the data structures used in API requests and responses,
//! as well as the core concepts of the Open Payments ecosystem.
//!
//! ## Core Types
//!
//! ### Authentication & Authorization
//!
//! - [`GrantRequest`] / [`GrantResponse`] - GNAP request and response
//! - [`AccessToken`] / [`AccessTokenRequest`] - Access token management
//! - [`AccessItem`] - Defines what resources a token can access
//! - [`ContinueRequest`] / [`ContinueResponse`] - Grant continuation flow
//!
//! ### Resources
//!
//! - [`WalletAddress`] - Wallet address information
//! - [`IncomingPayment`] - Incoming payment details
//! - [`OutgoingPayment`] - Outgoing payment details
//! - [`Quote`] - Payment quote details
//!
//! ### Common Types
//!
//! - [`Amount`] - Amounts of a specific currency and scale
//! - [`PageInfo`] - Pagination information
//! - [`PaginatedResponse`] - Generic paginated response wrapper
//!
//! ## Example Usage
//!
//! ```rust
//! use open_payments::types::{
//!     Amount, GrantRequest, AccessTokenRequest, AccessItem, QuoteAction,
//!     CreateIncomingPaymentRequest, WalletAddress
//! };
//!
//! // Create a grant request for quote access
//! let grant_request = GrantRequest {
//!     access_token: AccessTokenRequest {
//!         access: vec![AccessItem::Quote {
//!             actions: vec![QuoteAction::Create, QuoteAction::Read],
//!         }],
//!     },
//!     client: "https://example.com".to_string(),
//!     interact: None,
//! };
//!
//! // Create an incoming payment request
//! let payment_request = CreateIncomingPaymentRequest {
//!     wallet_address: "https://example.com/.well-known/pay".to_string(),
//!     incoming_amount: Some(Amount {
//!         value: "1000".to_string(),
//!         asset_code: "EUR".to_string(),
//!         asset_scale: 2,
//!     }),
//!     expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
//!     metadata: None,
//! };
//! ```
//!
//! ## Module Structure
//!
//! - [`auth`] - Authentication and authorization types
//! - [`common`] - Common types used across Open Payments
//! - [`resource`] - Resource-specific types (payments, quotes, etc.)
//! - [`wallet_address`] - Wallet address and JWK types

pub mod auth;
pub mod common;
pub mod resource;
pub mod wallet_address;

pub use common::*;

pub use auth::{
    AccessItem, AccessToken, Continue, ContinueRequest, ContinueResponse, GrantRequest,
    GrantResponse, IncomingPaymentAction, InteractRequest, InteractResponse, LimitsOutgoing,
    OutgoingPaymentAction, QuoteAction, AccessTokenResponse, AccessTokenRequest
};

pub use resource::{
    CreateIncomingPaymentRequest, CreateOutgoingPaymentRequest, CreateQuoteRequest,
    IncomingPayment, IncomingPaymentWithMethods, ListIncomingPaymentsResponse,
    ListOutgoingPaymentsResponse, OutgoingPayment, PageInfo, PaginatedResponse, PaymentMethod,
    PaymentMethodType, Quote, PublicIncomingPayment,
    CreateIncomingPaymentRequest as IncomingPaymentRequest,
    CreateOutgoingPaymentRequest as OutgoingPaymentRequest,
    CreateQuoteRequest as QuoteRequest,
};

pub use wallet_address::{
    JsonWebKey, JsonWebKeySet, JwkAlgorithm, JwkCurve, JwkKeyType, JwkUse,
    WalletAddress,
}; 