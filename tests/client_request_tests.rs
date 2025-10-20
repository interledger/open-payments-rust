use open_payments::client::{
    AuthenticatedClient, AuthenticatedResources, ClientConfig, UnauthenticatedClient,
    UnauthenticatedResources,
};
use open_payments::types::{
    Amount, CreateIncomingPaymentRequest, CreateOutgoingPaymentRequest, CreateQuoteRequest,
    IncomingPayment, PaymentMethodType, PublicIncomingPayment, Receiver, WalletAddress,
};
use tempfile::tempdir;
use url::Url;
use wiremock::matchers::{header, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn dummy_config(base: &str) -> ClientConfig {
    ClientConfig {
        key_id: "test-key".into(),
        private_key_path: std::path::PathBuf::from("tests/private.key"),
        jwks_path: None,
        wallet_address_url: format!("{base}/alice"),
    }
}

#[tokio::test]
async fn unauthenticated_wallet_address_get_builds_request() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    let wallet_url = base.join("alice").unwrap().to_string();
    let auth_url = base.join("auth").unwrap().to_string();
    let wallet = WalletAddress {
        id: wallet_url.clone(),
        public_name: None,
        asset_code: "EUR".into(),
        asset_scale: 2,
        auth_server: auth_url,
        resource_server: server.uri(),
    };

    Mock::given(method("GET"))
        .and(path(base.join("alice").unwrap().path()))
        .respond_with(ResponseTemplate::new(200).set_body_json(&wallet))
        .mount(&server)
        .await;

    let client = UnauthenticatedClient::new();
    let got = client.wallet_address().get(&wallet_url).await.unwrap();
    assert_eq!(got, wallet);
}

#[tokio::test]
async fn authenticated_incoming_payment_create_sets_headers_and_body() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    let response_payment = serde_json::json!({
        "id": base.join("incoming-payments/123").unwrap().to_string(),
        "walletAddress": base.join("alice").unwrap().to_string(),
        "completed": false,
        "receivedAmount": {"value": "0", "assetCode": "EUR", "assetScale": 2},
        "createdAt": "2025-01-01T00:00:00Z",
        "updatedAt": "2025-01-01T00:00:00Z"
    });

    Mock::given(method("POST"))
        .and(path(base.join("incoming-payments").unwrap().path()))
        .and(header("content-type", "application/json"))
        .and(header_exists("Signature"))
        .and(header_exists("Signature-Input"))
        .and(header_exists("Content-Digest"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(response_payment.to_string(), "application/json"),
        )
        .mount(&server)
        .await;

    let tmp = tempdir().unwrap();
    let mut config = dummy_config(&server.uri());
    config.private_key_path = tmp.path().join("private.key");
    let client = AuthenticatedClient::new(config).unwrap();

    let req = CreateIncomingPaymentRequest {
        wallet_address: base.join("alice").unwrap().to_string(),
        incoming_amount: Some(Amount {
            value: "100".into(),
            asset_code: "EUR".into(),
            asset_scale: 2,
        }),
        expires_at: None,
        metadata: None,
    };

    let _ = client
        .incoming_payments()
        .create(&server.uri(), &req, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn authenticated_quote_create_and_get() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    let created_quote = serde_json::json!({
        "id": base.join("quotes/q1").unwrap().to_string(),
        "walletAddress": base.join("alice").unwrap().to_string(),
        "receiver": base.join("incoming-payments/123").unwrap().to_string(),
        "receiveAmount": {"value": "10", "assetCode": "EUR", "assetScale": 2},
        "debitAmount": {"value": "110", "assetCode": "EUR", "assetScale": 2},
        "method": "ilp",
        "createdAt": "2025-01-01T00:00:00Z"
    });
    Mock::given(method("POST"))
        .and(path(base.join("quotes").unwrap().path()))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(created_quote.to_string(), "application/json"),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(base.join("quotes/q1").unwrap().path()))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(created_quote.to_string(), "application/json"),
        )
        .mount(&server)
        .await;

    let tmp = tempdir().unwrap();
    let mut config = dummy_config(&server.uri());
    config.private_key_path = tmp.path().join("private.key");
    let client = AuthenticatedClient::new(config).unwrap();

    let req = CreateQuoteRequest::FixedReceiveAmountQuote {
        wallet_address: base.join("alice").unwrap().to_string(),
        receiver: Receiver(base.join("incoming-payments/123").unwrap().to_string()),
        method: PaymentMethodType::Ilp,
        receive_amount: Amount {
            value: "10".into(),
            asset_code: "EUR".into(),
            asset_scale: 2,
        },
    };
    let q = client
        .quotes()
        .create(&server.uri(), &req, Some("tok"))
        .await
        .unwrap();
    assert_eq!(q.id, base.join("quotes/q1").unwrap().as_ref());

    let q2 = client
        .quotes()
        .get(base.join("quotes/q1").unwrap().as_ref(), Some("tok"))
        .await
        .unwrap();
    assert_eq!(q2, q);
}

