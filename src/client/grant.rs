use crate::OpClientError;
use crate::Result;
use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::{ContinueRequest, ContinueResponse, GrantRequest, GrantResponse};
use reqwest::Method;

pub(crate) async fn request_grant(
    client: &AuthenticatedOpenPaymentsClient,
    auth_url: &str,
    grant: &GrantRequest,
) -> Result<GrantResponse> {
    let body = serde_json::to_string(grant).map_err(OpClientError::Serde)?;

    AuthenticatedRequest::new(client, Method::POST, auth_url.to_string())
        .with_body(body)
        .build_and_execute(None)
        .await
}

pub(crate) async fn continue_grant(
    client: &AuthenticatedOpenPaymentsClient,
    continue_uri: &str,
    interact_ref: &str,
    access_token: Option<&str>,
) -> Result<ContinueResponse> {
    let body = serde_json::to_string(&ContinueRequest {
        interact_ref: Some(interact_ref.to_string()),
    })
    .map_err(OpClientError::Serde)?;

    AuthenticatedRequest::new(client, Method::POST, continue_uri.to_string())
        .with_body(body)
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn cancel_grant(
    client: &AuthenticatedOpenPaymentsClient,
    continue_uri: &str,
    access_token: Option<&str>,
) -> Result<()> {
    AuthenticatedRequest::new(client, Method::DELETE, continue_uri.to_string())
        .build_and_execute(access_token)
        .await
}
