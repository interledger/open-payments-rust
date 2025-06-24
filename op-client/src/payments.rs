use crate::OpClientError;
use crate::Result;
use crate::client::{AuthenticatedOpenPaymentsClient, BaseClient};
use crate::request::{AuthenticatedRequest, UnauthenticatedRequest};
use crate::types::{
    IncomingPayment, IncomingPaymentRequest, ListIncomingPaymentsResponse,
    ListOutgoingPaymentsResponse, OutgoingPayment, OutgoingPaymentRequest,
};
use crate::utils::join_url_paths;
use op_types::resource::PublicIncomingPayment;
use reqwest::Method;
use url::Url;

pub(crate) async fn create_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    req_body: &IncomingPaymentRequest,
    access_token: Option<&str>,
) -> Result<IncomingPayment> {
    let url = join_url_paths(resource_server_url, "incoming-payments")?;
    let body = serde_json::to_string(req_body).map_err(OpClientError::Serde)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn get_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
    access_token: Option<&str>,
) -> Result<IncomingPayment> {
    AuthenticatedRequest::new(client, Method::GET, payment_url.to_string())
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn complete_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
    access_token: Option<&str>,
) -> Result<IncomingPayment> {
    println!("Input payment URL: {}", payment_url);
    let url = join_url_paths(payment_url, "complete")?;

    println!("Completing payment at URL: {}", url);

    AuthenticatedRequest::new(client, Method::POST, url)
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn list_incoming_payments(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    wallet_address: &str,
    cursor: Option<&str>,
    first: Option<u32>,
    last: Option<u32>,
    access_token: Option<&str>,
) -> Result<ListIncomingPaymentsResponse> {
    let mut url = Url::parse(&join_url_paths(resource_server_url, "incoming-payments")?)?;

    url.query_pairs_mut()
        .append_pair("wallet-address", wallet_address);

    if let Some(cursor) = cursor {
        url.query_pairs_mut().append_pair("cursor", cursor);
    }
    if let Some(first) = first {
        url.query_pairs_mut()
            .append_pair("first", &first.to_string());
    }
    if let Some(last) = last {
        url.query_pairs_mut().append_pair("last", &last.to_string());
    }

    AuthenticatedRequest::new(client, Method::GET, url.to_string())
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn create_outgoing_payment(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    req_body: &OutgoingPaymentRequest,
    access_token: Option<&str>,
) -> Result<OutgoingPayment> {
    let url = join_url_paths(resource_server_url, "outgoing-payments")?;
    let body = serde_json::to_string(req_body).map_err(OpClientError::Serde)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn get_outgoing_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
    access_token: Option<&str>,
) -> Result<OutgoingPayment> {
    AuthenticatedRequest::new(client, Method::GET, payment_url.to_string())
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn list_outgoing_payments(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    wallet_address: &str,
    cursor: Option<&str>,
    first: Option<u32>,
    last: Option<u32>,
    access_token: Option<&str>,
) -> Result<ListOutgoingPaymentsResponse> {
    let mut url = Url::parse(&join_url_paths(resource_server_url, "outgoing-payments")?)?;

    url.query_pairs_mut()
        .append_pair("wallet-address", wallet_address);

    if let Some(cursor) = cursor {
        url.query_pairs_mut().append_pair("cursor", cursor);
    }
    if let Some(first) = first {
        url.query_pairs_mut()
            .append_pair("first", &first.to_string());
    }
    if let Some(last) = last {
        url.query_pairs_mut().append_pair("last", &last.to_string());
    }

    AuthenticatedRequest::new(client, Method::GET, url.to_string())
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn get_public_incoming_payment<C: BaseClient>(
    client: &C,
    payment_url: &str,
) -> Result<PublicIncomingPayment> {
    UnauthenticatedRequest::new(client.http_client(), Method::GET, payment_url.to_string())
        .build_and_execute()
        .await
}
