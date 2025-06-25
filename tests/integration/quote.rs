use crate::integration::common::TestSetup;
use open_payments::client::{AuthenticatedResources, UnauthenticatedResources};
use open_payments::types::{
    AccessItem, AccessTokenRequest, Amount, GrantRequest, GrantResponse, IncomingPaymentAction,
    IncomingPaymentRequest, PaymentMethodType, QuoteAction, QuoteRequest, Receiver,
};

async fn get_access_token(test_setup: &mut TestSetup) -> String {
    let wallet_address = test_setup
        .auth_client
        .wallet_address()
        .get(&test_setup.wallet_address)
        .await
        .expect("Failed to get wallet address");

    let grant_request = GrantRequest {
        access_token: AccessTokenRequest {
            access: vec![
                AccessItem::IncomingPayment {
                    actions: vec![IncomingPaymentAction::Create, IncomingPaymentAction::Read],
                    identifier: None,
                },
                AccessItem::Quote {
                    actions: vec![QuoteAction::Create, QuoteAction::Read],
                },
            ],
        },
        client: wallet_address.id,
        interact: None,
    };

    let response = test_setup
        .auth_client
        .grant()
        .request(&wallet_address.auth_server, &grant_request)
        .await
        .expect("Failed to request grant");

    let access_token = match response {
        GrantResponse::WithToken { access_token, .. } => access_token,
        GrantResponse::WithInteraction { .. } => {
            panic!("Unexpected interaction required for grant request");
        }
    };

    access_token.value
}

async fn create_incoming_payment(test_setup: &TestSetup, access_token: &str) -> String {
    let request = IncomingPaymentRequest {
        wallet_address: test_setup.wallet_address.clone(),
        incoming_amount: Some(Amount {
            value: "100".to_string(),
            asset_code: "EUR".to_string(),
            asset_scale: 2,
        }),
        metadata: None,
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
    };

    let incoming_payment = test_setup
        .auth_client
        .incoming_payments()
        .create(
            &test_setup.resource_server_url,
            &request,
            Some(&access_token),
        )
        .await
        .expect("Failed to create incoming payment");

    incoming_payment.id
}

#[tokio::test]
async fn test_quote_flows() {
    let mut test_setup = TestSetup::new().await.expect("Failed to create test setup");
    let access_token = get_access_token(&mut test_setup).await;

    // Create an incoming payment to use as the receiver
    let incoming_payment_url = create_incoming_payment(&test_setup, &access_token).await;

    // Test quote with no amount
    let request = QuoteRequest::NoAmountQuote {
        wallet_address: test_setup.wallet_address.clone(),
        receiver: Receiver(incoming_payment_url.clone()),
        method: PaymentMethodType::Ilp,
    };

    let quote = test_setup
        .auth_client
        .quotes()
        .create(
            &test_setup.resource_server_url,
            &request,
            Some(&access_token),
        )
        .await
        .expect("Failed to create quote");

    assert_eq!(quote.wallet_address, test_setup.wallet_address);

    // Test quote with fixed send/debit amount
    let request = QuoteRequest::FixedSendAmountQuote {
        wallet_address: test_setup.wallet_address.clone(),
        receiver: Receiver(incoming_payment_url.clone()),
        method: PaymentMethodType::Ilp,
        debit_amount: Amount {
            value: "100".to_string(),
            asset_code: "EUR".to_string(),
            asset_scale: 2,
        },
    };

    let quote = test_setup
        .auth_client
        .quotes()
        .create(
            &test_setup.resource_server_url,
            &request,
            Some(&access_token),
        )
        .await
        .expect("Failed to create quote");

    assert_eq!(quote.wallet_address, test_setup.wallet_address);
    assert_eq!(quote.debit_amount.value, "100");

    let retrieved_quote = test_setup
        .auth_client
        .quotes()
        .get(&quote.id, Some(&access_token))
        .await
        .expect("Failed to get quote");

    assert_eq!(retrieved_quote.id, quote.id);
    assert_eq!(retrieved_quote.wallet_address, test_setup.wallet_address);

    // Test quote with fixed receive amount
    let request = QuoteRequest::FixedReceiveAmountQuote {
        wallet_address: test_setup.wallet_address.clone(),
        receiver: Receiver(incoming_payment_url),
        method: PaymentMethodType::Ilp,
        receive_amount: Amount {
            value: "100".to_string(),
            asset_code: "EUR".to_string(),
            asset_scale: 2,
        },
    };

    let quote = test_setup
        .auth_client
        .quotes()
        .create(
            &test_setup.resource_server_url,
            &request,
            Some(&access_token),
        )
        .await
        .expect("Failed to create quote");

    assert_eq!(quote.wallet_address, test_setup.wallet_address);
    assert_eq!(quote.receive_amount.value, "100");
} 