use op_client::api::AuthenticatedResources;
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("QUOTE_ACCESS_TOKEN")?;
    let quote_url = get_env_var("QUOTE_URL")?;

    let quote = client.quotes().get(&quote_url, Some(&gnap_token)).await?;

    println!("Quote: {:#?}", quote);
    Ok(())
}
