use op_types::resource::Amount;
use serde::Serialize;

pub mod quotes {
    use super::*;

    #[derive(Serialize)]
    pub struct QuoteRequest {
        pub receiver: Option<String>,
        #[serde(rename = "debitAmount")]
        pub debit_amount: Option<Amount>,
        #[serde(rename = "receiveAmount")]
        pub receive_amount: Option<Amount>,
    }
}

pub mod payments {
    use super::*;

    #[derive(Serialize)]
    pub struct IncomingPaymentRequest {
        #[serde(rename = "walletAddress")]
        pub wallet_address: String,
        #[serde(rename = "incomingAmount")]
        pub incoming_amount: Option<Amount>,
        #[serde(rename = "expiresAt")]
        pub expires_at: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metadata: Option<serde_json::Value>,
    }

    #[derive(Serialize)]
    pub struct OutgoingPaymentRequest {
        #[serde(rename = "quoteId")]
        pub quote_id: String,
        pub description: Option<String>,
    }
}

pub mod grant {
    use super::*;
    use op_types::auth::AccessToken;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct AccessTokenRequest {
        pub access: op_types::auth::Access,
    }

    #[derive(Serialize)]
    pub struct GrantRequest {
        pub access_token: AccessTokenRequest,
        pub client: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub interact: Option<Interact>,
    }

    #[derive(serde::Deserialize)]
    pub struct GrantResponse {
        pub access_token: AccessToken,
        //TODO Add the rest of the fields for continuation
    }

    #[derive(Serialize)]
    pub struct PaymentLimits {
        #[serde(rename = "debitAmount")]
        pub debit_amount: Option<Amount>,
        #[serde(rename = "receiveAmount")]
        pub receive_amount: Option<Amount>,
    }

    #[derive(Serialize)]
    pub struct AccessRequestItem {
        #[serde(rename = "type")]
        pub res_type: String,
        pub actions: Vec<String>,
        pub identifier: Option<String>,
        pub limits: Option<PaymentLimits>,
    }

    #[derive(Serialize)]
    pub struct Interact {
        pub start: Vec<String>,
        pub finish: FinishInteract,
    }

    #[derive(Serialize)]
    pub struct FinishInteract {
        pub method: String,
        pub uri: String,
        pub nonce: String,
    }

    #[derive(Serialize)]
    pub struct ContinueRequest<'a> {
        pub interact_ref: &'a str,
    }
}
