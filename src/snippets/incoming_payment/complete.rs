use open_payments::client::api::AuthenticatedResources;
use open_payments::snippets::utils::{
    create_authenticated_client, get_env_var, load_env,
};

#[tokio::main]
async fn main() -> open_payments::client::Result<()> {
    load_env()?;

    let gnap_token = get_env_var("INCOMING_PAYMENT_ACCESS_TOKEN")?;
    let incoming_payment_url = get_env_var("INCOMING_PAYMENT_URL")?;

    let client = create_authenticated_client()?;

    let payment = client
        .incoming_payments()
        .complete(&incoming_payment_url, Some(&gnap_token))
        .await?;

    println!("Completed incoming payment: {:#?}", payment);
    Ok(())
}
