use crate::types::common::{Amount, Receiver};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IncomingPayment {
    pub id: String,
    pub wallet_address: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_amount: Option<Amount>,
    pub received_amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IncomingPaymentWithMethods {
    #[serde(flatten)]
    pub payment: IncomingPayment,
    pub methods: Vec<PaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PaymentMethod {
    #[serde(rename = "ilp")]
    Ilp {
        #[serde(rename = "ilpAddress")]
        ilp_address: String,
        #[serde(rename = "sharedSecret")]
        shared_secret: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicIncomingPayment {
    pub received_amount: Amount,
    pub auth_server: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateIncomingPaymentRequest {
    pub wallet_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingPayment {
    pub id: String,
    pub wallet_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    pub failed: bool,
    pub receiver: Receiver,
    pub receive_amount: Amount,
    pub debit_amount: Amount,
    pub sent_amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CreateQuoteRequest {
    NoAmountQuote {
        #[serde(rename = "walletAddress")]
        wallet_address: String,
        receiver: Receiver,
        method: PaymentMethodType,
    },
    FixedReceiveAmountQuote {
        #[serde(rename = "walletAddress")]
        wallet_address: String,
        receiver: Receiver,
        method: PaymentMethodType,
        #[serde(rename = "receiveAmount")]
        receive_amount: Amount,
    },
    FixedSendAmountQuote {
        #[serde(rename = "walletAddress")]
        wallet_address: String,
        receiver: Receiver,
        method: PaymentMethodType,
        #[serde(rename = "debitAmount")]
        debit_amount: Amount,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CreateOutgoingPaymentRequest {
    FromQuote {
        #[serde(rename = "walletAddress")]
        wallet_address: String,
        #[serde(rename = "quoteId")]
        quote_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<Value>,
    },
    FromIncomingPayment {
        #[serde(rename = "walletAddress")]
        wallet_address: String,
        #[serde(rename = "incomingPayment")]
        incoming_payment_id: String,
        #[serde(rename = "debitAmount")]
        debit_amount: Amount,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub id: String,
    pub wallet_address: String,
    pub receiver: Receiver,
    pub receive_amount: Amount,
    pub debit_amount: Amount,
    pub method: PaymentMethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethodType {
    Ilp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaginatedResponse<T> {
    pub pagination: PageInfo,
    pub result: Vec<T>,
}

pub type ListIncomingPaymentsResponse = PaginatedResponse<IncomingPayment>;
pub type ListOutgoingPaymentsResponse = PaginatedResponse<OutgoingPayment>;