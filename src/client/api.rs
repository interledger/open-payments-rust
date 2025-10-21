use crate::client::{AuthenticatedOpenPaymentsClient, BaseClient};
use crate::types::{
    AccessTokenResponse, ContinueResponse, GrantRequest, GrantResponse, IncomingPayment,
    IncomingPaymentRequest, JsonWebKeySet, ListIncomingPaymentsResponse,
    ListOutgoingPaymentsResponse, OutgoingPayment, OutgoingPaymentRequest, PublicIncomingPayment,
    Quote, QuoteRequest, WalletAddress,
};
use crate::{
    grant::{cancel_grant, continue_grant, request_grant},
    payments::{
        complete_incoming_payment, create_incoming_payment, create_outgoing_payment,
        get_incoming_payment, get_outgoing_payment, get_public_incoming_payment,
        list_incoming_payments, list_outgoing_payments,
    },
    quotes::{create_quote, get_quote},
    token::{revoke_access_token, rotate_access_token},
    wallet_address::{get_keys, get_wallet_address},
    Result,
};
pub mod authenticated {
    use super::*;

    pub struct QuoteResource<'a> {
        client: &'a AuthenticatedOpenPaymentsClient,
    }

    impl<'a> QuoteResource<'a> {
        pub(crate) fn new(client: &'a AuthenticatedOpenPaymentsClient) -> Self {
            Self { client }
        }

        pub async fn create(
            &self,
            resource_server_url: &str,
            req_body: &QuoteRequest,
            access_token: Option<&str>,
        ) -> Result<Quote> {
            create_quote(self.client, resource_server_url, req_body, access_token).await
        }

        pub async fn get(&self, quote_url: &str, access_token: Option<&str>) -> Result<Quote> {
            get_quote(self.client, quote_url, access_token).await
        }
    }
    pub struct IncomingPaymentResource<'a> {
        client: &'a AuthenticatedOpenPaymentsClient,
    }

    impl<'a> IncomingPaymentResource<'a> {
        pub(crate) fn new(client: &'a AuthenticatedOpenPaymentsClient) -> Self {
            Self { client }
        }

        pub async fn create(
            &self,
            resource_server_url: &str,
            req_body: &IncomingPaymentRequest,
            access_token: Option<&str>,
        ) -> Result<IncomingPayment> {
            create_incoming_payment(self.client, resource_server_url, req_body, access_token).await
        }

        pub async fn get(
            &self,
            payment_url: &str,
            access_token: Option<&str>,
        ) -> Result<IncomingPayment> {
            get_incoming_payment(self.client, payment_url, access_token).await
        }

        pub async fn complete(
            &self,
            payment_url: &str,
            access_token: Option<&str>,
        ) -> Result<IncomingPayment> {
            complete_incoming_payment(self.client, payment_url, access_token).await
        }

        pub async fn list(
            &self,
            resource_server_url: &str,
            wallet_address: &str,
            cursor: Option<&str>,
            first: Option<u32>,
            last: Option<u32>,
            access_token: Option<&str>,
        ) -> Result<ListIncomingPaymentsResponse> {
            list_incoming_payments(
                self.client,
                resource_server_url,
                wallet_address,
                cursor,
                first,
                last,
                access_token,
            )
            .await
        }
    }

    pub struct OutgoingPaymentResource<'a> {
        client: &'a AuthenticatedOpenPaymentsClient,
    }

    impl<'a> OutgoingPaymentResource<'a> {
        pub(crate) fn new(client: &'a AuthenticatedOpenPaymentsClient) -> Self {
            Self { client }
        }

        pub async fn create(
            &self,
            resource_server_url: &str,
            req_body: &OutgoingPaymentRequest,
            access_token: Option<&str>,
        ) -> Result<OutgoingPayment> {
            create_outgoing_payment(self.client, resource_server_url, req_body, access_token).await
        }

        pub async fn list(
            &self,
            resource_server_url: &str,
            wallet_address: &str,
            cursor: Option<&str>,
            first: Option<u32>,
            last: Option<u32>,
            access_token: Option<&str>,
        ) -> Result<ListOutgoingPaymentsResponse> {
            list_outgoing_payments(
                self.client,
                resource_server_url,
                wallet_address,
                cursor,
                first,
                last,
                access_token,
            )
            .await
        }

        pub async fn get(
            &self,
            payment_url: &str,
            access_token: Option<&str>,
        ) -> Result<OutgoingPayment> {
            get_outgoing_payment(self.client, payment_url, access_token).await
        }
    }

    pub struct Grant<'a> {
        client: &'a AuthenticatedOpenPaymentsClient,
    }

    impl<'a> Grant<'a> {
        pub(crate) fn new(client: &'a AuthenticatedOpenPaymentsClient) -> Self {
            Self { client }
        }

        pub async fn request(&self, auth_url: &str, grant: &GrantRequest) -> Result<GrantResponse> {
            request_grant(self.client, auth_url, grant).await
        }

        pub async fn continue_grant(
            &self,
            continue_uri: &str,
            interact_ref: &str,
            access_token: Option<&str>,
        ) -> Result<ContinueResponse> {
            continue_grant(self.client, continue_uri, interact_ref, access_token).await
        }

        pub async fn cancel(&self, continue_uri: &str, access_token: Option<&str>) -> Result<()> {
            cancel_grant(self.client, continue_uri, access_token).await
        }
    }

    pub struct Token<'a> {
        client: &'a AuthenticatedOpenPaymentsClient,
    }

    impl<'a> Token<'a> {
        pub(crate) fn new(client: &'a AuthenticatedOpenPaymentsClient) -> Self {
            Self { client }
        }

        pub async fn rotate(
            &self,
            auth_url: &str,
            access_token: Option<&str>,
        ) -> Result<AccessTokenResponse> {
            rotate_access_token(self.client, auth_url, access_token).await
        }

        pub async fn revoke(&self, auth_url: &str, access_token: Option<&str>) -> Result<()> {
            revoke_access_token(self.client, auth_url, access_token).await
        }
    }
}

