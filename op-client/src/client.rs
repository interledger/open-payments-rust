use crate::config::ClientConfig;
use anyhow::{Context, Result};
use ed25519_dalek::SigningKey;
use http_signature_utils::jwk::Jwk;
use log::info;
use reqwest::Client as ReqwestClient;

pub trait BaseClient {
    fn http_client(&self) -> &ReqwestClient;
}

pub struct AuthenticatedOpenPaymentsClient {
    pub http_client: ReqwestClient,
    pub config: ClientConfig,
    pub signing_key: SigningKey,
    pub access_token: Option<String>,
}

impl BaseClient for AuthenticatedOpenPaymentsClient {
    fn http_client(&self) -> &ReqwestClient {
        &self.http_client
    }
}

impl AuthenticatedOpenPaymentsClient {
    pub async fn new(config: ClientConfig) -> Result<Self> {
        let http_client = ReqwestClient::new();

        let signing_key = Jwk::load_or_generate_key(&config.private_key_path)
            .with_context(|| "Failed to load or generate signing key")?;

        if let Some(ref jwks_path) = config.jwks_path {
            let jwks_json = Jwk::generate_jwks_json(&signing_key, &config.key_id);
            Jwk::save_jwks(&jwks_json, jwks_path).with_context(|| "Failed to save JWKS")?;
            info!("JWKS saved to {:?}", jwks_path);
        }

        Ok(Self {
            http_client,
            config,
            signing_key,
            access_token: None,
        })
    }
}

pub struct UnauthenticatedOpenPaymentsClient {
    pub http_client: ReqwestClient,
}

impl UnauthenticatedOpenPaymentsClient {
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

pub use self::AuthenticatedOpenPaymentsClient as AuthenticatedClient;
pub use self::UnauthenticatedOpenPaymentsClient as UnauthenticatedClient;
