use anyhow::{Context, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};
use pem::parse;
use pkcs8::{der::Decode, PrivateKeyInfo};
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

    pub fn validate(&self) -> Result<(), JwkError> {
        if self.crv != "Ed25519" || self.kty != "OKP" || self.x.is_empty() {
            return Err(JwkError::InvalidKeyType);
        }
        Ok(())
    }

    //TODO Move to utils?
    pub fn load_or_generate_key(path: &std::path::Path) -> Result<SigningKey> {
        if path.exists() {
            let key_str = std::fs::read_to_string(path)?;

            let pem = parse(key_str).expect("Invalid PEM");
            assert_eq!(pem.tag(), "PRIVATE KEY");

            let private_key_info = PrivateKeyInfo::from_der(&pem.contents()).expect("Invalid DER");

            let raw = private_key_info.private_key;

            // The private key field inside contains the 32-byte Ed25519 seed
            let raw_private_key: [u8; 32] =
                raw[2..].try_into().expect("Invalid private key length");

            let signing_key = SigningKey::from_bytes(&raw_private_key);
            Ok(signing_key)
        } else {
            let mut csprng = OsRng {};
            let signing_key = SigningKey::generate(&mut csprng);
            fs::write(path, signing_key.to_bytes())
                .with_context(|| format!("Writing new key to {:?}", path))?;
            Ok(signing_key)
        }
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
        fs::write(jwks_path, jwks_json)
            .with_context(|| format!("Writing JWKS to {:?}", jwks_path))?;
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
