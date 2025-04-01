use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
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
    pub fn new(key_id: String, private_key: Option<&SigningKey>) -> Result<Self, JwkError> {
        if key_id.trim().is_empty() {
            return Err(JwkError::EmptyKeyId);
        }

        let signing_key = if let Some(priv_key) = private_key {
            priv_key
        } else {
            &SigningKey::generate(&mut OsRng)
        };

        let public_key = signing_key.verifying_key();
        let x = STANDARD.encode(public_key.to_bytes());

        Ok(Self {
            alg: "EdDSA".to_string(),
            kid: key_id,
            use_: Some("sig".to_string()),
            kty: "OKP".to_string(),
            crv: "Ed25519".to_string(),
            x,
        })
    }

    pub fn validate(&self) -> Result<(), JwkError> {
        if self.crv != "Ed25519" || self.kty != "OKP" || self.x.is_empty() {
            return Err(JwkError::InvalidKeyType);
        }
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
        assert!(matches!(result, Err(JwkError::EmptyKeyId)));
    }
}
