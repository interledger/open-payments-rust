use crate::OpClientError;
use crate::Result;
use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::{
    IncomingPayment, IncomingPaymentRequest, ListIncomingPaymentsResponse,
    ListOutgoingPaymentsResponse, OutgoingPayment, OutgoingPaymentRequest,
};
use reqwest::Method;

pub(crate) async fn create_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    req_body: &IncomingPaymentRequest,
) -> Result<IncomingPayment> {
    let url = format!(
        "{}/incoming-payments",
        resource_server_url.trim_end_matches('/')
    );
    let body = serde_json::to_string(req_body).map_err(OpClientError::Serde)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .build_and_execute()
        .await
}

pub(crate) async fn get_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
) -> Result<IncomingPayment> {
    AuthenticatedRequest::new(client, Method::GET, payment_url.to_string())
        .build_and_execute()
        .await
}

pub(crate) async fn complete_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
) -> Result<IncomingPayment> {
    AuthenticatedRequest::new(client, Method::POST, format!("{}/complete", payment_url))
        .build_and_execute()
        .await
}

pub(crate) async fn list_incoming_payments(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    wallet_address: &str,
    cursor: Option<&str>,
    first: Option<u32>,
    last: Option<u32>,
) -> Result<ListIncomingPaymentsResponse> {
    let mut url = format!(
        "{}/incoming-payments?wallet-address={}",
        resource_server_url.trim_end_matches('/'),
        wallet_address
    );

    if let Some(cursor) = cursor {
        url.push_str(&format!("&cursor={}", cursor));
    }
    if let Some(first) = first {
        url.push_str(&format!("&first={}", first));
    }
    if let Some(last) = last {
        url.push_str(&format!("&last={}", last));
    }

    AuthenticatedRequest::new(client, Method::GET, url)
        .build_and_execute()
        .await
}

pub(crate) async fn create_outgoing_payment(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    req_body: &OutgoingPaymentRequest,
) -> Result<OutgoingPayment> {
    let url = format!(
        "{}/outgoing-payments",
        resource_server_url.trim_end_matches('/')
    );
    let body = serde_json::to_string(req_body).map_err(OpClientError::Serde)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .build_and_execute()
        .await
}

pub(crate) async fn get_outgoing_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
) -> Result<OutgoingPayment> {
    AuthenticatedRequest::new(client, Method::GET, payment_url.to_string())
        .build_and_execute()
        .await
}

pub(crate) async fn list_outgoing_payments(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
    wallet_address: &str,
    cursor: Option<&str>,
    first: Option<u32>,
    last: Option<u32>,
) -> Result<ListOutgoingPaymentsResponse> {
    let mut url = format!(
        "{}/outgoing-payments?wallet-address={}",
        resource_server_url.trim_end_matches('/'),
        wallet_address
    );

    if let Some(cursor) = cursor {
        url.push_str(&format!("&cursor={}", cursor));
    }
    if let Some(first) = first {
        url.push_str(&format!("&first={}", first));
    }
    if let Some(last) = last {
        url.push_str(&format!("&last={}", last));
    }

    AuthenticatedRequest::new(client, Method::GET, url)
        .build_and_execute()
        .await
}
