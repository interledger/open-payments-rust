use crate::config::ClientConfig;
use crate::error::{OpClientError, Result};
use crate::http_signature::{jwk::Jwk, load_or_generate_key};
use ed25519_dalek::SigningKey;
use reqwest::{Client, Client as ReqwestClient};

/// Base trait for HTTP clients that provides access to the underlying reqwest client.
///
/// This trait is implemented by both authenticated and unauthenticated clients,
/// allowing generic code to work with either type.
pub trait BaseClient {
    /// Returns a reference to the underlying reqwest HTTP client.
    fn http_client(&self) -> &ReqwestClient;
}

/// An authenticated Open Payments client that can make signed HTTP requests.
///
/// This client automatically handles HTTP message signature creation for all requests
/// using the configured private key. It's used for operations that require authentication
/// such as creating payments, quotes and managing access tokens.
///
/// ## Example
///
/// ```rust,no_run
/// use open_payments::client::{AuthenticatedClient, ClientConfig, AuthenticatedResources, UnauthenticatedResources};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // In a real application, you would use actual file paths
///     let config = ClientConfig {
///         private_key_path: "path/to/private-key.pem".into(),
///         key_id: "my-key-id".to_string(),
///         jwks_path: Some("path/to/jwks.json".into()),
///         wallet_address_url: "https://rafiki.money/alice".into(),
///     };
///
///     // This would fail in a real scenario if the files don't exist
///     // but demonstrates the API usage
///     let _client = AuthenticatedClient::new(config)?;
///     Ok(())
/// }
/// ```
pub struct AuthenticatedOpenPaymentsClient {
    /// The underlying HTTP client for making requests.
    pub http_client: ReqwestClient,
    /// Client configuration including key paths and identifiers.
    pub config: ClientConfig,
    /// The signing key used for HTTP message signatures.
    pub signing_key: SigningKey,
}

impl BaseClient for AuthenticatedOpenPaymentsClient {
    fn http_client(&self) -> &ReqwestClient {
        &self.http_client
    }
}

impl AuthenticatedOpenPaymentsClient {
    /// Creates a new authenticated client with the given configuration.
    ///
    /// This method will:
    /// 1. Load or generate the signing key from the specified path
    /// 2. Generate and save JWKS if a JWKS path is provided
    /// 3. Create an HTTP client for making requests
    ///
    /// # Arguments
    ///
    /// * `config` - The client configuration containing key paths and identifiers
    ///
    /// # Returns
    ///
    /// Returns a configured authenticated client or an error if key loading fails.
    ///
    /// # Errors
    ///
    /// Returns an `OpClientError` with:
    /// - `description`: Human-readable error message
    /// - `status`: HTTP status text (for HTTP errors)
    /// - `code`: HTTP status code (for HTTP errors)
    /// - `validation_errors`: List of validation errors (if applicable)
    /// - `details`: Additional error details (if applicable)
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http_client = ReqwestClient::new();

        let signing_key = load_or_generate_key(&config.private_key_path).map_err(|e| {
            OpClientError::signature(format!("Failed to load or generate signing key: {e}"))
        })?;

        if let Some(ref jwks_path) = config.jwks_path {
            let jwks_json = Jwk::generate_jwks_json(&signing_key, &config.key_id);
            Jwk::save_jwks(&jwks_json, jwks_path).map_err(|e| {
                OpClientError::signature(format!("Failed to save JWK to file: {e}"))
            })?;
        }

        Ok(Self {
            http_client,
            config,
            signing_key,
        })
    }
}

/// An unauthenticated Open Payments client for making public requests.
///
/// This client is used for operations that don't require authentication,
/// such as retrieving public wallet address information or accessing
/// publicly available resources e.g. public incoming payments.
///
/// ## Example
///
/// ```rust,no_run
/// use open_payments::client::{UnauthenticatedClient, UnauthenticatedResources};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = UnauthenticatedClient::new();
///     // This would make an actual HTTP request in a real scenario
///     // but demonstrates the API usage
///     let _wallet_address = client.wallet_address().get("https://rafiki.money/alice").await?;
///     Ok(())
/// }
/// ```
pub struct UnauthenticatedOpenPaymentsClient {
    pub http_client: ReqwestClient,
}

impl Default for UnauthenticatedOpenPaymentsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl UnauthenticatedOpenPaymentsClient {
    /// Creates a new unauthenticated client.
    ///
    /// This creates a simple HTTP client without any authentication configuration.
    /// It's suitable for accessing public endpoints that don't require signatures.
    pub fn new() -> Self {
        Self {
            http_client: ReqwestClient::new(),
        }
    }
}

impl BaseClient for UnauthenticatedOpenPaymentsClient {
    fn http_client(&self) -> &ReqwestClient {
        &self.http_client
    }
}

impl BaseClient for Client {
    fn http_client(&self) -> &ReqwestClient {
        self
    }
}

pub use self::AuthenticatedOpenPaymentsClient as AuthenticatedClient;
pub use self::UnauthenticatedOpenPaymentsClient as UnauthenticatedClient;
