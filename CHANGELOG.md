# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-08-28

### Added

- Directed identity support: optional `client_override: Option<&JsonWebKey>` on `grant().request(...)`.
- `AuthenticatedClient::public_jwk()` helper for building directed-identity grant requests.
- Grant `subject` request/response types (`Subject`, `SubjectIdentifier`) and `GrantRequest::subject_only` / `with_subject`.
- `Client` enum for grant client identity (wallet address URL, wallet address object, or JWK).
- `outgoing_payments().get_grant_spent_amounts(...)` (`GET /outgoing-payment-grant`).
- `OutgoingPaymentGrantSpentAmounts` response type (required keys, nullable amounts).
- `ContinueResponse::WithSubject` variant and subject helpers.
- Open Payments specifications submodule pinned to Node SDK commit `70a3697` (spec `1.3.0` + spent-amount nullability).

### Changed

- **Breaking:** `grant().request` now takes a third argument `client_override: Option<&JsonWebKey>` (pass `None` for previous behavior).
- **Breaking:** `OutgoingPayment.grant_spent_debit_amount` / `grant_spent_receive_amount` are now `Option<Amount>`.
- **Breaking:** `GrantRequest.access_token` is now `Option<AccessTokenRequest>` (still set by `GrantRequest::new`).
- `JsonWebKey.use_` now serializes as JSON `"use"`.
