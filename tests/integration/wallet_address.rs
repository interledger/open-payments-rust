use crate::integration::common::TestSetup;
use open_payments::client::api::UnauthenticatedResources;

#[tokio::test]
async fn test_get_wallet_address_flows() {
    let test_setup = TestSetup::new().await.expect("Failed to create test setup");

    let wallet_address = test_setup
        .unauth_client
        .wallet_address()
        .get(&test_setup.wallet_address)
        .await
        .expect("Failed to get wallet address");

    assert_eq!(wallet_address.id, test_setup.wallet_address);
    assert!(!wallet_address.auth_server.is_empty());
    assert!(!wallet_address.resource_server.is_empty());

    let jwks = test_setup
        .unauth_client
        .wallet_address()
        .get_keys(&wallet_address)
        .await
        .expect("Failed to get JWKS");

    assert!(!jwks.keys.is_empty());
}
