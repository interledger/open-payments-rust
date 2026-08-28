use crate::types::common::{Amount, Interval, Receiver};
use crate::types::wallet_address::JsonWebKey;
use serde::{Deserialize, Serialize};

/// Client identification for grant requests.
///
/// Open Payments accepts either a wallet address string (backwards compatible),
/// a `{ "walletAddress": "..." }` object, or a directed-identity `{ "jwk": ... }` object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Client {
    /// Deprecated string form of the client wallet address.
    WalletAddressUrl(String),
    /// Object form with a wallet address.
    WalletAddress {
        #[serde(rename = "walletAddress")]
        wallet_address: String,
    },
    /// Directed identity — public key embedded in the grant request.
    Jwk { jwk: JsonWebKey },
}

/// Subject information requested or returned in a grant.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subject {
    pub sub_ids: Vec<SubjectIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubjectIdentifier {
    pub id: String,
    pub format: SubjectIdentifierFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubjectIdentifierFormat {
    Uri,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<AccessTokenRequest>,
    pub(crate) client: Client,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact: Option<InteractRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Subject>,
}

impl GrantRequest {
    /// Creates a grant request with the given access token request.
    ///
    /// The `client` field is filled by [`AuthenticatedResources::grant`] when the
    /// request is sent. Optionally attach [`Subject`] via [`GrantRequest::with_subject`].
    pub fn new(access_token: AccessTokenRequest, interact: Option<InteractRequest>) -> Self {
        Self {
            access_token: Some(access_token),
            client: Client::WalletAddressUrl(String::new()),
            interact,
            subject: None,
        }
    }

    /// Creates a grant request that only asks for subject information.
    pub fn subject_only(subject: Subject, interact: Option<InteractRequest>) -> Self {
        Self {
            access_token: None,
            client: Client::WalletAddressUrl(String::new()),
            interact,
            subject: Some(subject),
        }
    }

    /// Attaches subject information to this grant request.
    pub fn with_subject(mut self, subject: Subject) -> Self {
        self.subject = Some(subject);
        self
    }
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        subject: Option<Subject>,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        subject: Option<Subject>,
    },
    WithSubject {
        subject: Subject,
        #[serde(rename = "continue")]
        continue_: Continue,
    },
    Pending {
        #[serde(rename = "continue")]
        continue_: Continue,
    },
}

impl ContinueResponse {
    pub fn has_access_token(&self) -> bool {
        matches!(self, Self::WithToken { .. })
    }

    pub fn has_subject(&self) -> bool {
        match self {
            Self::WithSubject { .. } => true,
            Self::WithToken { subject, .. } => subject.is_some(),
            Self::Pending { .. } => false,
        }
    }
}
