use crate::http_signature::error::{HttpSignatureError, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};
use http::{HeaderMap, Request};
use sha2::{Digest, Sha512};

pub struct ValidationOptions<'a> {
    pub request: &'a Request<Option<String>>,
    pub headers: &'a HeaderMap,
    pub public_key: &'a VerifyingKey,
}

impl ValidationOptions<'_> {
    pub fn new<'a>(
        request: &'a Request<Option<String>>,
        headers: &'a HeaderMap,
        public_key: &'a VerifyingKey,
    ) -> ValidationOptions<'a> {
        ValidationOptions {
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
    let signature_input = signature_input
        .strip_prefix("sig1=")
        .unwrap_or(signature_input);

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

fn normalize_component(component: &str) -> &str {
    component.trim().trim_matches('"')
}

fn request_has_body(request: &Request<Option<String>>) -> bool {
    match request.body() {
        Some(body) if !body.is_empty() => true,
        _ => false,
    }
}

/// Enforce Open Payments / GNAP MUST covered components before cryptographic
/// verification (parity with open-payments-go / open-payments-node).
fn validate_required_covered_components(
    request: &Request<Option<String>>,
    components: &[&str],
) -> Result<()> {
    let covered: Vec<&str> = components.iter().map(|c| normalize_component(c)).collect();

    if !covered.iter().any(|c| c.eq_ignore_ascii_case("@method")) {
        return Err(HttpSignatureError::Validation(
            "Signature-Input missing required @method component".to_string(),
        ));
    }
    if !covered.iter().any(|c| c.eq_ignore_ascii_case("@target-uri")) {
        return Err(HttpSignatureError::Validation(
            "Signature-Input missing required @target-uri component".to_string(),
        ));
    }
    if request.headers().get("Authorization").is_some()
        && !covered.iter().any(|c| c.eq_ignore_ascii_case("authorization"))
    {
        return Err(HttpSignatureError::Validation(
            "Signature-Input missing required authorization component".to_string(),
        ));
    }
    if request_has_body(request)
        && !covered.iter().any(|c| c.eq_ignore_ascii_case("content-digest"))
    {
        return Err(HttpSignatureError::Validation(
            "Signature-Input missing required content-digest component for request body"
                .to_string(),
        ));
    }
    Ok(())
}

fn content_digest_for_body(body: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(body.as_bytes());
    let digest = STANDARD.encode(hasher.finalize());
    format!("sha-512=:{digest}:")
}

fn verify_content_digest(request: &Request<Option<String>>) -> Result<()> {
    let digest_header = request
        .headers()
        .get("Content-Digest")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            HttpSignatureError::Validation("Missing Content-Digest header".to_string())
        })?;

    let body = request.body().as_ref().ok_or_else(|| {
        HttpSignatureError::Validation("Missing request body for Content-Digest".to_string())
    })?;

    let expected = content_digest_for_body(body);
    if !expected.eq_ignore_ascii_case(digest_header) {
        return Err(HttpSignatureError::Validation(
            "Content-Digest does not match request body".to_string(),
        ));
    }
    Ok(())
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

    validate_required_covered_components(options.request, &components)?;

    if components
        .iter()
        .any(|c| normalize_component(c).eq_ignore_ascii_case("content-digest"))
    {
        verify_content_digest(options.request)?;
    }

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

    fn attach_content_headers(request: &mut Request<Option<String>>) {
        if let Some(body) = request.body().clone() {
            let digest = content_digest_for_body(&body);
            request
                .headers_mut()
                .insert("Content-Digest", digest.parse().unwrap());
            request
                .headers_mut()
                .insert("Content-Length", body.len().to_string().parse().unwrap());
        }
    }

    #[test]
    fn test_signature_validation() {
        let mut request = Request::new(Some("test body".to_string()));
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");
        request
            .headers_mut()
            .insert("Content-Type", "application/json".parse().unwrap());
        attach_content_headers(&mut request);

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

    #[test]
    fn test_missing_signature_input_header() {
        let mut request = Request::new(None);
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        let headers = HeaderMap::new();
        let options = ValidationOptions::new(&request, &headers, &verifying_key);
        let err = validate_signature(options).unwrap_err();
        match err {
            HttpSignatureError::Validation(msg) => {
                assert_eq!(msg, "Missing Signature-Input header");
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_missing_signature_header() {
        let mut request = Request::new(None);
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");

        let signing_key = SigningKey::generate(&mut OsRng);
        let options = SignOptions::new(&request, &signing_key, "k".to_string());
        let sig = create_signature_headers(options).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Signature-Input", sig.signature_input.parse().unwrap());

        let verifying_key = VerifyingKey::from(&signing_key);
        let options = ValidationOptions::new(&request, &headers, &verifying_key);
        let err = validate_signature(options).unwrap_err();
        match err {
            HttpSignatureError::Validation(msg) => {
                assert_eq!(msg, "Missing Signature header");
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_base64_decode_failed() {
        let mut request = Request::new(None);
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");

        let signing_key = SigningKey::generate(&mut OsRng);
        let options = SignOptions::new(&request, &signing_key, "k".to_string());
        let sig = create_signature_headers(options).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Signature-Input", sig.signature_input.parse().unwrap());
        headers.insert("Signature", "%%%".parse().unwrap());

        let verifying_key = VerifyingKey::from(&signing_key);
        let options = ValidationOptions::new(&request, &headers, &verifying_key);
        let err = validate_signature(options).unwrap_err();
        match err {
            HttpSignatureError::Validation(msg) => {
                assert_eq!(msg, "Base64 decode failed");
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_signature_length() {
        let mut request = Request::new(None);
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");

        let signing_key = SigningKey::generate(&mut OsRng);
        let options = SignOptions::new(&request, &signing_key, "k".to_string());
        let sig = create_signature_headers(options).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Signature-Input", sig.signature_input.parse().unwrap());
        headers.insert("Signature", "aGVsbG8=".parse().unwrap()); // "hello"

        let verifying_key = VerifyingKey::from(&signing_key);
        let options = ValidationOptions::new(&request, &headers, &verifying_key);
        let err = validate_signature(options).unwrap_err();
        match err {
            HttpSignatureError::Validation(msg) => {
                assert_eq!(msg, "Invalid signature length");
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_signature_verification_failed() {
        let mut request = Request::new(Some("body".to_string()));
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("http://example.com");
        request
            .headers_mut()
            .insert("Content-Type", "application/json".parse().unwrap());
        attach_content_headers(&mut request);

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        let options = SignOptions::new(&request, &signing_key, "k".to_string());
        let sig = create_signature_headers(options).unwrap();

        // Tamper with request after signing to force verification failure
        *request.uri_mut() = Uri::from_static("http://example.com/changed");

        let mut headers = HeaderMap::new();
        headers.insert("Signature-Input", sig.signature_input.parse().unwrap());
        headers.insert("Signature", sig.signature.parse().unwrap());

        let options = ValidationOptions::new(&request, &headers, &verifying_key);
        let err = validate_signature(options).unwrap_err();
        match err {
            HttpSignatureError::Validation(msg) => {
                assert_eq!(msg, "Signature verification failed");
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_rejects_under_covered_components_with_body() {
        let mut request = Request::new(Some(r#"{"amount":"1"}"#.to_string()));
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("https://example.com/resource");
        request
            .headers_mut()
            .insert("Content-Type", "application/json".parse().unwrap());
        attach_content_headers(&mut request);

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        // Cover method/target/type but omit content-digest despite body.
        let created = 1700000000i64;
        let keyid = "test-key";
        let components = ["@method", "@target-uri", "content-type"];
        let base = create_signature_base_string(&request, &components, created, keyid);
        let signature = STANDARD.encode(signing_key.sign(base.as_bytes()).to_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("Signature", signature.parse().unwrap());
        headers.insert(
            "Signature-Input",
            r#"sig1=(@method @target-uri content-type);created=1700000000;keyid="test-key""#
                .parse()
                .unwrap(),
        );

        let err = validate_signature(ValidationOptions::new(&request, &headers, &verifying_key))
            .unwrap_err();
        match err {
            HttpSignatureError::Validation(msg) => {
                assert!(msg.contains("content-digest"), "unexpected: {msg}");
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_rejects_body_digest_mismatch() {
        let mut request = Request::new(Some(r#"{"amount":"1"}"#.to_string()));
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("https://example.com/resource");
        request
            .headers_mut()
            .insert("Content-Type", "application/json".parse().unwrap());
        attach_content_headers(&mut request);

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        let options = SignOptions::new(&request, &signing_key, "test-key".to_string());
        let signature_headers = create_signature_headers(options).unwrap();

        // Swap body under the original Content-Digest + Signature headers.
        *request.body_mut() = Some(r#"{"amount":"999"}"#.to_string());
        request
            .headers_mut()
            .insert("Content-Length", "16".parse().unwrap());

        let mut headers = HeaderMap::new();
        headers.insert("Signature", signature_headers.signature.parse().unwrap());
        headers.insert(
            "Signature-Input",
            signature_headers.signature_input.parse().unwrap(),
        );

        let err = validate_signature(ValidationOptions::new(&request, &headers, &verifying_key))
            .unwrap_err();
        match err {
            HttpSignatureError::Validation(msg) => {
                assert!(msg.contains("Content-Digest"), "unexpected: {msg}");
            }
            _ => panic!("unexpected error type"),
        }
    }
}
