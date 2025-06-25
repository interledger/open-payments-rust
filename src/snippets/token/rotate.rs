use open_payments::client::AuthenticatedResources;
use open_payments::snippets::utils::{create_authenticated_client, get_env_var, load_env};

#[tokio::main]
async fn main() -> open_payments::client::Result<()> {
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("ACCESS_TOKEN")?;
    let token_manage_url = get_env_var("TOKEN_MANAGE_URL")?;

    let response = client
        .token()
        .rotate(&token_manage_url, Some(&gnap_token))
        .await?;

    println!("Rotated access token: {:#?}", response.access_token);
    Ok(())
}
