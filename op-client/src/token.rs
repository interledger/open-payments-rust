use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use anyhow::Result;
use op_types::auth::AccessToken;
use reqwest::Method;

//TODO get token from the client instead of passing it as a parameter
pub(crate) async fn rotate_access_token(
    client: &AuthenticatedOpenPaymentsClient,
    auth_url: &str,
    token: &str,
) -> Result<AccessToken> {
    let url = format!("{}/token/{}", auth_url.trim_end_matches('/'), token);

    AuthenticatedRequest::new(client, Method::POST, url)
        .execute()
        .await
}

pub(crate) async fn revoke_access_token(
    client: &AuthenticatedOpenPaymentsClient,
    auth_url: &str,
    token: &str,
) -> Result<()> {
    let url = format!("{}/token/{}", auth_url.trim_end_matches('/'), token);

    AuthenticatedRequest::new(client, Method::DELETE, url)
        .execute()
        .await
}
