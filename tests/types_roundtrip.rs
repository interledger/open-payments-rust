use open_payments::types::*;
use chrono::{TimeZone, Utc};

fn serde_roundtrip<T>(value: &T)
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
{
    let s = serde_json::to_string(value).expect("serialize");
    let back: T = serde_json::from_str(&s).expect("deserialize");
    assert_eq!(&back, value);
}

#[test]
fn amount_roundtrip() {
    let v = Amount { value: "1000".into(), asset_code: "USD".into(), asset_scale: 2 };
    serde_roundtrip(&v);
}

#[test]
fn wallet_address_roundtrip() {
    let v = WalletAddress {
        id: "https://ilp.interledger-test.dev/alice".into(),
        public_name: Some("Alice Test Wallet".into()),
        asset_code: "USD".into(),
        asset_scale: 2,
        auth_server: "https://auth.interledger-test.dev".into(),
        resource_server: "https://ilp.interledger-test.dev".into(),
    };
    serde_roundtrip(&v);
}

#[test]
fn incoming_payment_roundtrip_minimal() {
    let v = IncomingPayment {
        id: "https://ilp.interledger-test.dev/incoming-payments/123".into(),
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        completed: false,
        incoming_amount: None,
        received_amount: Amount { value: "0".into(), asset_code: "USD".into(), asset_scale: 2 },
        expires_at: None,
        metadata: None,
        created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        updated_at: None,
        methods: None,
    };
    serde_roundtrip(&v);
}

#[test]
fn outgoing_payment_roundtrip_minimal() {
    let v = OutgoingPayment {
        id: "https://ilp.interledger-test.dev/outgoing-payments/abc".into(),
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        quote_id: Some("https://ilp.interledger-test.dev/quotes/q1".into()),
        failed: false,
        receiver: Receiver("https://ilp.interledger-test.dev/incoming-payments/123".into()),
        receive_amount: Amount { value: "10".into(), asset_code: "USD".into(), asset_scale: 2 },
        debit_amount: Amount { value: "110".into(), asset_code: "USD".into(), asset_scale: 2 },
        sent_amount: Amount { value: "0".into(), asset_code: "USD".into(), asset_scale: 2 },
        grant_spent_debit_amount: Amount { value: "0".into(), asset_code: "USD".into(), asset_scale: 2 },
        grant_spent_receive_amount: Amount { value: "0".into(), asset_code: "USD".into(), asset_scale: 2 },
        metadata: None,
        created_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        updated_at: None,
    };
    serde_roundtrip(&v);
}

#[test]
fn quote_roundtrip() {
    let v = Quote {
        id: "https://ilp.interledger-test.dev/quotes/q1".into(),
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        receiver: Receiver("https://ilp.interledger-test.dev/incoming-payments/123".into()),
        receive_amount: Amount { value: "10".into(), asset_code: "USD".into(), asset_scale: 2 },
        debit_amount: Amount { value: "110".into(), asset_code: "USD".into(), asset_scale: 2 },
        method: PaymentMethodType::Ilp,
        expires_at: None,
        created_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
    };
    serde_roundtrip(&v);
}

#[test]
fn quote_request_untagged_roundtrip_variants() {
    let base = CreateQuoteRequest::NoAmountQuote {
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        receiver: Receiver("https://ilp.interledger-test.dev/incoming-payments/123".into()),
        method: PaymentMethodType::Ilp,
    };
    serde_roundtrip(&base);

    let fixed_recv = CreateQuoteRequest::FixedReceiveAmountQuote {
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        receiver: Receiver("https://ilp.interledger-test.dev/incoming-payments/123".into()),
        method: PaymentMethodType::Ilp,
        receive_amount: Amount { value: "10".into(), asset_code: "USD".into(), asset_scale: 2 },
    };
    serde_roundtrip(&fixed_recv);

    let fixed_send = CreateQuoteRequest::FixedSendAmountQuote {
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        receiver: Receiver("https://ilp.interledger-test.dev/incoming-payments/123".into()),
        method: PaymentMethodType::Ilp,
        debit_amount: Amount { value: "10".into(), asset_code: "USD".into(), asset_scale: 2 },
    };
    serde_roundtrip(&fixed_send);
}

