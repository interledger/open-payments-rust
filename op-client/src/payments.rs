use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::payments::{IncomingPaymentRequest, OutgoingPaymentRequest};
use anyhow::Result;
use op_types::resource::{IncomingPayment, OutgoingPayment};
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
    let body = serde_json::to_string(req_body)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .execute()
        .await
}

pub(crate) async fn list_incoming_payments(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
) -> Result<Vec<IncomingPayment>> {
    let url = format!(
        "{}/incoming-payments",
        resource_server_url.trim_end_matches('/')
    );

    AuthenticatedRequest::new(client, Method::GET, url)
        .execute()
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
    let body = serde_json::to_string(req_body)?;

    AuthenticatedRequest::new(client, Method::POST, url)
        .with_body(body)
        .execute()
        .await
}

pub(crate) async fn list_outgoing_payments(
    client: &AuthenticatedOpenPaymentsClient,
    resource_server_url: &str,
) -> Result<Vec<OutgoingPayment>> {
    let url = format!(
        "{}/outgoing-payments",
        resource_server_url.trim_end_matches('/')
    );
    AuthenticatedRequest::new(client, Method::GET, url)
        .execute()
        .await
}

pub(crate) async fn get_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
) -> Result<IncomingPayment> {
    AuthenticatedRequest::new(client, Method::GET, payment_url.to_string())
        .execute()
        .await
}

pub(crate) async fn complete_incoming_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
) -> Result<IncomingPayment> {
    AuthenticatedRequest::new(client, Method::POST, format!("{}/complete", payment_url))
        .execute()
        .await
}

pub(crate) async fn get_outgoing_payment(
    client: &AuthenticatedOpenPaymentsClient,
    payment_url: &str,
) -> Result<OutgoingPayment> {
    AuthenticatedRequest::new(client, Method::GET, payment_url.to_string())
        .execute()
        .await
}
