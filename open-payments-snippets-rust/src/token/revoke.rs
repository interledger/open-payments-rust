use op_client::AuthenticatedResources;
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("ACCESS_TOKEN")?;
    let token_manage_url = get_env_var("TOKEN_MANAGE_URL")?;

    client
        .token()
        .revoke(&token_manage_url, Some(&gnap_token))
        .await?;

    println!("Access token revoked successfully");
    Ok(())
}
