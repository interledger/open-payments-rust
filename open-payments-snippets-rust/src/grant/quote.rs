use op_client::AuthenticatedResources;
use op_client::api::UnauthenticatedResources;
use op_types::{
    GrantResponse,
    auth::{AccessItem, AccessTokenRequest, GrantRequest, QuoteAction},
};
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    // Authenticated client can be also used for unauthenticated resources
    let client = create_authenticated_client()?;

    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;
    let wallet_address = client.wallet_address().get(&wallet_address_url).await?;

    let grant_request = GrantRequest {
        access_token: AccessTokenRequest {
            access: vec![AccessItem::Quote {
                actions: vec![QuoteAction::Create, QuoteAction::Read, QuoteAction::ReadAll],
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
            unreachable!("Interaction not required for quotes");
        }
    }

    Ok(())
}
