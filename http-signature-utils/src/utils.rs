use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::SigningKey;
use pem::{parse, encode, Pem};
use pkcs8::{der::Decode, PrivateKeyInfo};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;
use crate::error::{HttpSignatureError, Result};

pub fn load_or_generate_key(path: &Path) -> Result<SigningKey> {
    if path.exists() {
        let key_str = fs::read_to_string(path)?;
        let key_str = key_str.trim();

        let key_str = if let Ok(decoded) = STANDARD.decode(key_str) {
            String::from_utf8(decoded)?
        } else {
            key_str.to_string()
        };

        let pem = parse(&key_str).map_err(|e| HttpSignatureError::Pem(e.to_string()))?;
        if pem.tag() != "PRIVATE KEY" {
            return Err(HttpSignatureError::Pem("Not a PRIVATE KEY".to_string()));
        }

        let private_key_info = PrivateKeyInfo::from_der(pem.contents()).map_err(|e| HttpSignatureError::Pkcs8(e.to_string()))?;
        let raw = private_key_info.private_key;

        let raw_private_key: [u8; 32] = raw[2..].try_into().map_err(|_| HttpSignatureError::InvalidPrivateKeyLength)?;

        let signing_key = SigningKey::from_bytes(&raw_private_key);
        Ok(signing_key)
    } else {
        let mut csprng = OsRng {};
        let signing_key = SigningKey::generate(&mut csprng);
        
        let pem = Pem::new(
            "PRIVATE KEY",
            // We need to prepend the Ed25519 OID and key length
            // See RFC 8410 Section 7 for details
            [
                0x30, 0x2E, // SEQUENCE, length 46
                0x02, 0x01, 0x00, // INTEGER, 1 byte, value 0
                0x30, 0x05, // SEQUENCE, 5 bytes
                0x06, 0x03, 0x2B, 0x65, 0x70, // OID 1.3.101.112 (Ed25519)
                0x04, 0x22, // OCTET STRING, 34 bytes
                0x04, 0x20, // OCTET STRING, 32 bytes (the actual key)
            ].iter()
            .chain(signing_key.to_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
        );
        
        let pem_string = encode(&pem);
        fs::write(path, pem_string)?;
        Ok(signing_key)
    }
} 