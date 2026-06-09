# Open Payments

<p align="center">
  <img src="https://raw.githubusercontent.com/interledger/open-payments/main/docs/public/img/logo.svg" width="700" alt="Open Payments">
</p>

## What is Open Payments?

Open Payments is an open API standard that can be implemented by account servicing entities (e.g. banks, digital wallet providers, and mobile money providers) to facilitate interoperability in the setup and completion of payments for different use cases including:

- [Web Monetization](https://webmonetization.org)
- Tipping/Donations (low value/low friction)
- eCommerce checkout
- P2P transfers
- Subscriptions
- Invoice Payments

An Open Payments server runs two sub-systems, a **resource server** which exposes APIs for performing functions against the
underlying accounts and an **authorisation server** which exposes APIs compliant with the
[GNAP](https://datatracker.ietf.org/doc/html/draft-ietf-gnap-core-protocol) standard for getting grants to access the resource server
APIs.

This repository hosts the Open API Specifications of the two APIs which are published along with additional documentation at
https://openpayments.dev.

Additionally, this crate contains several modules:

- [`client`](./src/client) contains a Rust client to make requests via the Open Payments API.
- [`types`](./src/types) contains Rust types for the API.
- [`snippets`](./src/snippets) contains examples of Rust client usage for getting accustomed to the Open Payments flow.
- [`http_signature`](./src/http_signature) provides tools for working with [HTTP Message Signatures](https://datatracker.ietf.org/doc/draft-ietf-httpbis-message-signatures).

## Dependencies

- [Interledger](https://interledger.org/developers/rfcs/interledger-protocol/)

### New to Interledger?

Never heard of Interledger before? Or would you like to learn more? Here are some excellent places to start:

- [Interledger Website](https://interledger.org/)
- [Interledger Specification](https://interledger.org/developers/rfcs/interledger-protocol/)
- [Interledger Explainer Video](https://twitter.com/Interledger/status/1567916000074678272)
- [Open Payments](https://openpayments.dev/)
- [Web monetization](https://webmonetization.org/)

## Contributing

Please read the [contribution guidelines](.github/contributing.md) before submitting contributions. All contributions must adhere to our [code of conduct](.github/code_of_conduct.md).

## Open Payments Catchup Call

Our catchup calls are open to our community. We have them every other Wednesday at 13:00 GMT, via Google Meet.

Video call link: https://meet.google.com/htd-eefo-ovn

Or dial: (DE) +49 30 300195061 and enter this PIN: 105 520 503#

More phone numbers: https://tel.meet/htd-eefo-ovn?hs=5

[Add to Google Calendar](https://calendar.google.com/calendar/event?action=TEMPLATE&tmeid=MDNjYTdhYmE5MTgwNGJhMmIxYmU0YWFkMzI2NTFmMjVfMjAyNDA1MDhUMTIwMDAwWiBjX2NqMDI3Z21oc3VqazkxZXZpMjRkOXB2bXQ0QGc&tmsrc=c_cj027gmhsujk91evi24d9pvmt4%40group.calendar.google.com&scp=ALL)

## Local Development Environment

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (>= 1.70)
- [Git](https://git-scm.com/downloads)

### Environment Setup

Clone the repository and build:

```sh
git clone https://github.com/interledger/open-payments-rust.git
cd open-payments-rust
cargo build
```

### Running Examples

See [src/snippets/README.md](./src/snippets/README.md) for details on running code snippets and payment flows. For example, to run a snippet:

```sh
cd src/snippets
cargo run --features snippets --bin grant-incoming-payment
```

### Useful commands

```sh
# Format all code:
cargo fmt --all

# Run all tests:
cargo test

# Check code for warnings and errors:
cargo check

# Build crate:
cargo build

# Generate documentation:
cargo doc --no-deps

# Generate documentation and open in browser:
cargo doc --no-deps --open

# Generate documentation with snippets feature:
cargo doc --features snippets --no-deps
```

## Adding crate as dependency

Add this to your `Cargo.toml`:

```toml
[dependencies]
open-payments = "0.1.2"
```

## Quick Start

### Unauthenticated Client

Fetch public wallet address information without authentication:

```rust
use open_payments::client::{UnauthenticatedClient, UnauthenticatedResources};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UnauthenticatedClient::new();

    let wallet = client
        .wallet_address()
        .get("https://ilp.rafiki.money/alice")
        .await?;

    println!("Wallet: {} ({})", wallet.public_name, wallet.id);
    println!("Asset: {} (scale {})", wallet.asset_code, wallet.asset_scale);
    println!("Auth server: {}", wallet.auth_server);
    Ok(())
}
```

### Authenticated Client

Create payments using an authenticated client with HTTP message signatures:

```rust
use open_payments::client::{AuthenticatedClient, AuthenticatedResources, ClientConfig};
use open_payments::types::{
    AccessItem, AccessTokenRequest, CreateIncomingPaymentRequest, GrantRequest,
    IncomingPaymentAction,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig {
        key_id: "my-key-id".to_string(),
        private_key_path: PathBuf::from("private.pem"),
        jwks_path: Some(PathBuf::from("jwks.json")),
        wallet_address_url: "https://ilp.rafiki.money/alice".into(),
        ..Default::default()
    };

    let client = AuthenticatedClient::new(config)?;

    // 1. Request a grant for incoming payments
    let grant_request = GrantRequest::new(
        AccessTokenRequest {
            access: vec![AccessItem::IncomingPayment {
                actions: vec![
                    IncomingPaymentAction::Create,
                    IncomingPaymentAction::Read,
                ],
                identifier: None,
            }],
        },
        None,
    );

    let grant = client
        .grant()
        .request("https://auth.ilp.rafiki.money", &grant_request)
        .await?;

    // 2. Create an incoming payment (using the access token from the grant)
    // ... see src/snippets/ for complete payment flow examples

    Ok(())
}
```

### HTTP Message Signatures

The SDK handles HTTP message signature creation and validation per [RFC 9421](https://www.rfc-editor.org/rfc/rfc9421):

```rust
use open_payments::http_signature::{
    create_signature_headers, validate_signature,
    SignOptions, ValidationOptions,
};
use ed25519_dalek::SigningKey;
use http::{Request, Method, Uri};
use rand::rngs::OsRng;

let mut request = Request::new(Some("{}".to_string()));
*request.method_mut() = Method::POST;
*request.uri_mut() = Uri::from_static("https://ilp.rafiki.money/incoming-payments");

let signing_key = SigningKey::generate(&mut OsRng);
let options = SignOptions::new(&request, &signing_key, "my-key-id".to_string());
let sig_headers = create_signature_headers(options)?;

println!("Signature: {}", sig_headers.signature);
println!("Signature-Input: {}", sig_headers.signature_input);
```
