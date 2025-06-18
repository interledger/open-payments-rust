use op_client::client::{AuthenticatedOpenPaymentsClient, UnauthenticatedOpenPaymentsClient};
use op_client::config::ClientConfig;
use op_client::error::{OpClientError, Result};
use std::env;

pub struct TestSetup {
    pub auth_client: AuthenticatedOpenPaymentsClient,
    pub unauth_client: UnauthenticatedOpenPaymentsClient,
    pub resource_server_url: String,
    pub wallet_address: String,
}

impl TestSetup {
    pub async fn new() -> Result<Self> {
        dotenv::from_filename("tests/integration/.env").map_err(|_| {
            OpClientError::Other(".env file not found in tests/integration directory".into())
        })?;

        let resource_server_url = env::var("OPEN_PAYMENTS_SERVER_URL").map_err(|_| {
            OpClientError::Other("OPEN_PAYMENTS_SERVER_URL not set in .env file".into())
        })?;
        let wallet_address = env::var("OPEN_PAYMENTS_WALLET_ADDRESS").map_err(|_| {
            OpClientError::Other("OPEN_PAYMENTS_WALLET_ADDRESS not set in .env file".into())
        })?;
        let key_id = env::var("OPEN_PAYMENTS_KEY_ID").map_err(|_| {
            OpClientError::Other("OPEN_PAYMENTS_KEY_ID not set in .env file".into())
        })?;
        let private_key_path = env::var("OPEN_PAYMENTS_PRIVATE_KEY_PATH").map_err(|_| {
            OpClientError::Other("OPEN_PAYMENTS_PRIVATE_KEY_PATH not set in .env file".into())
        })?;

        let config = ClientConfig {
            key_id,
            private_key_path: private_key_path.into(),
            ..Default::default()
        };

        let auth_client = AuthenticatedOpenPaymentsClient::new(config)?;
        let unauth_client = UnauthenticatedOpenPaymentsClient::new();

        let test_setup = Self {
            auth_client,
            unauth_client,
            resource_server_url,
            wallet_address,
        };

        Ok(test_setup)
    }
}
