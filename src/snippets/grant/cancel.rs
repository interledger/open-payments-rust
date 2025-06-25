use open_payments::client::AuthenticatedResources;
use open_payments::snippets::utils::{
    create_authenticated_client, get_env_var, load_env,
};

#[tokio::main]
async fn main() -> open_payments::client::Result<()> {
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("CONTINUE_ACCESS_TOKEN")?;
    let continue_uri = get_env_var("CONTINUE_URI")?;

    client
        .grant()
        .cancel(&continue_uri, Some(&gnap_token))
        .await?;

    println!("Grant cancelled successfully");
    Ok(())
}
