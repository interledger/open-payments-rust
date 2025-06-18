use op_client::AuthenticatedResources;
use op_client::api::UnauthenticatedResources;
use op_types::{
    GrantResponse,
    auth::{
        AccessItem, AccessTokenRequest, GrantRequest, InteractFinish, InteractRequest,
        LimitsOutgoing, OutgoingPaymentAction,
    },
};
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    // Authenticated client can be also used for unauthenticated resources
    let mut client = create_authenticated_client()?;

    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;
    let quote_url = get_env_var("QUOTE_URL")?;
    let gnap_token = get_env_var("QUOTE_ACCESS_TOKEN")?;

    client.access_token = Some(gnap_token);

    let wallet_address = client.wallet_address().get(&wallet_address_url).await?;

    let quote = client.quotes().get(&quote_url).await?;

    let wallet_id = &wallet_address.id;
    let grant_request = GrantRequest {
        access_token: AccessTokenRequest {
            access: vec![AccessItem::OutgoingPayment {
                actions: vec![
                    OutgoingPaymentAction::Read,
                    OutgoingPaymentAction::ReadAll,
                    OutgoingPaymentAction::List,
                    OutgoingPaymentAction::Create,
                ],
                identifier: wallet_id.to_string(),
                limits: Some(LimitsOutgoing {
                    receiver: None,
                    debit_amount: Some(quote.debit_amount),
                    receive_amount: None,
                    interval: None,
                }),
            }],
        },
        client: wallet_id.to_string(),
        interact: Some(InteractRequest {
            start: vec!["redirect".to_string()],
            finish: InteractFinish {
                method: "redirect".to_string(),
                uri: "http://localhost".to_string(),
                nonce: Uuid::new_v4().to_string(),
            },
        }),
    };

    println!(
        "Grant request JSON: {}",
        serde_json::to_string_pretty(&grant_request)?
    );

    let response = client
        .grant()
        .request(&wallet_address.auth_server, &grant_request)
        .await?;

    match response {
        GrantResponse::WithToken { access_token, .. } => {
            println!("Received access token: {:#?}", access_token.value);
            println!(
                "Received access token manage URL: {:#?}",
                access_token.manage
            );
        }
        GrantResponse::WithInteraction {
            interact,
            continue_,
        } => {
            println!("Received interact: {:#?}", interact);
            println!("Received continue: {:#?}", continue_);
        }
    }

    Ok(())
}
