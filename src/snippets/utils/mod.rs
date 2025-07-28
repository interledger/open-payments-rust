use crate::client::{
    AuthenticatedClient, ClientConfig, OpClientError, Result, UnauthenticatedClient,
};
use crate::http_signature::jwk::Jwk;
use dotenv::dotenv;
use std::{env, path::PathBuf};

pub fn load_env() -> Result<()> {
    dotenv().ok();
    Ok(())
}

pub fn get_env_var(key: &str) -> Result<String> {
    env::var(key).map_err(|_| OpClientError::Other(format!("{key} environment variable not set")))
}

pub fn create_authenticated_client() -> Result<AuthenticatedClient> {
    let private_key_path = PathBuf::from(get_env_var("PRIVATE_KEY_PATH")?);
    let key_id = get_env_var("KEY_ID")?;
    let key_id_clone = key_id.clone();
    let jwks_path = get_env_var("JWKS_PATH").map(PathBuf::from).ok();

    let config = ClientConfig {
        private_key_path,
        key_id,
        jwks_path,
    };

    let client = AuthenticatedClient::new(config)
        .map_err(|e| OpClientError::Other(format!("Client creation error: {e}")))?;
    Jwk::new(key_id_clone, Some(&client.signing_key))
        .map_err(|e| OpClientError::Other(format!("JWK error: {e}")))?;
    Ok(client)
}

pub fn create_unauthenticated_client() -> UnauthenticatedClient {
    UnauthenticatedClient::new()
}
