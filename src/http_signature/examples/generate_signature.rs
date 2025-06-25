use base64::Engine;
use ed25519_dalek::SigningKey;
use http::{Method, Request, Uri};
use open_payments::http_signature::signatures::create_signature_headers;
use open_payments::http_signature::{HttpSignatureError, SignOptions};
use rand::rngs::OsRng;
use serde_json::json;

fn main() -> Result<(), HttpSignatureError> {
    // Create a test request
    let mut request = Request::new(Some("test body".to_string()));
    *request.method_mut() = Method::POST;
    *request.uri_mut() = Uri::from_static("http://example.com/");
    request.headers_mut().insert(
        "Content-Type",
        "application/json".parse().map_err(|e| {
            HttpSignatureError::Other(format!("Failed to parse content type: {}", e))
        })?,
    );

    // Generate a signing key
    let signing_key = SigningKey::generate(&mut OsRng);

    // Create signature headers
    let options = SignOptions::new(&request, &signing_key, "test-key".to_string());
    let headers = create_signature_headers(options)?;

    // Output the signature data and public key in JSON format
    println!(
        "{}",
        json!({
            "signature": headers.signature,
            "signature_input": headers.signature_input,
            "public_key": base64::engine::general_purpose::STANDARD.encode(signing_key.verifying_key().to_bytes()),
        })
    );
    Ok(())
}