#[tokio::test]
async fn authenticated_outgoing_payment_create_from_quote() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    let created_payment = serde_json::json!({
        "id": base.join("outgoing-payments/op1").unwrap().to_string(),
        "walletAddress": base.join("alice").unwrap().to_string(),
        "quoteId": base.join("quotes/q1").unwrap().to_string(),
        "failed": false,
        "receiver": base.join("incoming-payments/123").unwrap().to_string(),
        "receiveAmount": {"value": "10", "assetCode": "EUR", "assetScale": 2},
        "debitAmount": {"value": "110", "assetCode": "EUR", "assetScale": 2},
        "sentAmount": {"value": "0", "assetCode": "EUR", "assetScale": 2},
        "grantSpentDebitAmount": {"value": "0", "assetCode": "EUR", "assetScale": 2},
        "grantSpentReceiveAmount": {"value": "0", "assetCode": "EUR", "assetScale": 2},
        "createdAt": "2025-01-01T00:00:00Z"
    });
    Mock::given(method("POST"))
        .and(path(base.join("outgoing-payments").unwrap().path()))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(created_payment.to_string(), "application/json"),
        )
        .mount(&server)
        .await;

    let tmp = tempdir().unwrap();
    let mut config = dummy_config(&server.uri());
    config.private_key_path = tmp.path().join("private.key");
    let client = AuthenticatedClient::new(config).unwrap();

    let req = CreateOutgoingPaymentRequest::FromQuote {
        wallet_address: base.join("alice").unwrap().to_string(),
        quote_id: base.join("quotes/q1").unwrap().to_string(),
        metadata: None,
    };
    let p = client
        .outgoing_payments()
        .create(&server.uri(), &req, Some("tok"))
        .await
        .unwrap();
    assert_eq!(
        p.id,
        base.join("outgoing-payments/op1").unwrap().to_string()
    );
}

#[tokio::test]
async fn error_propagates_http_status_and_message() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    Mock::given(method("GET"))
        .and(path(base.join("public-payment").unwrap().path()))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = UnauthenticatedClient::new();
    let res: Result<PublicIncomingPayment, _> = client
        .public_incoming_payments()
        .get(base.join("public-payment").unwrap().as_ref())
        .await;
    assert!(res.is_err());
}

#[tokio::test]
async fn error_includes_status_code_and_reason() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    Mock::given(method("GET"))
        .and(path(base.join("missing").unwrap().path()))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = UnauthenticatedClient::new();
    let res: Result<PublicIncomingPayment, _> = client
        .public_incoming_payments()
        .get(base.join("missing").unwrap().as_ref())
        .await;
    let err = res.err().expect("expected error");
    assert_eq!(err.description, "HTTP request failed");
    assert_eq!(err.code, Some(404));
    assert_eq!(err.status.as_deref(), Some("Not Found"));
}

#[tokio::test]
async fn json_decode_error_maps_to_client_error_without_status() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    // Return invalid JSON with 200 OK
    Mock::given(method("GET"))
        .and(path(base.join("alice").unwrap().path()))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let client = UnauthenticatedClient::new();
    let res = client
        .wallet_address()
        .get(base.join("alice").unwrap().as_ref())
        .await;
    let err = res.err().expect("expected error");
    assert!(err.description.starts_with("HTTP error:"));
    assert!(err.code.is_none());
    assert!(err.status.is_none());
}

#[tokio::test]
async fn header_parse_error_with_invalid_token() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();

    let tmp = tempdir().unwrap();
    let mut config = dummy_config(&server.uri());
    config.private_key_path = tmp.path().join("private.key");
    let client = AuthenticatedClient::new(config).unwrap();

    // Use an invalid header value (contains newline) to force parse failure
    let res: Result<IncomingPayment, _> = client
        .incoming_payments()
        .get(
            base.join("incoming-payments/p1").unwrap().as_ref(),
            Some("bad\ntoken"),
        )
        .await;
    let err = res.err().expect("expected error");
    assert!(err.description.starts_with("Header parse error:"));
}

#[tokio::test]
async fn revoke_token_204_no_content_succeeds() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    Mock::given(method("DELETE"))
        .and(path(base.join("token/revoke").unwrap().path()))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let tmp = tempdir().unwrap();
    let mut config = dummy_config(&server.uri());
    config.private_key_path = tmp.path().join("private.key");
    let client = AuthenticatedClient::new(config).unwrap();

    let res = client
        .token()
        .revoke(base.join("token/revoke").unwrap().as_ref(), Some("token"))
        .await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn cancel_grant_204_no_content_succeeds() {
    let server = MockServer::start().await;

    let base = Url::parse(&server.uri()).unwrap();
    Mock::given(method("DELETE"))
        .and(path(base.join("continue/123").unwrap().path()))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let tmp = tempdir().unwrap();
    let mut config = dummy_config(&server.uri());
    config.private_key_path = tmp.path().join("private.key");
    let client = AuthenticatedClient::new(config).unwrap();

    let res = client
        .grant()
        .cancel(base.join("continue/123").unwrap().as_ref(), Some("token"))
        .await;
    assert!(res.is_ok());
}
