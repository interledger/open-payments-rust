use op_client::api::AuthenticatedResources;
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let mut client = create_authenticated_client()?;

    let gnap_token = get_env_var("INCOMING_PAYMENT_ACCESS_TOKEN")?;
    let incoming_payment_url = get_env_var("INCOMING_PAYMENT_URL")?;

    client.access_token = Some(gnap_token);

    let payment = client
        .incoming_payments()
        .get(&incoming_payment_url)
        .await?;

    println!("Incoming payment: {:#?}", payment);
    Ok(())
}
