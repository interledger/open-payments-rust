use crate::request::UnauthenticatedRequest;
use anyhow::Result;
use op_types::wallet_address::{DidDocument, JsonWebKeySet, WalletAddress};
use reqwest::{Client, Method};

pub(crate) async fn get_wallet_address(
    http_client: &Client,
    wallet_address_url: &str,
) -> Result<WalletAddress> {
    UnauthenticatedRequest::new(http_client.clone(), Method::GET, wallet_address_url.into())
        .execute()
        .await
}

pub(crate) async fn get_keys(
    http_client: &Client,
    wallet: &WalletAddress,
) -> Result<JsonWebKeySet> {
    let url = format!("{}/jwks.json", wallet.id.trim_end_matches('/'));

    UnauthenticatedRequest::new(http_client.clone(), Method::GET, url)
        .execute()
        .await
}

#[deprecated(
    since = "0.1.0",
    note = "This method is not implemented yet but preserved for compatibility"
)]
pub(crate) async fn get_did_document(
    http_client: &Client,
    wallet: &WalletAddress,
) -> Result<DidDocument> {
    let url = format!("{}/did.json", wallet.id.trim_end_matches('/'));

    UnauthenticatedRequest::new(http_client.clone(), Method::GET, url)
        .execute()
        .await
}
