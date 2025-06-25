use crate::error::{HttpSignatureError, Result};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwkError {
    #[error("KeyId cannot be empty")]
    EmptyKeyId,
    #[error("Key is not EdDSA-Ed25519")]
    InvalidKeyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub alg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,
    pub kty: String,
    pub crv: String,
    pub x: String,
}

impl Jwk {
    pub fn new(key_id: String, private_key: Option<&SigningKey>) -> Result<Self> {
        if key_id.trim().is_empty() {
            return Err(HttpSignatureError::Jwk(JwkError::EmptyKeyId.to_string()));
        }

        let signing_key = if let Some(priv_key) = private_key {
            priv_key
        } else {
            &SigningKey::generate(&mut OsRng)
        };

        let public_key = signing_key.verifying_key();
        let x = URL_SAFE_NO_PAD.encode(public_key.to_bytes());

        Ok(Self {
            alg: "EdDSA".to_string(),
            kid: key_id,
            use_: Some("sig".to_string()),
            kty: "OKP".to_string(),
            crv: "Ed25519".to_string(),
            x,
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.crv != "Ed25519" || self.kty != "OKP" || self.x.is_empty() {
            return Err(HttpSignatureError::Jwk(
                JwkError::InvalidKeyType.to_string(),
            ));
        }
        Ok(())
    }

    pub fn generate_jwks_json(signing_key: &SigningKey, key_id: &str) -> String {
        let verifying_key: VerifyingKey = signing_key.verifying_key();
        let pub_bytes = verifying_key.as_bytes();
        let x = URL_SAFE_NO_PAD.encode(pub_bytes);
        let jwk = json!({
            "kty": "OKP",
            "crv": "Ed25519",
            "alg": "EdDSA",
            "use": "sig",
            "kid": key_id,
            "x": x
        });
        let jwks = json!({ "keys": [ jwk ] });
        jwks.to_string()
    }

    pub fn save_jwks(jwks_json: &str, jwks_path: &Path) -> Result<()> {
        fs::write(jwks_path, jwks_json).map_err(HttpSignatureError::from)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwk_generation() {
        let jwk = Jwk::new("test-key".to_string(), None).unwrap();
        assert_eq!(jwk.kid, "test-key");
        assert_eq!(jwk.alg, "EdDSA");
        assert_eq!(jwk.kty, "OKP");
        assert_eq!(jwk.crv, "Ed25519");
        assert!(!jwk.x.is_empty());
    }

    #[test]
    fn test_empty_key_id() {
        let result = Jwk::new("".to_string(), None);
        assert!(matches!(result, Err(HttpSignatureError::Jwk(_))));
    }
}
