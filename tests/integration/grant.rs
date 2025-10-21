use crate::integration::common::TestSetup;
use open_payments::{
    client::{AuthenticatedResources, UnauthenticatedResources},
    types::{AccessItem, AccessTokenRequest, GrantRequest, GrantResponse, IncomingPaymentAction},
};

#[tokio::test]
async fn test_grant_flows() {
    let test_setup = TestSetup::new().await.expect("Failed to create test setup");

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

    let continue_ = match response {
        GrantResponse::WithToken {
            access_token,
            continue_,
        } => {
            assert!(!access_token.value.is_empty());
            assert!(!access_token.manage.is_empty());
            assert!(!continue_.uri.is_empty());
            assert!(!continue_.access_token.value.is_empty());
            continue_
        }
        GrantResponse::WithInteraction { .. } => {
            panic!("Unexpected interaction required for incoming payments");
        }
    };

    test_setup
        .auth_client
        .grant()
        .cancel(&continue_.uri, Some(&continue_.access_token.value))
        .await
        .expect("Failed to cancel grant");
}