#[test]
fn outgoing_payment_request_untagged_roundtrip_variants() {
    let from_quote = CreateOutgoingPaymentRequest::FromQuote {
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        quote_id: "https://ilp.interledger-test.dev/quotes/123".into(),
        metadata: None,
    };
    serde_roundtrip(&from_quote);

    let from_incoming = CreateOutgoingPaymentRequest::FromIncomingPayment {
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        incoming_payment_id: "https://ilp.interledger-test.dev/incoming-payments/123".into(),
        debit_amount: Amount { value: "110".into(), asset_code: "USD".into(), asset_scale: 2 },
        metadata: None,
    };
    serde_roundtrip(&from_incoming);
}

#[test]
fn access_item_roundtrip_variants() {
    let inc = AccessItem::IncomingPayment { actions: vec![IncomingPaymentAction::Create, IncomingPaymentAction::Read], identifier: None };
    serde_roundtrip(&inc);

    let out = AccessItem::OutgoingPayment {
        actions: vec![OutgoingPaymentAction::Create, OutgoingPaymentAction::Read, OutgoingPaymentAction::List],
        identifier: "https://ilp.interledger-test.dev/alice".into(),
        limits: None,
    };
    serde_roundtrip(&out);

    let quote = AccessItem::Quote { actions: vec![QuoteAction::Create, QuoteAction::Read] };
    serde_roundtrip(&quote);
}

#[test]
fn actions_roundtrip_all_variants() {
    for action in [IncomingPaymentAction::Create, IncomingPaymentAction::Complete, IncomingPaymentAction::Read, IncomingPaymentAction::ReadAll, IncomingPaymentAction::List, IncomingPaymentAction::ListAll] {
        serde_roundtrip(&action);
    }
    for action in [OutgoingPaymentAction::Create, OutgoingPaymentAction::Read, OutgoingPaymentAction::ReadAll, OutgoingPaymentAction::List, OutgoingPaymentAction::ListAll] {
        serde_roundtrip(&action);
    }
    for action in [QuoteAction::Create, QuoteAction::Read, QuoteAction::ReadAll] {
        serde_roundtrip(&action);
    }
}

#[test]
fn grant_and_continue_response_roundtrip_variants() {
    let cont = Continue { access_token: open_payments::types::ContinueAccessToken { value: "ctok".into() }, uri: "https://auth.interledger-test.dev/continue/abc".into(), wait: Some(1) };
    let with_token = GrantResponse::WithToken { access_token: AccessToken { value: "av".into(), manage: "https://auth.interledger-test.dev/manage".into(), expires_in: Some(3600), access: None }, continue_: cont.clone() };
    serde_roundtrip(&with_token);

    let with_interaction = GrantResponse::WithInteraction { interact: InteractResponse { redirect: "https://auth.interledger-test.dev/interact/abc/finish".into(), finish: "finish-nonce".into() }, continue_: cont.clone() };
    serde_roundtrip(&with_interaction);

    let cr_with_token = ContinueResponse::WithToken { access_token: AccessToken { value: "av".into(), manage: "https://auth.interledger-test.dev/manage".into(), expires_in: Some(3600), access: None }, continue_: cont.clone() };
    serde_roundtrip(&cr_with_token);
    let cr_pending = ContinueResponse::Pending { continue_: cont };
    serde_roundtrip(&cr_pending);
}

#[test]
fn jwk_enums_and_payment_method_roundtrip() {
    serde_roundtrip(&PaymentMethodType::Ilp);
    let pm = PaymentMethod::Ilp { ilp_address: "test.bank".into(), shared_secret: "abc".into() };
    serde_roundtrip(&pm);

    serde_roundtrip(&JwkAlgorithm::EdDSA);
    serde_roundtrip(&JwkUse::Signature);
    serde_roundtrip(&JwkKeyType::OKP);
    serde_roundtrip(&JwkCurve::Ed25519);
}

#[test]
fn receiver_walleturi_interval_roundtrip() {
    serde_roundtrip(&Receiver("https://ilp.interledger-test.dev/incoming-payments/xyz".into()));
    serde_roundtrip(&WalletAddressUri("https://ilp.interledger-test.dev/alice".into()));
    serde_roundtrip(&Interval("P1D".into()));
}

