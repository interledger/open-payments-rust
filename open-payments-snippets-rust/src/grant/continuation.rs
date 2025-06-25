use op_client::api::AuthenticatedResources;
use op_types::ContinueResponse;
use open_payments_snippets_rust::utils::{
    create_authenticated_client, get_env_var, init_logging, load_env,
};

#[tokio::main]
async fn main() -> op_client::Result<()> {
    init_logging();
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("CONTINUE_ACCESS_TOKEN")?;
    let continue_uri = get_env_var("CONTINUE_URI")?;
    let interact_ref = get_env_var("INTERACT_REF")?;

    let response = client
        .grant()
        .continue_grant(&continue_uri, &interact_ref, Some(&gnap_token))
        .await?;

    match response {
        ContinueResponse::WithToken { access_token, .. } => {
            println!("Received access token: {:#?}", access_token.value);
            println!(
                "Received access token manage URL: {:#?}",
                access_token.manage
            );
        }
        ContinueResponse::Pending { .. } => {
            println!("Pending");
        }
    }
    Ok(())
}
