# HTTP Signature Utils

A Rust implementation of HTTP signature utilities for Open Payments, providing functionality for creating and validating HTTP message signatures using Ed25519.
Based on [`@interledger/http-signature-utils`](https://www.npmjs.com/package/@interledger/http-signature-utils)

## Features

- JWK (JSON Web Key) generation and handling
- HTTP message signing with Ed25519
- Signature validation
- Header management
- Key management utilities

## Usage

This module is part of the [`open-payments`](https://github.com/interledger/open-payments-rust) crate. Add the crate to your `Cargo.toml`:

```toml
[dependencies]
open-payments = "0.1.0"
```

Then import the module in your code:

### Generating a JWK

```rust
use open_payments::http_signature::Jwk;

let jwk = Jwk::new("my-key-id".to_string(), None)?;
println!("Generated JWK: {}", serde_json::to_string_pretty(&jwk)?);
```

### Creating HTTP Message Signatures

```rust
use http::{Request, Method, Uri};
use open_payments::http_signature::{create_signature_headers, SignOptions};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

// Create a request
let mut request = Request::new(Some("test body".to_string()));
*request.method_mut() = Method::POST;
*request.uri_mut() = Uri::from_static("http://example.com/");

// Generate a signing key
let signing_key = SigningKey::generate(&mut OsRng);

// Create signature headers
let options = SignOptions::new(
    &request,
    &signing_key,
    "test-key".to_string(),
);
let headers = create_signature_headers(options)?;

println!("Signature: {}", headers.signature);
println!("Signature-Input: {}", headers.signature_input);
```

### Validating Signatures

```rust
use open_payments::http_signature::{validate_signature, ValidationOptions};
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

This module uses custom error types for different failure scenarios:

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