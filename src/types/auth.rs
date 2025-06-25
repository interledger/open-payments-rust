use crate::types::common::{Amount, Interval, Receiver};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AccessItem {
    #[serde(rename = "incoming-payment")]
    IncomingPayment {
        actions: Vec<IncomingPaymentAction>,
        #[serde(skip_serializing_if = "Option::is_none")]
        identifier: Option<String>,
    },
    #[serde(rename = "outgoing-payment")]
    OutgoingPayment {
        actions: Vec<OutgoingPaymentAction>,
        identifier: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        limits: Option<LimitsOutgoing>,
    },
    #[serde(rename = "quote")]
    Quote { actions: Vec<QuoteAction> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum IncomingPaymentAction {
    Create,
    Complete,
    Read,
    ReadAll,
    List,
    ListAll,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OutgoingPaymentAction {
    Create,
    Read,
    ReadAll,
    List,
    ListAll,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum QuoteAction {
    Create,
    Read,
    ReadAll,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LimitsOutgoing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<Receiver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessToken {
    pub value: String,
    pub manage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<AccessItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessTokenResponse {
    pub access_token: AccessToken,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GrantRequest {
    pub access_token: AccessTokenRequest,
    pub client: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact: Option<InteractRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessTokenRequest {
    pub access: Vec<AccessItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InteractRequest {
    pub start: Vec<String>,
    pub finish: Option<InteractFinish>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InteractFinish {
    pub method: String,
    pub uri: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InteractResponse {
    pub redirect: String,
    pub finish: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Continue {
    pub access_token: ContinueAccessToken,
    pub uri: String,
    pub wait: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinueAccessToken {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum GrantResponse {
    WithInteraction {
        interact: InteractResponse,
        #[serde(rename = "continue")]
        continue_: Continue,
    },
    WithToken {
        access_token: AccessToken,
        #[serde(rename = "continue")]
        continue_: Continue,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinueRequest {
    pub interact_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ContinueResponse {
    WithToken {
        access_token: AccessToken,
        #[serde(rename = "continue")]
        continue_: Continue,
    },
    Pending {
        #[serde(rename = "continue")]
        continue_: Continue,
    },
}