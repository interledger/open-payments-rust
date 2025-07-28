use open_payments::client::api::UnauthenticatedResources;
use open_payments::snippets::utils::{create_unauthenticated_client, get_env_var, load_env};

#[tokio::main]
async fn main() -> open_payments::client::Result<()> {
    load_env()?;

    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;

    let client = create_unauthenticated_client();

    let wallet_address = client.wallet_address().get(&wallet_address_url).await?;

    println!("Retrieved wallet address: {wallet_address:#?}");
    Ok(())
}
