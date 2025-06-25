use crate::http_signature::error::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Signer, SigningKey};
use http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureHeaders {
    pub signature: String,
    pub signature_input: String,
}

pub struct SignOptions<'a> {
    pub request: &'a Request<Option<String>>,
    pub private_key: &'a SigningKey,
    pub key_id: String,
}

impl<'a> SignOptions<'a> {
    pub fn new(
        request: &'a Request<Option<String>>,
        private_key: &'a SigningKey,
        key_id: String,
    ) -> Self {
        Self {
            request,
            private_key,
            key_id,
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
        parts.push(format!("\"{}\": {}", component, value));
    }

    let sig_params = format!(
        "({});created={};keyid=\"{}\"",
        components.join(" "),
        created,
        keyid
    );
    parts.push(format!("\"@signature-params\": {}", sig_params));

    parts.join("\n")
}

pub fn create_signature_headers(options: SignOptions<'_>) -> Result<SignatureHeaders> {
    let mut components = vec!["@method", "@target-uri", "content-type"];

    if options.request.headers().get("Authorization").is_some() {
        components.push("authorization");
    }

    if options.request.body().is_some() {
        components.extend_from_slice(&["content-digest", "content-length"]);
    }
    let created = chrono::Utc::now().timestamp();

    let signature_base =
        create_signature_base_string(options.request, &components, created, &options.key_id);

    let signature_bytes = options.private_key.sign(signature_base.as_bytes());
    let signature = STANDARD.encode(signature_bytes.to_bytes());

    let signature_input = format!(
        "sig1=({});created={};keyid=\"{}\"",
        components.join(" "),
        created,
        options.key_id
    );

    Ok(SignatureHeaders {
        signature,
        signature_input,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use http::{Method, Request, Uri};
    use rand::rngs::OsRng;

    #[test]
    fn test_signature_creation() {
        let mut request = Request::new(Some("test body".to_string()));
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");
        request
            .headers_mut()
            .insert("Content-Type", "application/json".parse().unwrap());

        let signing_key = SigningKey::generate(&mut OsRng);
        let options = SignOptions::new(&request, &signing_key, "test-key".to_string());

        let headers = create_signature_headers(options).unwrap();
        assert!(!headers.signature.is_empty());
        assert!(!headers.signature_input.is_empty());
    }
}
