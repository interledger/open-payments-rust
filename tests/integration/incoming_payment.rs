use crate::integration::common::TestSetup;
use open_payments::client::{AuthenticatedResources, UnauthenticatedResources};
use open_payments::types::{
    AccessItem, AccessTokenRequest, Amount, GrantRequest, GrantResponse, IncomingPaymentAction,
    IncomingPaymentRequest,
};

async fn get_access_token(test_setup: &mut TestSetup) -> String {
    let wallet_address = test_setup
        .auth_client
        .wallet_address()
        .get(&test_setup.wallet_address)
        .await
        .expect("Failed to get wallet address");

    let grant_request = GrantRequest::new(
        AccessTokenRequest {
            access: vec![AccessItem::IncomingPayment {
                actions: vec![
                    IncomingPaymentAction::Create,
                    IncomingPaymentAction::Read,
                    IncomingPaymentAction::ReadAll,
                    IncomingPaymentAction::List,
                    IncomingPaymentAction::Complete,
                ],
                identifier: None,
            }],
        },
        None,
    );

    let response = test_setup
        .auth_client
        .grant()
        .request(&wallet_address.auth_server, &grant_request)
        .await
        .expect("Failed to request grant");

    let access_token = match response {
        GrantResponse::WithToken { access_token, .. } => access_token,
        GrantResponse::WithInteraction { .. } => {
            panic!("Unexpected interaction required for incoming payments");
        }
    };

    access_token.value
}

#[tokio::test]
async fn test_incoming_payment_flows() {
    let mut test_setup = TestSetup::new().await.expect("Failed to create test setup");

    let access_token = get_access_token(&mut test_setup).await;

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

    assert_eq!(incoming_payment.wallet_address, test_setup.wallet_address);
    assert_eq!(
        incoming_payment.incoming_amount.as_ref().unwrap().value,
        "100"
    );

    let retrieved_payment = test_setup
        .auth_client
        .incoming_payments()
        .get(&incoming_payment.id, Some(&access_token))
        .await
        .expect("Failed to get incoming payment");

    assert_eq!(retrieved_payment.id, incoming_payment.id);
    assert_eq!(retrieved_payment.wallet_address, test_setup.wallet_address);

    let list_response = test_setup
        .auth_client
        .incoming_payments()
        .list(
            &test_setup.resource_server_url,
            &test_setup.wallet_address,
            None,
            Some(10),
            None,
            Some(&access_token),
        )
        .await
        .expect("Failed to list incoming payments");

    assert!(!list_response.result.is_empty());
    assert!(list_response
        .result
        .iter()
        .any(|p| p.id == incoming_payment.id));

    let complete_response = test_setup
        .auth_client
        .incoming_payments()
        .complete(&incoming_payment.id, Some(&access_token))
        .await
        .expect("Failed to complete incoming payment");

    // `retrieved_payment`` should have `completed` set to false
    assert_eq!(complete_response.completed, !retrieved_payment.completed);
}
