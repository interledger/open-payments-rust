use crate::http_signature::error::{HttpSignatureError, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use http::{HeaderMap, Request};

pub struct ValidationOptions<'a> {
    pub request: &'a Request<Option<String>>,
    pub headers: &'a HeaderMap,
    pub public_key: &'a VerifyingKey,
}

impl<'a> ValidationOptions<'a> {
    pub fn new(
        request: &'a Request<Option<String>>,
        headers: &'a HeaderMap,
        public_key: &'a VerifyingKey,
    ) -> Self {
        Self {
            request,
            headers,
            public_key,
        }
    }
}

fn create_signature_base_string(
    request: &Request<Option<String>>,
    components: &[&str],
    created: i64,
    keyid: &str,
) -> String {
    let mut parts = Vec::new();

    for component in components {
        let value = match *component {
            "@method" => request.method().as_str(),
            "@target-uri" => &request.uri().to_string(),
            "authorization" => request
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            "content-digest" => request
                .headers()
                .get("Content-Digest")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            "content-length" => request
                .headers()
                .get("Content-Length")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            "content-type" => request
                .headers()
                .get("Content-Type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            _ => "",
        };
        parts.push(format!("\"{component}\": {value}"));
    }

    let sig_params = format!(
        "({});created={};keyid=\"{}\"",
        components.join(" "),
        created,
        keyid
    );
    parts.push(format!("\"@signature-params\": {sig_params}"));

    parts.join("\n")
}

fn parse_signature_input(signature_input: &str) -> Result<(Vec<&str>, i64, String)> {
    let mut components = Vec::new();
    let mut created = None;
    let mut keyid = None;

    // Remove the sig1= prefix if present
    let signature_input = signature_input.strip_prefix("sig1=").unwrap_or(signature_input);

    for part in signature_input.split(';') {
        if let Some(inner) = part.strip_prefix('(').and_then(|p| p.strip_suffix(')')) {
            components = inner.split(' ').map(|s| s.trim()).collect();
        } else if let Some(value) = part.strip_prefix("created=") {
            created = value.parse::<i64>().ok();
        } else if let Some(value) = part.strip_prefix("keyid=") {
            keyid = Some(value.trim_matches('"').to_string());
        }
    }

    let created = created
        .ok_or_else(|| HttpSignatureError::Validation("Missing created field".to_string()))?;
    let keyid =
        keyid.ok_or_else(|| HttpSignatureError::Validation("Missing keyid field".to_string()))?;

    Ok((components, created, keyid))
}

pub fn validate_signature(options: ValidationOptions<'_>) -> Result<()> {
    let signature_input = options
        .headers
        .get("Signature-Input")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            HttpSignatureError::Validation("Missing Signature-Input header".to_string())
        })?;

    let (components, created, keyid) = parse_signature_input(signature_input)?;

    let signature = options
        .headers
        .get("Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| HttpSignatureError::Validation("Missing Signature header".to_string()))?;

    let signature_base =
        create_signature_base_string(options.request, &components, created, &keyid);

    let signature_bytes = STANDARD
        .decode(signature)
        .map_err(|_| HttpSignatureError::Validation("Base64 decode failed".to_string()))?;

    let signature_bytes: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| HttpSignatureError::Validation("Invalid signature length".to_string()))?;
    let signature = Signature::from_bytes(&signature_bytes);

    options
        .public_key
        .verify(signature_base.as_bytes(), &signature)
        .map_err(|_| HttpSignatureError::Validation("Signature verification failed".to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_signature::{create_signature_headers, SignOptions};
    use ed25519_dalek::{SigningKey, VerifyingKey};
    use http::{HeaderMap, Method, Request, Uri};
    use rand::rngs::OsRng;

    #[test]
    fn test_signature_validation() {
        let mut request = Request::new(Some("test body".to_string()));
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");
        request
            .headers_mut()
            .insert("Content-Type", "application/json".parse().unwrap());

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        let options = SignOptions::new(&request, &signing_key, "test-key".to_string());
        let signature_headers = create_signature_headers(options).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Signature", signature_headers.signature.parse().unwrap());
        headers.insert(
            "Signature-Input",
            signature_headers.signature_input.parse().unwrap(),
        );

        let options = ValidationOptions::new(&request, &headers, &verifying_key);
        assert!(validate_signature(options).is_ok());
    }
}
