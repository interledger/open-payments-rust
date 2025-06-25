use open_payments::client::api::UnauthenticatedResources;
use open_payments::client::AuthenticatedResources;
use open_payments::snippets::utils::{create_authenticated_client, get_env_var, load_env};
use open_payments::types::auth::{
    AccessItem, AccessTokenRequest, GrantRequest, GrantResponse, IncomingPaymentAction,
};

#[tokio::main]
async fn main() -> open_payments::client::Result<()> {
    load_env()?;

    // Authenticated client can be also used for unauthenticated resources
    let client = create_authenticated_client()?;

    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;
    let wallet_address = client.wallet_address().get(&wallet_address_url).await?;

    let grant_request = GrantRequest {
        access_token: AccessTokenRequest {
            access: vec![AccessItem::IncomingPayment {
                actions: vec![
                    IncomingPaymentAction::Create,
                    IncomingPaymentAction::Read,
                    IncomingPaymentAction::ReadAll,
                    IncomingPaymentAction::List,
                    IncomingPaymentAction::Complete,
                ],
                identifier: None,
            }],
        },
        client: wallet_address.id,
        interact: None,
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
        GrantResponse::WithInteraction { .. } => {
            unreachable!("Interaction not required for incoming payments");
        }
    }

    Ok(())
}
