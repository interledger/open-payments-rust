use crate::integration::common::TestSetup;
use open_payments::client::{AuthenticatedResources, UnauthenticatedResources};
use open_payments::types::{
    AccessItem, AccessTokenRequest, Amount, ContinueResponse, CreateOutgoingPaymentRequest,
    CreateQuoteRequest, GrantRequest, GrantResponse, IncomingPaymentAction, IncomingPaymentRequest,
    InteractFinish, InteractRequest, OutgoingPaymentAction, PaymentMethodType, QuoteAction,
    Receiver,
};
use thirtyfour::prelude::*;

#[tokio::test]
async fn test_outgoing_payment_flow_with_interaction() {
    async fn webdriver_ready(base_url: &str) -> bool {
        let status_url = format!("{}/status", base_url.trim_end_matches('/'));
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .ok();
        if let Some(c) = client {
            if let Ok(resp) = c.get(&status_url).send().await {
                return resp.status().is_success();
            }
        }
        false
    }
    // Determine WebDriver URL and skip if not reachable
    let webdriver_url =
        std::env::var("WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:4444".into());
    if !webdriver_ready(&webdriver_url).await {
        eprintln!(
            "Skipping test_outgoing_payment_flow_with_interaction: WebDriver not available at {webdriver_url}"
        );
        return;
    }
    // Skip test if integration .env is missing
    let test_setup = match TestSetup::new().await {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Skipping test_outgoing_payment_flow_with_interaction: {err}");
            return;
        }
    };

    let wallet_address = test_setup
        .auth_client
        .wallet_address()
        .get(&test_setup.wallet_address)
        .await
        .expect("Failed to get wallet address");

    // Create an incoming payment (receiver) using a non-interactive grant
    let ip_grant_request = GrantRequest::new(
        AccessTokenRequest {
            access: vec![AccessItem::IncomingPayment {
                actions: vec![IncomingPaymentAction::Create, IncomingPaymentAction::Read],
                identifier: None,
            }],
        },
        None,
    );
    let ip_grant = test_setup
        .auth_client
        .grant()
        .request(&wallet_address.auth_server, &ip_grant_request)
        .await
        .expect("Failed to request incoming payment grant");

    let ip_access_token = match ip_grant {
        GrantResponse::WithToken { access_token, .. } => access_token.value,
        GrantResponse::WithInteraction { .. } => {
            panic!("Unexpected interaction for incoming payment creation")
        }
    };

    let incoming_req = IncomingPaymentRequest {
        wallet_address: wallet_address.id.clone(),
        incoming_amount: Some(Amount {
            value: "1000".into(),
            asset_code: wallet_address.asset_code.clone(),
            asset_scale: wallet_address.asset_scale,
        }),
        metadata: None,
        expires_at: Some(chrono::Utc::now() + chrono::Duration::minutes(30)),
    };

    let incoming_payment = test_setup
        .auth_client
        .incoming_payments()
        .create(
            &test_setup.resource_server_url,
            &incoming_req,
            Some(&ip_access_token),
        )
        .await
        .expect("Failed to create incoming payment");

    // Helper to perform the browser interaction and return the outgoing payment token
    async fn perform_interaction_and_continue(
        driver_url: &str,
        redirect: &str,
        consent_selector: &str,
        continue_uri: &str,
        continue_token: &str,
        client: &open_payments::client::AuthenticatedClient,
        test_setup: &TestSetup,
    ) -> Option<String> {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new(driver_url, caps)
            .await
            .expect("Start webdriver");
        driver
            .set_page_load_timeout(std::time::Duration::from_secs(20))
            .await
            .ok();
        driver
            .set_implicit_wait_timeout(std::time::Duration::from_secs(10))
            .await
            .ok();
        driver.goto(redirect).await.expect("Navigate to redirect");
        if let Ok(url_now) = driver.current_url().await {
            println!("Navigated to: {url_now}");
        }

        // If we're on the wallet login, attempt to log in using env creds
        if let Ok(url_now) = driver.current_url().await {
            let on_login = url_now.as_str().contains("/auth/login");
            if on_login {
                let email = test_setup.test_wallet_email.clone();
                let password = test_setup.test_wallet_password.clone();
                if let (Some(email), Some(password)) = (email, password) {
                    let email_input = {
                        let mut found = None;
                        for _ in 0..100 {
                            if let Ok(e) = driver
                                .find(By::Css("input[type='email']".to_string()))
                                .await
                            {
                                found = Some(e);
                                break;
                            }
                            if let Ok(e) = driver
                                .find(By::Css("input[name='email']".to_string()))
                                .await
                            {
                                found = Some(e);
                                break;
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        }
                        found.expect("Email input not found")
                    };
                    email_input.clear().await.ok();
                    email_input.send_keys(email).await.expect("Type email");

                    let password_input = {
                        let mut found = None;
                        for _ in 0..100 {
                            if let Ok(e) = driver
                                .find(By::Css("input[type='password']".to_string()))
                                .await
                            {
                                found = Some(e);
                                break;
                            }
                            if let Ok(e) = driver
                                .find(By::Css("input[name='password']".to_string()))
                                .await
                            {
                                found = Some(e);
                                break;
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        }
                        found.expect("Password input not found")
                    };
                    password_input.clear().await.ok();
                    password_input
                        .send_keys(password)
                        .await
                        .expect("Type password");

                    let submit_btn = {
                        let mut found = None;
                        for _ in 0..100 {
                            if let Ok(b) = driver
                                .find(By::Css("button[type='submit']".to_string()))
                                .await
                            {
                                found = Some(b);
                                break;
                            }
                            if let Ok(b) = driver.find(By::XPath("//button[normalize-space()='Sign in' or normalize-space()='Log in']".to_string())).await { found = Some(b); break; }
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        }
                        found.expect("Submit button not found")
                    };
                    submit_btn.click().await.expect("Click submit");

                    // Wait to be redirected to interact page showing Accept
                    for _ in 0..150 {
                        if let Ok(src) = driver.source().await {
                            if src.contains("Accept") {
                                break;
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                } else {
                    eprintln!("Skipping: TEST_WALLET_EMAIL or TEST_WALLET_PASSWORD not set; login required for consent page");
                    let _ = driver.quit().await;
                    return None;
                }
            }
        }

        let btn = match driver.find(By::Css(consent_selector.to_string())).await {
            Ok(elem) => elem,
            Err(_) => driver
                .find(By::XPath(
                    "//*[normalize-space()='Accept' and (self::button or @role='button')]",
                ))
                .await
                .expect("Find consent button by text"),
        };
        btn.click().await.expect("Click consent");

        let mut current_url = String::new();
        for _ in 0..100 {
            // ~10s
            let url_now = driver.current_url().await.expect("Get current url");
            let url_now_str = url_now.as_str().to_string();
            if url_now_str.contains("interact_ref=") {
                current_url = url_now_str;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        let _ = driver.quit().await;
        let interact_ref = url::Url::parse(&current_url)
            .ok()
            .and_then(|u| {
                u.query_pairs()
                    .find(|(k, _)| k == "interact_ref")
                    .map(|(_, v)| v.to_string())
            })
            .expect("interact_ref not found in URL");
        let cont = client
            .grant()
            .continue_grant(continue_uri, &interact_ref, Some(continue_token))
            .await
            .expect("Continue grant");
        match cont {
            ContinueResponse::WithToken { access_token, .. } => Some(access_token.value),
            _ => panic!("Expected token after grant continuation"),
        }
    }

    // Request grant for quote, then create quote with that token
    let quote_grant_req = GrantRequest::new(
        AccessTokenRequest {
            access: vec![AccessItem::Quote {
                actions: vec![QuoteAction::Create, QuoteAction::Read],
            }],
        },
        None,
    );

    let quote_grant = test_setup
        .auth_client
        .grant()
        .request(&wallet_address.auth_server, &quote_grant_req)
        .await
        .expect("Request quote grant");
    let quote_token = match quote_grant {
        GrantResponse::WithToken { access_token, .. } => access_token.value,
        GrantResponse::WithInteraction { .. } => {
            panic!("Unexpected interaction required for quote grant")
        }
    };

    let quote_req = CreateQuoteRequest::FixedReceiveAmountQuote {
        wallet_address: wallet_address.id.clone(),
        receiver: Receiver(incoming_payment.id.clone()),
        method: PaymentMethodType::Ilp,
        receive_amount: Amount {
            value: "1000".into(),
            asset_code: wallet_address.asset_code.clone(),
            asset_scale: wallet_address.asset_scale,
        },
    };
    let quote = test_setup
        .auth_client
        .quotes()
        .create(
            &test_setup.resource_server_url,
            &quote_req,
            Some(&quote_token),
        )
        .await
        .expect("Create quote");

    // Request grant for outgoing payment, then create it
    let consent_selector =
        std::env::var("CONSENT_SELECTOR").unwrap_or_else(|_| "button[aria-label='accept']".into());
    let op_interact = InteractRequest {
        start: vec!["redirect".into()],
        finish: Some(InteractFinish {
            method: "redirect".into(),
            uri: "http://localhost/callback".into(),
            nonce: "op-nonce".into(),
        }),
    };
    let op_grant_req = GrantRequest::new(
        AccessTokenRequest {
            access: vec![AccessItem::OutgoingPayment {
                actions: vec![
                    OutgoingPaymentAction::Create,
                    OutgoingPaymentAction::Read,
                    OutgoingPaymentAction::List,
                ],
                identifier: wallet_address.id.clone(),
                limits: None,
            }],
        },
        Some(op_interact),
    );
    let op_grant = test_setup
        .auth_client
        .grant()
        .request(&wallet_address.auth_server, &op_grant_req)
        .await
        .expect("Request outgoing payment grant");

    let op_token = match op_grant {
        GrantResponse::WithInteraction {
            interact,
            continue_,
        } => {
            perform_interaction_and_continue(
                &webdriver_url,
                &interact.redirect,
                &consent_selector,
                &continue_.uri,
                &continue_.access_token.value,
                &test_setup.auth_client,
                &test_setup,
            )
            .await
        }
        GrantResponse::WithToken { access_token, .. } => Some(access_token.value),
    }
    .expect("Get outgoing payment token");

    let req = CreateOutgoingPaymentRequest::FromQuote {
        wallet_address: wallet_address.id.clone(),
        quote_id: quote.id,
        metadata: None,
    };
    test_setup
        .auth_client
        .outgoing_payments()
        .create(&test_setup.resource_server_url, &req, Some(&op_token))
        .await
        .expect("Failed to create outgoing payment");
}
