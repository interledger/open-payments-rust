use open_payments::client::ClientConfig;
use open_payments::client::{AuthenticatedClient, UnauthenticatedClient};
use open_payments::client::{OpClientError, Result};
use open_payments::utils;
use std::env;

pub struct TestSetup {
    pub auth_client: AuthenticatedClient,
    pub unauth_client: UnauthenticatedClient,
    pub resource_server_url: String,
    pub wallet_address: String,
    pub test_wallet_email: Option<String>,
    pub test_wallet_password: Option<String>,
}

impl TestSetup {
    pub async fn new() -> Result<Self> {
        dotenv::from_filename("tests/integration/.env").map_err(|_| {
            OpClientError::other(".env file not found in tests/integration directory".to_string())
        })?;

        let wallet_address = env::var("OPEN_PAYMENTS_WALLET_ADDRESS").map_err(|_| {
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
        let resource_server_url = utils::get_resource_server_url(&wallet_address)?;

        let config = ClientConfig {
            key_id,
            private_key_path: private_key_path.into(),
            wallet_address_url: wallet_address.clone(),
            ..Default::default()
        };

        let auth_client = AuthenticatedClient::new(config)?;
        let unauth_client = UnauthenticatedClient::new();

        let test_setup = Self {
            auth_client,
            unauth_client,
            resource_server_url,
            wallet_address,
            test_wallet_email,
            test_wallet_password,
        };

        Ok(test_setup)
    }
}