#[test]
fn jwk_set_roundtrip() {
    let jwk = JsonWebKey {
        kid: "kid-1".into(),
        alg: JwkAlgorithm::EdDSA,
        use_: Some(JwkUse::Signature),
        kty: JwkKeyType::OKP,
        crv: JwkCurve::Ed25519,
        x: "base64url".into(),
    };
    let set = JsonWebKeySet { keys: vec![jwk] };
    serde_roundtrip(&set);
}

#[test]
fn access_token_and_response_roundtrip() {
    let tok = AccessToken { value: "token".into(), manage: "https://auth.interledger-test.dev/manage".into(), expires_in: Some(3600), access: None };
    serde_roundtrip(&tok);
    let resp = AccessTokenResponse { access_token: tok };
    serde_roundtrip(&resp);
}

#[test]
fn grant_and_related_requests_roundtrip() {
    let at_req = AccessTokenRequest { access: vec![AccessItem::Quote { actions: vec![QuoteAction::Create] }] };
    serde_roundtrip(&at_req);

    let grant = GrantRequest::new(at_req, Some(InteractRequest { start: vec!["redirect".into()], finish: Some(InteractFinish { method: "redirect".into(), uri: "https://client.example/finish".into(), nonce: "n".into() }) }));
    serde_roundtrip(&grant);

    let ir = InteractResponse { redirect: "https://auth.interledger-test.dev/interact/abc/finish".into(), finish: "finish".into() };
    serde_roundtrip(&ir);

    let cont_req = ContinueRequest { interact_ref: Some("ref123".into()) };
    serde_roundtrip(&cont_req);

    let cont = Continue { access_token: ContinueAccessToken { value: "ctok".into() }, uri: "https://auth.interledger-test.dev/continue/abc".into(), wait: Some(1) };
    serde_roundtrip(&cont);
}

#[test]
fn limits_outgoing_roundtrip() {
    let limits = LimitsOutgoing {
        receiver: Some(Receiver("https://ilp.interledger-test.dev/incoming-payments/xyz".into())),
        debit_amount: Some(Amount { value: "200".into(), asset_code: "USD".into(), asset_scale: 2 }),
        receive_amount: None,
        interval: Some(Interval("P1D".into())),
    };
    serde_roundtrip(&limits);
}

#[test]
fn public_and_create_incoming_payment_roundtrip() {
    let pip = PublicIncomingPayment { received_amount: Amount { value: "0".into(), asset_code: "USD".into(), asset_scale: 2 }, auth_server: "https://auth.interledger-test.dev".into() };
    serde_roundtrip(&pip);

    let cip = CreateIncomingPaymentRequest { wallet_address: "https://ilp.interledger-test.dev/alice".into(), incoming_amount: Some(Amount { value: "100".into(), asset_code: "USD".into(), asset_scale: 2 }), expires_at: None, metadata: None };
    serde_roundtrip(&cip);
}

#[test]
fn incoming_payment_with_methods_roundtrip() {
    let ilp = PaymentMethod::Ilp { ilp_address: "test.bank".into(), shared_secret: "s".into() };
    let base = IncomingPayment {
        id: "https://ilp.interledger-test.dev/incoming-payments/123".into(),
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        completed: false,
        incoming_amount: Some(Amount { value: "10".into(), asset_code: "USD".into(), asset_scale: 2 }),
        received_amount: Amount { value: "0".into(), asset_code: "USD".into(), asset_scale: 2 },
        expires_at: None,
        metadata: None,
        created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        updated_at: None,
        methods: Some(vec![ilp.clone()]),
    };
    let wrapped = IncomingPaymentWithMethods { payment: base, methods: vec![ilp] };
    serde_roundtrip(&wrapped);
}

#[test]
fn paginated_response_roundtrip() {
    let item = IncomingPayment {
        id: "https://ilp.interledger-test.dev/incoming-payments/1".into(),
        wallet_address: "https://ilp.interledger-test.dev/alice".into(),
        completed: false,
        incoming_amount: None,
        received_amount: Amount { value: "0".into(), asset_code: "USD".into(), asset_scale: 2 },
        expires_at: None,
        metadata: None,
        created_at: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        updated_at: None,
        methods: None,
    };
    let page = PaginatedResponse {
        pagination: PageInfo { start_cursor: Some("s".into()), end_cursor: Some("e".into()), has_next_page: false, has_previous_page: false },
        result: vec![item],
    };
    serde_roundtrip(&page);
}
