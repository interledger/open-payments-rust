use op_client::AuthenticatedResources;
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let mut client = create_authenticated_client()?;

    let gnap_token = get_env_var("ACCESS_TOKEN")?;
    let token_manage_url = get_env_var("TOKEN_MANAGE_URL")?;

    client.access_token = Some(gnap_token);

    let response = client.token().rotate(&token_manage_url).await?;

    println!("Rotated access token: {:#?}", response.access_token);
    Ok(())
}
