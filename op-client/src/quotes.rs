use crate::OpClientError;
use crate::Result;
use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::{Quote, QuoteRequest};
use crate::utils::join_url_paths;
use reqwest::Method;

pub(crate) async fn create_quote(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    req_body: &QuoteRequest,
    access_token: Option<&str>,
) -> Result<Quote> {
    let url = join_url_paths(resource_server_url, "quotes")?;
    let body = serde_json::to_string(req_body).map_err(OpClientError::Serde)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn get_quote(
    client: &AuthenticatedOpenPaymentsClient,
    quote_url: &str,
    access_token: Option<&str>,
) -> Result<Quote> {
    AuthenticatedRequest::new(client, Method::GET, quote_url.to_string())
        .build_and_execute(access_token)
        .await
}
