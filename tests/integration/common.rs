use open_payments::client::ClientConfig;
use open_payments::client::UnauthenticatedResources;
use open_payments::client::{AuthenticatedClient, UnauthenticatedClient};
use open_payments::client::UnauthenticatedResources;
use open_payments::client::{OpClientError, Result};
use std::env;

pub struct TestSetup {
    pub auth_client: AuthenticatedClient,
    pub unauth_client: UnauthenticatedClient,
    pub resource_server_url: String,
    pub wallet_address: String,
    pub asset_code: String,
    pub asset_scale: u8,
    pub test_wallet_email: Option<String>,
    pub test_wallet_password: Option<String>,
}

impl TestSetup {
    pub async fn new() -> Result<Self> {
        dotenv::from_filename("tests/integration/.env").map_err(|_| {
            OpClientError::other(".env file not found in tests/integration directory".to_string())
        })?;

        let wallet_address_url = env::var("OPEN_PAYMENTS_WALLET_ADDRESS").map_err(|_| {
            OpClientError::other("OPEN_PAYMENTS_WALLET_ADDRESS not set in .env file".to_string())
        })?;
        let key_id = env::var("OPEN_PAYMENTS_KEY_ID").map_err(|_| {
            OpClientError::other("OPEN_PAYMENTS_KEY_ID not set in .env file".to_string())
        })?;
        let private_key_path = env::var("OPEN_PAYMENTS_PRIVATE_KEY_PATH").map_err(|_| {
            OpClientError::other("OPEN_PAYMENTS_PRIVATE_KEY_PATH not set in .env file".to_string())
        })?;
        let test_wallet_email = env::var("TEST_WALLET_EMAIL").ok();
        let test_wallet_password = env::var("TEST_WALLET_PASSWORD").ok();

        let unauth_client = UnauthenticatedClient::new();
        // Use the wallet address document's resourceServer — do not derive it by
        // stripping path segments from the wallet URL (those hosts can differ).
        let wallet = unauth_client
            .wallet_address()
            .get(&wallet_address_url)
            .await?;

        let config = ClientConfig {
            key_id,
            private_key_path: private_key_path.into(),
            wallet_address_url: wallet.id.clone(),
            ..Default::default()
        };

        let auth_client = AuthenticatedClient::new(config)?;

        Ok(Self {
            auth_client,
            unauth_client,
            resource_server_url: wallet.resource_server,
            wallet_address: wallet.id,
            asset_code: wallet.asset_code,
            asset_scale: wallet.asset_scale,
            test_wallet_email,
            test_wallet_password,
        })
    }
}
