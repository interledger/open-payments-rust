//! # Open Payments Types
//!
//! This module contains all the type definitions for the Open Payments specifications.
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
//! - [`Receiver`] - Receiver of a payment e.g. incoming payment
//! - [`WalletAddressUri`] - Wallet address descriptor
//! - [`Interval`] - ISO 8601 defined interval
//!
//! ## Example Usage
//!
//! ```rust
//! use open_payments::types::{
//!     Amount, GrantRequest, AccessTokenRequest, AccessItem, QuoteAction,
//!     CreateIncomingPaymentRequest, WalletAddress, IncomingPaymentAction
//! };
//!
//! // Create a grant request for incoming payment
//! let grant_request = GrantRequest::new(
//!     AccessTokenRequest {
//!         access: vec![AccessItem::IncomingPayment {
//!             actions: vec![IncomingPaymentAction::Create, IncomingPaymentAction::Read],
//!             identifier: None,
//!         }],
//!     },
//!     None,
//! );
//!
//! // Create an incoming payment request
//! let payment_request = CreateIncomingPaymentRequest {
//!     wallet_address: "https://rafiki.money/alice".to_string(),
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
    AccessItem, AccessToken, AccessTokenRequest, AccessTokenResponse, Continue, ContinueRequest,
    ContinueResponse, GrantRequest, GrantResponse, IncomingPaymentAction, InteractRequest,
    InteractResponse, LimitsOutgoing, OutgoingPaymentAction, QuoteAction,
};

pub use resource::{
    CreateIncomingPaymentRequest, CreateIncomingPaymentRequest as IncomingPaymentRequest,
    CreateOutgoingPaymentRequest, CreateOutgoingPaymentRequest as OutgoingPaymentRequest,
    CreateQuoteRequest, CreateQuoteRequest as QuoteRequest, IncomingPayment,
    IncomingPaymentWithMethods, ListIncomingPaymentsResponse, ListOutgoingPaymentsResponse,
    OutgoingPayment, PageInfo, PaginatedResponse, PaymentMethod, PaymentMethodType,
    PublicIncomingPayment, Quote,
};

pub use wallet_address::{
    JsonWebKey, JsonWebKeySet, JwkAlgorithm, JwkCurve, JwkKeyType, JwkUse, WalletAddress,
};
