use op_client::api::AuthenticatedResources;
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("OUTGOING_PAYMENT_ACCESS_TOKEN")?;
    let outgoing_payment_url = get_env_var("OUTGOING_PAYMENT_URL")?;

    let payment = client
        .outgoing_payments()
        .get(&outgoing_payment_url, Some(&gnap_token))
        .await?;

    println!("Outgoing payment: {:#?}", payment);
    Ok(())
}
