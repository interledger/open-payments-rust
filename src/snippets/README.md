# Open Payments Rust SDK Snippets

These code snippets are intended for use with <a href="https://rafiki.money" target="_blank">Rafiki Money</a>. While
everyone is welcome to use these code snippets as a reference, please note that they may need to be adapted to suit your
particular application.

---

### Prerequisites

-   Rust (>=1.43.1)
-   Cargo (comes with Rust)
-   [Rust Open Payments SDK](https://github.com/interledger/open-payments-rust)

### Recommendations

Before working with the code snippets, create a <a href="https://wallet.interledger-test.dev/auth/signup" target="_blank">test wallet account</a> on the Interledger test network. More information about setting up and funding your test wallet account, as well as obtaining a public-private key pair and ID can be found in the <a href="https://openpayments.dev/snippets/before-you-begin" target="_blank">Open Payments documentation</a>. You'll need the private key to complete your setup.

### Setup

Build the project:

```sh
cargo build
```

### Loading the private key

When you <a href="https://openpayments.dev/snippets/before-you-begin/#obtain-a-public-private-key-pair-and-key-id" target="_blank">get your public-private key pair</a>, the private key is automatically downloaded to your machine. 

Move the `private.key` file to the location specified in your environment variables.

Ensure you are at the repository root and execute the following command in your terminal:

```sh
cp .env.example .env
```

Open the newly created `.env` file and fill in the following variables:

-   `PRIVATE_KEY_PATH`
-   `KEY_ID`
-   `JWKS_PATH` (path to store the JWK generated from the private key)

Now that you have all the necessary components to initialize the authenticated Open Payments client, you're ready to
begin utilizing the code snippets.

### Running a snippet

From the repository's root you can execute the following commands:

| Command | Description |
| ------------------ | ---------------------------------------------- |
| `cargo run --features snippets --bin grant-cancel` | Cancel a grant |
| `cargo run --features snippets --bin grant-continuation` | Continuation request for a grant (interactive) |
| `cargo run --features snippets --bin grant-incoming-payment` | Request a grant for an incoming payment |
| `cargo run --features snippets --bin grant-outgoing-payment` | Request a grant for an outgoing payment |
| `cargo run --features snippets --bin grant-quote` | Request a grant for a quote |
| `cargo run --features snippets --bin incoming-payment-create` | Create an incoming payment |
| `cargo run --features snippets --bin incoming-payment-complete` | Complete an incoming payment |
| `cargo run --features snippets --bin incoming-payment-get` | Retrieve an incoming payment |
| `cargo run --features snippets --bin incoming-payment-list` | List incoming payments |
| `cargo run --features snippets --bin outgoing-payment-create` | Create an outgoing payment |
| `cargo run --features snippets --bin outgoing-payment-get` | Retrieve an outgoing payment |
| `cargo run --features snippets --bin outgoing-payment-list` | List outgoing payments |
| `cargo run --features snippets --bin quote-create` | Create a quote |
| `cargo run --features snippets --bin quote-create-debit-amount` | Create a quote with debit amount |
| `cargo run --features snippets --bin quote-create-receive-amount` | Create a quote with receive amount |
| `cargo run --features snippets --bin quote-get` | Retrieve a quote |
| `cargo run --features snippets --bin wallet-address-get` | Retrieve wallet address' information |
| `cargo run --features snippets --bin wallet-address-get-keys` | Retrieve wallet address' JWKs |
| `cargo run --features snippets --bin token-revoke` | Revoke a token |
| `cargo run --features snippets --bin token-rotate` | Rotate a token |

Example:

```sh
cargo run --features snippets --bin grant-incoming-payment
```

### Examples

-   [Payment flow example](./examples/payment-flow.md)