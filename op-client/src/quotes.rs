use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::quotes::QuoteRequest;
use anyhow::Result;
use op_types::resource::Quote;
use reqwest::Method;

pub(crate) async fn create_quote(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    req_body: &QuoteRequest,
) -> Result<Quote> {
    let url = format!("{}/quotes", resource_server_url.trim_end_matches('/'));
    let body = serde_json::to_string(req_body)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .execute()
        .await
}

pub(crate) async fn get_quote(
    client: &AuthenticatedOpenPaymentsClient,
    quote_url: &str,
) -> Result<Quote> {
    AuthenticatedRequest::new(client, Method::GET, quote_url.to_string())
        .execute()
        .await
}
