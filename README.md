# Open Payments Rust

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

- [`client`](./client) contains a Rust client to make requests via the Open Payments API, as well as re-exporting Rust types for the API from `op-types`.
- [`types`](./types) contains Rust types for the API.
- [`snippets`](./snippets) contains examples of Rust client usage for getting accustomed to the Open Payments flow.
- [`http-signature`](./http-signature) provides tools for working with [HTTP Message Signatures](https://datatracker.ietf.org/doc/draft-ietf-httpbis-message-signatures).

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

- [Rust](https://www.rust-lang.org/tools/install) (>= 1.43.1)
- [Git](https://git-scm.com/downloads)

### Environment Setup

Clone the repository and build:

```sh
git clone https://github.com/interledger/open-payments-rust.git
cd open-payments-rust
cargo build
```

### Running Examples

See [op-snippets/README.md](./op-snippets/README.md) for details on running code snippets and payment flows. For example, to run a snippet:

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
```
