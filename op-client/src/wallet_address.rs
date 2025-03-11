use anyhow::{Result, Context};
use reqwest::Client;
use log::info;
use op_types::wallet_address::{WalletAddress, JsonWebKeySet, DidDocument};

// Might not be needed here
pub fn pointer_to_url(pointer: &str) -> String {
    if pointer.starts_with('$') {
        format!("https://{}", &pointer[1..])
    } else {
        pointer.to_string()
    }
}

pub async fn get_wallet_address(http_client: &Client, pointer: &str) -> Result<WalletAddress> {
    let url = pointer_to_url(pointer);
    info!("Resolving wallet address from URL: {}", url);
    let response = http_client
        .get(&url)
        .send()
        .await
        .with_context(|| format!("GET request failed for wallet address URL: {}", url))?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to resolve wallet address: HTTP {}", response.status()));
    }
    
    let wallet_address: WalletAddress = response
        .json()
        .await
        .with_context(|| format!("Failed to parse wallet address JSON from {}", url))?;
    
    info!("Successfully resolved wallet address with ID: {}", wallet_address.id);
    Ok(wallet_address)
}

pub async fn get_keys(http_client: &Client, wallet: &WalletAddress) -> Result<JsonWebKeySet> {
    let url = format!("{}/jwks.json", wallet.id.trim_end_matches('/'));
    info!("Fetching JWKS from URL: {}", url);
    let resp = http_client
        .get(&url)
        .send()
        .await
        .with_context(|| format!("GET request failed for JWKS URL: {}", url))?;
    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch JWKS: HTTP {}", resp.status()));
    }
    let jwks: JsonWebKeySet = resp
        .json()
        .await
        .with_context(|| format!("Failed to parse JWKS JSON from {}", url))?;
    info!("Fetched JWKS successfully");
    Ok(jwks)
}

pub async fn get_did_document(http_client: &Client, wallet: &WalletAddress) -> Result<DidDocument> {
    let url = format!("{}/did.json", wallet.id.trim_end_matches('/'));
    info!("Fetching DID document from URL: {}", url);
    let resp = http_client
        .get(&url)
        .send()
        .await
        .with_context(|| format!("GET request failed for DID document URL: {}", url))?;
    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch DID document: HTTP {}", resp.status()));
    }
    let did_doc: DidDocument = resp
        .json()
        .await
        .with_context(|| format!("Failed to parse DID document JSON from {}", url))?;
    info!("Fetched DID document successfully");
    Ok(did_doc)
}
