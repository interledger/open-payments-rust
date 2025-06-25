use crate::http_signature::{HttpSignatureError, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use ed25519_dalek::SigningKey;
use pem::{Pem, encode, parse};
use pkcs8::{PrivateKeyInfo, der::Decode};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

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

        let private_key_info = PrivateKeyInfo::from_der(pem.contents())
            .map_err(|e| HttpSignatureError::Pkcs8(e.to_string()))?;
        let raw = private_key_info.private_key;

        let raw_private_key: [u8; 32] = raw[2..]
            .try_into()
            .map_err(|_| HttpSignatureError::InvalidPrivateKeyLength)?;

        let signing_key = SigningKey::from_bytes(&raw_private_key);
        Ok(signing_key)
    } else {
        // Generate new key
        let mut csprng = OsRng;
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
            ]
            .iter()
            .chain(signing_key.to_bytes().iter())
            .copied()
            .collect::<Vec<u8>>(),
        );

        let pem_string = encode(&pem);
        fs::write(path, pem_string)?;
        Ok(signing_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_generate_new_key() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("test_key.pem");

        // Key should not exist initially
        assert!(!path.exists());

        // Generate a new key
        let key1 = load_or_generate_key(&path).unwrap();

        // File should now exist
        assert!(path.exists());

        // Loading the same key should return the same key
        let key2 = load_or_generate_key(&path).unwrap();
        assert_eq!(key1.to_bytes(), key2.to_bytes());
    }

    #[test]
    fn test_load_existing_key() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("test_key.pem");

        // Generate a key first
        let original_key = load_or_generate_key(&path).unwrap();

        // Load the existing key
        let loaded_key = load_or_generate_key(&path).unwrap();

        // Should be the same key
        assert_eq!(original_key.to_bytes(), loaded_key.to_bytes());
    }

    #[test]
    fn test_invalid_pem_format() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("invalid.pem");

        // Write invalid PEM content
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(b"-----BEGIN INVALID-----\ninvalid content\n-----END INVALID-----")
            .unwrap();

        let result = load_or_generate_key(&path);
        assert!(matches!(result, Err(HttpSignatureError::Pem(_))));
    }

    #[test]
    fn test_wrong_pem_tag() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("wrong_tag.pem");

        // Write PEM with wrong tag
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(b"-----BEGIN PUBLIC KEY-----\ninvalid content\n-----END PUBLIC KEY-----")
            .unwrap();

        let result = load_or_generate_key(&path);
        assert!(matches!(result, Err(HttpSignatureError::Pem(_))));
    }

    #[test]
    fn test_base64_encoded_pem() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("test_key.pem");

        // Generate a key first
        let original_key = load_or_generate_key(&path).unwrap();

        // Read the PEM content and base64 encode it
        let pem_content = fs::read_to_string(&path).unwrap();
        let encoded = STANDARD.encode(pem_content.as_bytes());

        // Write the base64 encoded content back
        fs::write(&path, encoded).unwrap();

        // Should still be able to load the key
        let loaded_key = load_or_generate_key(&path).unwrap();
        assert_eq!(original_key.to_bytes(), loaded_key.to_bytes());
    }

    #[test]
    fn test_key_persistence() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().join("test_key.pem");

        // Generate a key
        let key1 = load_or_generate_key(&path).unwrap();

        // Verify the file contains valid PEM
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("-----BEGIN PRIVATE KEY-----"));
        assert!(content.contains("-----END PRIVATE KEY-----"));

        // Parse the PEM to verify it's valid
        let pem = parse(&content).unwrap();
        assert_eq!(pem.tag(), "PRIVATE KEY");

        // Load the key again to verify it's the same
        let key2 = load_or_generate_key(&path).unwrap();
        assert_eq!(key1.to_bytes(), key2.to_bytes());
    }
}
