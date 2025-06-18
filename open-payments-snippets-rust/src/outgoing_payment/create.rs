use op_client::api::AuthenticatedResources;
use op_client::{types::OutgoingPaymentRequest, utils::get_resource_server_url};
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};
use serde_json;

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let mut client = create_authenticated_client()?;

    let gnap_token = get_env_var("OUTGOING_PAYMENT_ACCESS_TOKEN")?;
    let quote_url = get_env_var("QUOTE_URL")?;
    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;
    let resource_server_url = get_resource_server_url(&wallet_address_url)?;

    client.access_token = Some(gnap_token);

    let request = OutgoingPaymentRequest::FromQuote {
        wallet_address: wallet_address_url,
        quote_id: quote_url,
        metadata: None,
    };

    println!(
        "Outgoing payment create request JSON: {}",
        serde_json::to_string_pretty(&request)?
    );

    let payment = client
        .outgoing_payments()
        .create(&resource_server_url, &request)
        .await?;

    println!("Created outgoing payment: {:#?}", payment);
    Ok(())
}