pub mod unauthenticated {
    use super::*;

    pub struct WalletAddressResource<'a, C: BaseClient> {
        client: &'a C,
    }

    impl<'a, C: BaseClient> WalletAddressResource<'a, C> {
        pub(crate) fn new(client: &'a C) -> Self {
            Self { client }
        }

        pub async fn get(&self, wallet_address_url: &str) -> Result<WalletAddress> {
            get_wallet_address(self.client.http_client(), wallet_address_url).await
        }

        pub async fn get_keys(&self, wallet: &WalletAddress) -> Result<JsonWebKeySet> {
            get_keys(self.client.http_client(), wallet).await
        }

        pub async fn get_did_document(&self, _wallet: &WalletAddress) -> Result<()> {
            unimplemented!()
        }
    }

    pub struct IncomingPaymentResource<'a, C: BaseClient> {
        client: &'a C,
    }

    impl<'a, C: BaseClient> IncomingPaymentResource<'a, C> {
        pub(crate) fn new(client: &'a C) -> Self {
            Self { client }
        }

        pub async fn get(&self, payment_url: &str) -> Result<PublicIncomingPayment> {
            get_public_incoming_payment(self.client, payment_url).await
        }
    }
}

pub trait AuthenticatedResources {
    fn quotes(&self) -> authenticated::QuoteResource<'_>;
    fn incoming_payments(&self) -> authenticated::IncomingPaymentResource<'_>;
    fn outgoing_payments(&self) -> authenticated::OutgoingPaymentResource<'_>;
    fn grant(&self) -> authenticated::Grant<'_>;
    fn token(&self) -> authenticated::Token<'_>;
}

/// Extension trait for any client (authenticated or not)
pub trait UnauthenticatedResources: BaseClient + Sized {
    fn wallet_address(&self) -> unauthenticated::WalletAddressResource<'_, Self>;
    fn public_incoming_payments(&self) -> unauthenticated::IncomingPaymentResource<'_, Self>;
}

impl AuthenticatedResources for AuthenticatedOpenPaymentsClient {
    fn quotes(&self) -> authenticated::QuoteResource<'_> {
        authenticated::QuoteResource::new(self)
    }

    fn incoming_payments(&self) -> authenticated::IncomingPaymentResource<'_> {
        authenticated::IncomingPaymentResource::new(self)
    }

    fn outgoing_payments(&self) -> authenticated::OutgoingPaymentResource<'_> {
        authenticated::OutgoingPaymentResource::new(self)
    }

    fn grant(&self) -> authenticated::Grant<'_> {
        authenticated::Grant::new(self)
    }

    fn token(&self) -> authenticated::Token<'_> {
        authenticated::Token::new(self)
    }
}

impl<C: BaseClient> UnauthenticatedResources for C {
    fn wallet_address(&self) -> unauthenticated::WalletAddressResource<'_, Self> {
        unauthenticated::WalletAddressResource::new(self)
    }

    fn public_incoming_payments(&self) -> unauthenticated::IncomingPaymentResource<'_, Self> {
        unauthenticated::IncomingPaymentResource::new(self)
    }
}
