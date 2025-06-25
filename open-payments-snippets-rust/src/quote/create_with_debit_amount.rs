use op_client::api::AuthenticatedResources;
use op_client::utils::get_resource_server_url;
use op_types::{Amount, PaymentMethodType, Receiver, resource::CreateQuoteRequest};
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("QUOTE_ACCESS_TOKEN")?;
    let incoming_payment_url = get_env_var("INCOMING_PAYMENT_URL")?;
    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;
    let resource_server_url = get_resource_server_url(&wallet_address_url)?;

    let request = CreateQuoteRequest::FixedSendAmountQuote {
        wallet_address: wallet_address_url,
        receiver: Receiver(incoming_payment_url),
        method: PaymentMethodType::Ilp,
        debit_amount: Amount {
            value: "1000".to_string(),
            asset_code: "EUR".to_string(),
            asset_scale: 2u8,
        },
    };

    println!(
        "Quote create request JSON: {}",
        serde_json::to_string_pretty(&request)?
    );

    let quote = client
        .quotes()
        .create(&resource_server_url, &request, Some(&gnap_token))
        .await?;

    println!("Created quote: {:#?}", quote);
    Ok(())
}
