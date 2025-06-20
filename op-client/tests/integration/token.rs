use crate::integration::common::TestSetup;
use op_client::{
    AuthenticatedResources, UnauthenticatedResources,
    types::{
        AccessItem, AccessToken, AccessTokenRequest, GrantRequest, GrantResponse,
        IncomingPaymentAction,
    },
};

async fn get_access_token(test_setup: &mut TestSetup) -> AccessToken {
    let wallet_address = test_setup
        .auth_client
        .wallet_address()
        .get(&test_setup.wallet_address)
        .await
        .expect("Failed to get wallet address");

    let grant_request = GrantRequest {
        access_token: AccessTokenRequest {
            access: vec![AccessItem::IncomingPayment {
                actions: vec![IncomingPaymentAction::Create, IncomingPaymentAction::Read],
                identifier: None,
            }],
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

    match response {
        GrantResponse::WithToken { access_token, .. } => access_token,
        GrantResponse::WithInteraction { .. } => {
            panic!("Unexpected interaction required for grant request");
        }
    }
}

#[tokio::test]
async fn test_token_flows() {
    let mut test_setup = TestSetup::new().await.expect("Failed to create test setup");

    let initial_token = get_access_token(&mut test_setup).await;
    test_setup.auth_client.access_token = Some(initial_token.value.clone());

    let rotated_token_response = test_setup
        .auth_client
        .token()
        .rotate(&initial_token.manage)
        .await
        .expect("Failed to rotate token");

    let rotated_token = rotated_token_response.access_token;
    assert!(!rotated_token.value.is_empty());
    assert!(!rotated_token.manage.is_empty());
    assert_ne!(rotated_token.value, initial_token.value);

    test_setup.auth_client.access_token = Some(rotated_token.value);

    test_setup
        .auth_client
        .token()
        .revoke(&rotated_token.manage)
        .await
        .expect("Failed to revoke token");
}
