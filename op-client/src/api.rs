use crate::client::{AuthenticatedOpenPaymentsClient, BaseClient};
use crate::types::{
    grant::GrantRequest,
    payments::{IncomingPaymentRequest, OutgoingPaymentRequest},
    quotes::QuoteRequest,
};
use crate::{
    grant::{continue_grant, request_grant, revoke_grant},
    payments::{
        complete_incoming_payment, create_incoming_payment, create_outgoing_payment,
        get_incoming_payment, get_outgoing_payment, list_incoming_payments, list_outgoing_payments,
    },
    quotes::{create_quote, get_quote},
    token::{revoke_access_token, rotate_access_token},
    wallet_address::{get_did_document, get_keys, get_wallet_address},
};
use anyhow::Result;
use op_types::auth::AccessToken;
use op_types::resource::{IncomingPayment, OutgoingPayment, Quote};
use op_types::wallet_address::{DidDocument, JsonWebKeySet, WalletAddress};

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
        ) -> Result<Quote> {
            create_quote(self.client, resource_server_url, req_body).await
        }

        pub async fn get(&self, quote_url: &str) -> Result<Quote> {
            get_quote(self.client, quote_url).await
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
        ) -> Result<IncomingPayment> {
            create_incoming_payment(self.client, resource_server_url, req_body).await
        }

        pub async fn get(&self, payment_url: &str) -> Result<IncomingPayment> {
            get_incoming_payment(self.client, payment_url).await
        }

        pub async fn complete(&self, payment_url: &str) -> Result<IncomingPayment> {
            complete_incoming_payment(self.client, payment_url).await
        }

        pub async fn list(&self, resource_server_url: &str) -> Result<Vec<IncomingPayment>> {
            list_incoming_payments(self.client, resource_server_url).await
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
        ) -> Result<OutgoingPayment> {
            create_outgoing_payment(self.client, resource_server_url, req_body).await
        }

        pub async fn list(&self, resource_server_url: &str) -> Result<Vec<OutgoingPayment>> {
            list_outgoing_payments(self.client, resource_server_url).await
        }

        pub async fn get(&self, payment_url: &str) -> Result<OutgoingPayment> {
            get_outgoing_payment(self.client, payment_url).await
        }
    }

    pub struct Grant<'a> {
        client: &'a AuthenticatedOpenPaymentsClient,
    }

    impl<'a> Grant<'a> {
        pub(crate) fn new(client: &'a AuthenticatedOpenPaymentsClient) -> Self {
            Self { client }
        }

        //TODO Move to unauthenticated client
        pub async fn request(&self, auth_url: &str, grant: &GrantRequest) -> Result<AccessToken> {
            request_grant(self.client, auth_url, grant).await
        }

        pub async fn continue_grant(
            &self,
            continue_uri: &str,
            interact_ref: &str,
        ) -> Result<AccessToken> {
            continue_grant(self.client, continue_uri, interact_ref).await
        }

        pub async fn revoke(&self, revoke_url: &str) -> Result<()> {
            revoke_grant(self.client, revoke_url).await
        }
    }

    pub struct Token<'a> {
        client: &'a AuthenticatedOpenPaymentsClient,
    }

    impl<'a> Token<'a> {
        pub(crate) fn new(client: &'a AuthenticatedOpenPaymentsClient) -> Self {
            Self { client }
        }

        pub async fn rotate(&self, auth_url: &str, token: &str) -> Result<AccessToken> {
            rotate_access_token(self.client, auth_url, token).await
        }

        pub async fn revoke(&self, auth_url: &str, token: &str) -> Result<()> {
            revoke_access_token(self.client, auth_url, token).await
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

        #[deprecated(
            since = "0.1.0",
            note = "This method is not implemented yet but preserved for compatibility"
        )]
        pub async fn get_did_document(&self, wallet: &WalletAddress) -> Result<DidDocument> {
            get_did_document(self.client.http_client(), wallet).await
        }
    }
}

pub trait AuthenticatedResources {
    fn quotes(&self) -> authenticated::QuoteResource;
    fn incoming_payments(&self) -> authenticated::IncomingPaymentResource;
    fn outgoing_payments(&self) -> authenticated::OutgoingPaymentResource;
    fn grant(&self) -> authenticated::Grant;
    fn token(&self) -> authenticated::Token;
}

/// Extension trait for any client (authenticated or not)
pub trait UnauthenticatedResources: BaseClient + Sized {
    fn wallet_address(&self) -> unauthenticated::WalletAddressResource<Self>;
}

impl AuthenticatedResources for AuthenticatedOpenPaymentsClient {
    fn quotes(&self) -> authenticated::QuoteResource {
        authenticated::QuoteResource::new(self)
    }

    fn incoming_payments(&self) -> authenticated::IncomingPaymentResource {
        authenticated::IncomingPaymentResource::new(self)
    }

    fn outgoing_payments(&self) -> authenticated::OutgoingPaymentResource {
        authenticated::OutgoingPaymentResource::new(self)
    }

    fn grant(&self) -> authenticated::Grant {
        authenticated::Grant::new(self)
    }

    fn token(&self) -> authenticated::Token {
        authenticated::Token::new(self)
    }
}

impl<C: BaseClient> UnauthenticatedResources for C {
    fn wallet_address(&self) -> unauthenticated::WalletAddressResource<Self> {
        unauthenticated::WalletAddressResource::new(self)
    }
}
