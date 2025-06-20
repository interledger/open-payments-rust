use crate::Result;
use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::AccessTokenResponse;
use reqwest::Method;

pub(crate) async fn rotate_access_token(
    client: &AuthenticatedOpenPaymentsClient,
    token_manage_url: &str,
) -> Result<AccessTokenResponse> {
    AuthenticatedRequest::new(client, Method::POST, token_manage_url.to_string())
        .build_and_execute()
        .await
}

pub(crate) async fn revoke_access_token(
    client: &AuthenticatedOpenPaymentsClient,
    token_manage_url: &str,
) -> Result<()> {
    AuthenticatedRequest::new(client, Method::DELETE, token_manage_url.to_string())
        .build_and_execute()
        .await
}
