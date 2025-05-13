use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::grant::{ContinueRequest, GrantRequest, GrantResponse};
use anyhow::Result;
use op_types::auth::AccessToken;
use reqwest::Method;

pub(crate) async fn request_grant(
    client: &AuthenticatedOpenPaymentsClient,
    auth_url: &str,
    grant: &GrantRequest,
) -> Result<AccessToken> {
    let url = format!("{}/", auth_url.trim_end_matches('/'));
    let body = serde_json::to_string(grant)?;

    let grant_response: GrantResponse = AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .execute()
        .await?;

    Ok(grant_response.access_token)
}

pub(crate) async fn continue_grant(
    client: &AuthenticatedOpenPaymentsClient,
    continue_uri: &str,
    interact_ref: &str,
) -> Result<AccessToken> {
    let body = serde_json::to_string(&ContinueRequest { interact_ref })?;

    AuthenticatedRequest::new(client, Method::POST, continue_uri.to_string())
        .with_body(body)
        .execute()
        .await
}

pub(crate) async fn revoke_grant(
    client: &AuthenticatedOpenPaymentsClient,
    revoke_url: &str,
) -> Result<()> {
    AuthenticatedRequest::new(client, Method::DELETE, revoke_url.to_string())
        .execute()
        .await
}
