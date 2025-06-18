use crate::OpClientError;
use crate::Result;
use crate::client::AuthenticatedOpenPaymentsClient;
use crate::client::BaseClient;
use base64::Engine;
use base64::engine::general_purpose;
use http::{
    Method as HttpMethod, Request,
    header::{HeaderName, HeaderValue},
};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha512};
use tracing::debug;

pub struct HttpRequest<'a, C> {
    client: &'a C,
    method: Method,
    url: String,
    body: Option<String>,
}

impl<'a, C> HttpRequest<'a, C> {
    pub fn new(client: &'a C, method: Method, url: String) -> Self {
        Self {
            client,
            method,
            url,
            body: None,
        }
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
}

pub type AuthenticatedRequest<'a> = HttpRequest<'a, AuthenticatedOpenPaymentsClient>;
pub type UnauthenticatedRequest<'a> = HttpRequest<'a, Client>;

impl<'a> AuthenticatedRequest<'a> {
    pub async fn build_and_execute<T: DeserializeOwned + 'static>(self) -> Result<T> {
        let mut req = build_request(&self)?;

        if let Some(token) = &self.client.access_token {
            req.headers_mut().insert(
                "Authorization",
                format!("GNAP {}", token).parse().map_err(|e| {
                    OpClientError::HeaderParse(format!(
                        "Failed to parse authorization header: {}",
                        e
                    ))
                })?,
            );
        }

        if let Some((content_length, content_digest)) = Self::create_content_headers(&self.body) {
            req.headers_mut().insert(
                "Content-Length",
                content_length.to_string().parse().map_err(|e| {
                    OpClientError::HeaderParse(format!("Failed to parse content length: {}", e))
                })?,
            );
            req.headers_mut().insert(
                "Content-Digest",
                content_digest.parse().map_err(|e| {
                    OpClientError::HeaderParse(format!("Failed to parse content digest: {}", e))
                })?,
            );
        }

        let (signature, signature_input) = Self::create_signature_headers(&self, &req)?;

        req.headers_mut().insert(
            "Signature",
            signature.parse().map_err(|e| {
                OpClientError::HeaderParse(format!("Failed to parse signature header: {}", e))
            })?,
        );
        req.headers_mut().insert(
            "Signature-Input",
            signature_input.parse().map_err(|e| {
                OpClientError::HeaderParse(format!("Failed to parse signature input header: {}", e))
            })?,
        );

        execute_request(&self.client.http_client, req).await
    }

    fn create_signature_headers(&self, req: &reqwest::Request) -> Result<(String, String)> {
        // Convert to http::Request for signing
        let mut http_req = Request::new(self.body.clone());
        *http_req.method_mut() = HttpMethod::from_bytes(req.method().as_str().as_bytes())
            .map_err(|e| OpClientError::HeaderParse(format!("Converting HTTP method: {}", e)))?;
        *http_req.uri_mut() = req
            .url()
            .as_str()
            .parse()
            .map_err(|e| OpClientError::HeaderParse(format!("Converting URL to URI: {}", e)))?;

        for (key, value) in req.headers() {
            let header_name = HeaderName::from_bytes(key.as_str().as_bytes()).map_err(|e| {
                OpClientError::HeaderParse(format!("Converting header name: {}", e))
            })?;
            let header_value = HeaderValue::from_bytes(value.as_bytes()).map_err(|e| {
                OpClientError::HeaderParse(format!("Converting header value: {}", e))
            })?;
            http_req.headers_mut().insert(header_name, header_value);
        }

        // Create and return signature headers
        let options = http_signature_utils::SignOptions::new(
            &http_req,
            &self.client.signing_key,
            self.client.config.key_id.clone(),
        );
        let headers = http_signature_utils::create_signature_headers(options)
            .map_err(|e| OpClientError::Signature(e.to_string()))?;

        Ok((headers.signature, headers.signature_input))
    }

    fn create_content_headers(body: &Option<String>) -> Option<(usize, String)> {
        match body {
            Some(body) => {
                let content_length = body.len();
                let mut hasher = Sha512::new();
                hasher.update(body.as_bytes());
                let digest = general_purpose::STANDARD.encode(hasher.finalize());
                let content_digest = format!("sha-512=:{}:", digest);
                debug!("Content-Digest: {}", content_digest);

                Some((content_length, content_digest))
            }
            None => None,
        }
    }
}

impl<'a> UnauthenticatedRequest<'a> {
    pub async fn build_and_execute<T: DeserializeOwned + 'static>(self) -> Result<T> {
        let req = build_request(&self)?;
        execute_request(self.client, req).await
    }
}

impl<'a, C: BaseClient> BaseClient for HttpRequest<'a, C> {
    fn http_client(&self) -> &reqwest::Client {
        self.client.http_client()
    }
}

fn build_request<C: BaseClient>(req: &HttpRequest<C>) -> Result<reqwest::Request> {
    let mut builder = req
        .http_client()
        .request(req.method.clone(), &req.url)
        .header("Content-Type", "application/json");

    if let Some(body) = &req.body {
        builder = builder.body(body.clone());
    }

    Ok(builder.build().map_err(OpClientError::from)?)
}

async fn execute_request<T: DeserializeOwned + 'static>(
    client: &Client,
    req: reqwest::Request,
) -> Result<T> {
    let resp = client.execute(req).await.map_err(OpClientError::from)?;

    if !resp.status().is_success() {
        return Err(OpClientError::Http(format!(
            "Request failed: HTTP {}",
            resp.status()
        )));
    }

    if resp.status() == reqwest::StatusCode::NO_CONTENT {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<()>() {
            return Ok(serde_json::from_str::<T>("null")
                .expect("Deserializing unit type from null should never fail"));
        }
    }

    let result: T = resp.json().await.map_err(OpClientError::from)?;

    Ok(result)
}
