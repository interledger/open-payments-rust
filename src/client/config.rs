use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 
/// Configuration for an authenticated Open Payments client.
///
/// This struct contains all the necessary configuration for creating an authenticated
/// client that can sign HTTP requests. It includes paths to cryptographic keys and
/// identifiers used in the signing process.
/// 
/// ## Example
///
/// ```rust
/// use open_payments::client::ClientConfig;
/// use std::path::PathBuf;
///
/// let config = ClientConfig {
///     key_id: "my-key-2024".to_string(),
///     private_key_path: PathBuf::from("keys/private.pem"),
///     jwks_path: Some(PathBuf::from("keys/jwks.json")),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub key_id: String,
    
    /// Path to the private key file used for signing HTTP requests.
    ///
    /// The private key should be in PEM format (either in plain text or base64 encoded) and compatible with Ed25519 signing.
    /// If the file doesn't exist, a new key will be generated automatically.
    pub private_key_path: PathBuf,
    
    /// Optional path where the JSON Web Key Set (JWKS) should be saved.
    ///
    /// If provided, the client will automatically generate a JWKS containing the
    /// public key corresponding to the private key and save it to this location.
    ///
    /// ## Usage
    ///
    /// - Set to `Some(path)` to enable automatic JWKS generation
    /// - Set to `None` to disable JWKS generation
    /// - The JWKS file will be created automatically when the client is initialized
    ///
    /// Example: `Some(PathBuf::from("keys/jwks.json"))`
    pub jwks_path: Option<PathBuf>,
}

impl Default for ClientConfig {
    /// Creates a default configuration with reasonable defaults.
    ///
    /// The default configuration uses:
    /// - Empty key ID
    /// - `private.key` as the private key path
    /// - `jwks.json` as the JWKS path
    ///
    /// **Note**: You should typically override the `key_id` with a meaningful value
    /// and consider using more secure paths for production environments.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use open_payments::client::ClientConfig;
    ///
    /// let mut config = ClientConfig::default();
    /// config.key_id = "my-key".to_string();
    /// ```
    fn default() -> Self {
        Self {
            key_id: "".into(),
            private_key_path: PathBuf::from("private.key"),
            jwks_path: Some(PathBuf::from("jwks.json")),
        }
    }
}
