use crate::client::AuthenticatedOpenPaymentsClient;
use crate::request::AuthenticatedRequest;
use crate::types::{
    AccessItem, Client, ContinueRequest, ContinueResponse, GrantRequest, GrantResponse,
    JsonWebKey,
};
use crate::OpClientError;
use crate::Result;
use reqwest::Method;

pub(crate) async fn request_grant(
    client: &AuthenticatedOpenPaymentsClient,
    auth_url: &str,
    grant: &GrantRequest,
    client_override: Option<&JsonWebKey>,
) -> Result<GrantResponse> {
    validate_grant_request(grant)?;

    let client_id = match client_override {
        Some(jwk) => Client::Jwk { jwk: jwk.clone() },
        None => Client::WalletAddressUrl(client.config.wallet_address_url.clone()),
    };

    let grant_with_client = GrantRequest {
        client: client_id,
        ..grant.clone()
    };
    let body = serde_json::to_string(&grant_with_client).map_err(OpClientError::from)?;

    AuthenticatedRequest::new(client, Method::POST, auth_url.to_string())
        .with_body(body)
        .build_and_execute(None)
        .await
}

fn validate_grant_request(grant: &GrantRequest) -> Result<()> {
    if grant.access_token.is_none() && grant.subject.is_none() {
        return Err(Box::new(OpClientError::validation(
            "Invalid Grant Request",
            vec![
                "Grant request must include at least one of \"access_token\" or \"subject\"."
                    .into(),
            ],
        )));
    }

    if let Some(access_token) = &grant.access_token {
        for item in &access_token.access {
            if let AccessItem::OutgoingPayment { limits: Some(limits), .. } = item {
                if limits.debit_amount.is_some() && limits.receive_amount.is_some() {
                    return Err(Box::new(OpClientError::validation(
                        "Invalid Grant Request",
                        vec![
                            "Only one of \"debitAmount\" or \"receiveAmount\" may be specified."
                                .into(),
                        ],
                    )));
                }
            }
        }
    }

    Ok(())
}

pub(crate) async fn continue_grant(
    client: &AuthenticatedOpenPaymentsClient,
    continue_uri: &str,
    interact_ref: &str,
    access_token: Option<&str>,
) -> Result<ContinueResponse> {
    let body = serde_json::to_string(&ContinueRequest {
        interact_ref: Some(interact_ref.to_string()),
    })
    .map_err(OpClientError::from)?;

    AuthenticatedRequest::new(client, Method::POST, continue_uri.to_string())
        .with_body(body)
        .build_and_execute(access_token)
        .await
}

pub(crate) async fn cancel_grant(
    client: &AuthenticatedOpenPaymentsClient,
    continue_uri: &str,
    access_token: Option<&str>,
) -> Result<()> {
    AuthenticatedRequest::new(client, Method::DELETE, continue_uri.to_string())
        .build_and_execute(access_token)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AccessTokenRequest, Amount, IncomingPaymentAction, LimitsOutgoing, OutgoingPaymentAction,
    };

    #[test]
    fn rejects_grant_without_access_token_or_subject() {
        let grant = GrantRequest {
            access_token: None,
            client: Client::WalletAddressUrl(String::new()),
            interact: None,
            subject: None,
        };
        let err = validate_grant_request(&grant).unwrap_err();
        assert!(err.description.contains("Invalid Grant Request"));
    }

    #[test]
    fn rejects_outgoing_limits_with_both_amounts() {
        let grant = GrantRequest::new(
            AccessTokenRequest {
                access: vec![AccessItem::OutgoingPayment {
                    actions: vec![OutgoingPaymentAction::Create],
                    identifier: "https://wallet.example/alice".into(),
                    limits: Some(LimitsOutgoing {
                        receiver: None,
                        debit_amount: Some(Amount {
                            value: "1".into(),
                            asset_code: "USD".into(),
                            asset_scale: 2,
                        }),
                        receive_amount: Some(Amount {
                            value: "1".into(),
                            asset_code: "USD".into(),
                            asset_scale: 2,
                        }),
                        interval: None,
                    }),
                }],
            },
            None,
        );
        let err = validate_grant_request(&grant).unwrap_err();
        assert!(err
            .validation_errors
            .as_ref()
            .unwrap()
            .iter()
            .any(|e| e.contains("debitAmount")));
    }

    #[test]
    fn accepts_incoming_payment_grant() {
        let grant = GrantRequest::new(
            AccessTokenRequest {
                access: vec![AccessItem::IncomingPayment {
                    actions: vec![IncomingPaymentAction::Create],
                    identifier: None,
                }],
            },
            None,
        );
        assert!(validate_grant_request(&grant).is_ok());
    }
}
