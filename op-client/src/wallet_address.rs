use crate::Result;
use crate::request::UnauthenticatedRequest;
use crate::types::{JsonWebKeySet, WalletAddress};
use reqwest::{Client, Method};

pub(crate) async fn get_wallet_address(
    client: &Client,
    wallet_address_url: &str,
) -> Result<WalletAddress> {
    UnauthenticatedRequest::new(client, Method::GET, wallet_address_url.into())
        .build_and_execute()
        .await
}

pub(crate) async fn get_keys(client: &Client, wallet: &WalletAddress) -> Result<JsonWebKeySet> {
    let url = format!("{}/jwks.json", wallet.id.trim_end_matches('/'));

    UnauthenticatedRequest::new(client, Method::GET, url)
        .build_and_execute()
        .await
}
