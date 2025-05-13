use crate::client::AuthenticatedOpenPaymentsClient;
use anyhow::{Context, Result};
use base64::engine::general_purpose;
use base64::Engine;
use http::{
    header::{HeaderName, HeaderValue},
    Method as HttpMethod, Request,
};
use reqwest::{Client, Method, Request as ReqwestRequest};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha512};

pub struct AuthenticatedRequest<'a> {
    client: &'a AuthenticatedOpenPaymentsClient,
    method: Method,
    url: String,
    body: Option<String>,
}

impl<'a> AuthenticatedRequest<'a> {
    pub fn new(client: &'a AuthenticatedOpenPaymentsClient, method: Method, url: String) -> Self {
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

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut builder = self
            .client
            .http_client
            .request(self.method.clone(), &self.url)
            .header("Content-Type", "application/json");

        if let Some(token) = &self.client.access_token {
            builder = builder.header("Authorization", format!("GNAP {}", token));
        }

        if let Some(body) = &self.body {
            builder = builder.body(body.clone());
        }

        let mut req = builder
            .build()
            .with_context(|| format!("Failed to build request for URL: {}", self.url))?;

        if let Some(body) = &self.body {
            let content_length = body.len();
            req.headers_mut().insert(
                "Content-Length",
                content_length.to_string().parse().unwrap(),
            );

            let mut hasher = Sha512::new();
            hasher.update(body.as_bytes());
            let digest = general_purpose::STANDARD.encode(hasher.finalize());
            let content_digest = format!("sha-512=:{}:", digest);
            println!("Content-Digest: {}", content_digest);
            req.headers_mut()
                .insert("Content-Digest", content_digest.parse().unwrap());
        }

        // Convert to http::Request for signing
        let mut http_req = Request::new(self.body);
        *http_req.method_mut() = HttpMethod::from_bytes(req.method().as_str().as_bytes())
            .with_context(|| "Converting HTTP method")?;
        *http_req.uri_mut() = req
            .url()
            .as_str()
            .parse()
            .with_context(|| "Converting URL to URI")?;

        for (key, value) in req.headers() {
            let header_name = HeaderName::from_bytes(key.as_str().as_bytes())
                .with_context(|| "Converting header name")?;
            let header_value = HeaderValue::from_bytes(value.as_bytes())
                .with_context(|| "Converting header value")?;
            http_req.headers_mut().insert(header_name, header_value);
        }

        // Create and add signature headers
        let options = http_signature_utils::SignOptions::new(
            &http_req,
            &self.client.signing_key,
            self.client.config.key_id.clone(),
        );
        let headers = http_signature_utils::create_signature_headers(options)
            .await
            .with_context(|| "Failed to create signature headers")?;

        req.headers_mut()
            .insert("Signature", headers.signature.parse().unwrap());
        req.headers_mut()
            .insert("Signature-Input", headers.signature_input.parse().unwrap());

        let resp = self
            .client
            .http_client
            .execute(req)
            .await
            .with_context(|| format!("Executing request to {}", self.url))?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("Request failed: HTTP {}", resp.status()));
        }

        let result: T = resp.json().await.with_context(|| "Parsing response")?;

        Ok(result)
    }
}

pub struct UnauthenticatedRequest {
    client: Client,
    method: Method,
    url: String,
    body: Option<String>,
}

impl UnauthenticatedRequest {
    pub fn new(client: Client, method: Method, url: String) -> Self {
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

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let mut builder = self
            .client
            .request(self.method.clone(), &self.url)
            .header("Content-Type", "application/json");

        if let Some(body) = &self.body {
            builder = builder.body(body.clone());
        }

        let req = builder
            .build()
            .with_context(|| format!("Failed to build request for URL: {}", self.url))?;

        let resp = self
            .client
            .execute(req)
            .await
            .with_context(|| format!("Executing request to {}", self.url))?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("Request failed: HTTP {}", resp.status()));
        }

        let result: T = resp.json().await.with_context(|| "Parsing response")?;

        Ok(result)
    }
}
