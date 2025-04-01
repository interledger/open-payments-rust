# HTTP Signature Utils

A Rust implementation of HTTP signature utilities for Open Payments, providing functionality for creating and validating HTTP message signatures using Ed25519.
Based on `https://www.npmjs.com/package/@interledger/http-signature-utils`

## Features

- JWK (JSON Web Key) generation and handling
- HTTP message signing with Ed25519
- Signature validation
- Header management
- Key management utilities (to be added)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
http-signature-utils = "0.1.0"
```

## Usage

### Generating a JWK

```rust
use http_signature_utils::Jwk;

let jwk = Jwk::new("my-key-id".to_string(), None)?;
println!("Generated JWK: {}", serde_json::to_string_pretty(&jwk)?);
```

### Creating HTTP Message Signatures

```rust
use http::{Request, Method, Uri};
use http_signature_utils::{create_signature_headers, SignOptions};
use ed25519_dalek::Keypair;

// Create a request
let mut request = Request::new(Some("test body".to_string()));
*request.method_mut() = Method::POST;
*request.uri_mut() = Uri::from_static("http://example.com/");

// Generate a keypair
let signing_key = SigningKey::generate(&mut OsRng);

// Create signature headers
let options = SignOptions::new(
    &request,
    &signing_key,
    "test-key".to_string(),
);
let headers = create_signature_headers(options).await?;

println!("Signature: {}", headers.signature);
println!("Signature-Input: {}", headers.signature_input);
```

### Validating Signatures

```rust
use http_signature_utils::{validate_signature, ValidationOptions};
use http::HeaderMap; // headers type

// Assuming you have a request, headers, and public key
let options = ValidationOptions::new(
    &request,
    &headers,
    &public_key,
);
validate_signature(options)?;
```

## Error Handling

The crate uses custom error types for different failure scenarios:

- `JwkError`: Errors related to JWK operations
- `SignatureError`: Errors during signature creation
- `ValidationError`: Errors during signature validation

## Testing

Run the tests with:

```bash
cargo test
```

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details. 