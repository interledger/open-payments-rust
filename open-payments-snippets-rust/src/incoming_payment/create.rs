use chrono::{Duration, Utc};
use op_client::OpClientError;
use op_client::api::AuthenticatedResources;
use op_client::utils::get_resource_server_url;
use op_types::{Amount, resource::CreateIncomingPaymentRequest};
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};
use serde_json;

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env().map_err(|e| OpClientError::Other(e.to_string()))?;

    let mut client = create_authenticated_client()?;

    let gnap_token = get_env_var("INCOMING_PAYMENT_ACCESS_TOKEN")?;
    client.access_token = Some(gnap_token);

    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;
    let resource_server_url = get_resource_server_url(&wallet_address_url)?;

    // Remove unwrap
    let expires_at = Utc::now()
        .checked_add_signed(Duration::minutes(10000))
        .unwrap();

    let request = CreateIncomingPaymentRequest {
        wallet_address: wallet_address_url,
        incoming_amount: Some(Amount {
            value: 1000,
            asset_code: "EUR".to_string(),
            asset_scale: 2u8,
        }),
        expires_at: Some(expires_at),
        metadata: None,
    };

    println!(
        "Incoming payment create request JSON: {}",
        serde_json::to_string_pretty(&request)?
    );

    let payment = client
        .incoming_payments()
        .create(&resource_server_url, &request)
        .await?;

    println!("Created incoming payment: {:#?}", payment);
    Ok(())
}
