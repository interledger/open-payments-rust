use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WalletAddress {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,
    pub asset_code: String,
    pub asset_scale: u8,
    pub auth_server: String,
    pub resource_server: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonWebKeySet {
    pub keys: Vec<JsonWebKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonWebKey {
    pub kid: String,
    pub alg: JwkAlgorithm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_: Option<JwkUse>,
    pub kty: JwkKeyType,
    pub crv: JwkCurve,
    pub x: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JwkAlgorithm {
    EdDSA,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JwkUse {
    #[serde(rename = "sig")]
    Signature,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JwkKeyType {
    OKP,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JwkCurve {
    Ed25519,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub controller: String,
    pub public_key_jwk: JsonWebKey,
}
