//! # HTTP Request Building and Execution
//!
//! This module provides the internal HTTP request building and execution functionality
//! for the Open Payments client. It handles both authenticated and unauthenticated
//! requests, including HTTP message signature creation and content digest generation.
//!
//! ## Key Components
//!
//! - Generic HTTP request builder for constructing requests
//! - Request builders for authenticated and unauthenticated operations
//! - Internal request execution and signature handling
//!
//! ## Features
//!
//! - **Automatic Signature Creation**: Authenticated requests automatically include HTTP message signatures
//! - **Content Digest Generation**: SHA-512 content digests for request bodies
//! - **GNAP Authorization**: Support for Grant Negotiation and Authorization Protocol tokens
//! - **Error Handling**: Comprehensive error handling for HTTP and signature operations
use crate::client::AuthenticatedOpenPaymentsClient;
use crate::client::BaseClient;
use crate::http_signature::{create_signature_headers, SignOptions};
use crate::OpClientError;
use crate::Result;
use base64::engine::general_purpose;
use base64::Engine;
use http::{
    header::{HeaderName, HeaderValue},
    Method as HttpMethod, Request,
};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha512};

/// Generic HTTP request builder for Open Payments operations.
///
/// This struct provides a fluent interface for building HTTP requests with
/// optional body content. It's used internally by the client to construct
/// requests before execution.
///
/// ## Type Parameters
///
/// - `C` - The client type (authenticated or unauthenticated)
pub(crate) struct HttpRequest<'a, C> {
    /// Reference to the client that will execute this request.
    client: &'a C,
    /// HTTP method for the request.
    method: Method,
    /// Target URL for the request.
    url: String,
    /// Optional request body content.
    body: Option<String>,
}

impl<'a, C> HttpRequest<'a, C> {
    /// Creates a new HTTP request builder.
    ///
    /// ## Arguments
    ///
    /// * `client` - Reference to the client that will execute the request
    /// * `method` - HTTP method (GET, POST, PUT, DELETE, etc.)
    /// * `url` - Target URL for the request
    ///
    /// ## Returns
    ///
    /// Returns a new `HttpRequest` builder with no body content.
    pub fn new(client: &'a C, method: Method, url: String) -> Self {
        Self {
            client,
            method,
            url,
            body: None,
        }
    }

    /// Adds a body to the request.
    ///
    /// This method consumes the request builder and returns a new one with
    /// the specified body content. The body is typically JSON for Open Payments
    /// API requests.
    ///
    /// ## Arguments
    ///
    /// * `body` - The request body content as a string
    ///
    /// ## Returns
    ///
    /// Returns a new `HttpRequest` with the body content added.
    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
}

/// Type alias for authenticated HTTP requests.
///
/// This type represents HTTP requests that will be executed with authentication,
/// including HTTP message signatures and optional GNAP access tokens.
pub(crate) type AuthenticatedRequest<'a> = HttpRequest<'a, AuthenticatedOpenPaymentsClient>;

/// Type alias for unauthenticated HTTP requests.
///
/// This type represents HTTP requests that will be executed without authentication,
/// suitable for public endpoints that don't require signatures or tokens.
pub(crate) type UnauthenticatedRequest<'a> = HttpRequest<'a, Client>;

impl AuthenticatedRequest<'_> {
    /// Builds and executes an authenticated HTTP request.
    ///
    /// This method performs the following steps:
    /// 1. Builds the HTTP request with proper headers
    /// 2. Adds GNAP authorization header if a token is provided
    /// 3. Generates content digest and length headers for request bodies
    /// 4. Creates HTTP message signatures using the client's signing key
    /// 5. Executes the request and deserializes the response
    ///
    /// ## Arguments
    ///
    /// * `access_token` - Optional GNAP access token for authorization
    ///
    /// ## Returns
    ///
    /// Returns the deserialized response of type `T`, or an error if the request fails.
    ///
    /// ## Errors
    ///
    /// Returns an `OpClientError` with:
    /// - `description`: Human-readable error message
    /// - `status`: HTTP status text (for HTTP errors)
    /// - `code`: HTTP status code (for HTTP errors)
    /// - `validation_errors`: List of validation errors (if applicable)
    /// - `details`: Additional error details (if applicable)
    pub async fn build_and_execute<T: DeserializeOwned + 'static>(
        self,
        access_token: Option<&str>,
    ) -> Result<T> {
        let mut req = build_request(&self)?;

        if let Some(token) = access_token {
            req.headers_mut().insert(
                "Authorization",
                format!("GNAP {token}").parse().map_err(|e| {
                    OpClientError::header_parse(format!(
                        "Failed to parse authorization header: {e}"
                    ))
                })?,
            );
        }

        if let Some((content_length, content_digest)) = Self::create_content_headers(&self.body) {
            req.headers_mut().insert(
                "Content-Length",
                content_length.to_string().parse().map_err(|e| {
                    OpClientError::header_parse(format!("Failed to parse content length: {e}"))
                })?,
            );
            req.headers_mut().insert(
                "Content-Digest",
                content_digest.parse().map_err(|e| {
                    OpClientError::header_parse(format!("Failed to parse content digest: {e}"))
                })?,
            );
        }

        let (signature, signature_input) = Self::create_signature_headers(&self, &req)?;

        req.headers_mut().insert(
            "Signature",
            signature.parse().map_err(|e| {
                OpClientError::header_parse(format!("Failed to parse signature header: {e}"))
            })?,
        );
        req.headers_mut().insert(
            "Signature-Input",
            signature_input.parse().map_err(|e| {
                OpClientError::header_parse(format!("Failed to parse signature input header: {e}"))
            })?,
        );

        execute_request(&self.client.http_client, req).await
    }

    /// Creates HTTP message signature headers for the request.
    ///
    /// This method converts the reqwest request to an http::Request for signature
    /// creation, then generates the signature and signature-input headers using
    /// the client's signing key.
    ///
    /// ## Arguments
    ///
    /// * `req` - The reqwest request to sign
    ///
    /// ## Returns
    ///
    /// Returns a tuple of `(signature, signature_input)` strings, or an error if
    /// signature creation fails.
    fn create_signature_headers(&self, req: &reqwest::Request) -> Result<(String, String)> {
        // Convert to http::Request for signing
        let mut http_req = Request::new(self.body.clone());
        *http_req.method_mut() = HttpMethod::from_bytes(req.method().as_str().as_bytes())
            .map_err(|e| OpClientError::header_parse(format!("Converting HTTP method: {e}")))?;
        *http_req.uri_mut() = req
            .url()
            .as_str()
            .parse()
            .map_err(|e| OpClientError::header_parse(format!("Converting URL to URI: {e}")))?;

        for (key, value) in req.headers() {
            let header_name = HeaderName::from_bytes(key.as_str().as_bytes())
                .map_err(|e| OpClientError::header_parse(format!("Converting header name: {e}")))?;
            let header_value = HeaderValue::from_bytes(value.as_bytes()).map_err(|e| {
                OpClientError::header_parse(format!("Converting header value: {e}"))
            })?;
            http_req.headers_mut().insert(header_name, header_value);
        }

        // Create and return signature headers
        let options = SignOptions::new(
            &http_req,
            &self.client.signing_key,
            self.client.config.key_id.clone(),
        );
        let headers = create_signature_headers(options)
            .map_err(|e| OpClientError::signature(e.to_string()))?;

        Ok((headers.signature, headers.signature_input))
    }

    /// Creates content headers for request bodies.
    ///
    /// This method generates `Content-Length` and `Content-Digest` headers for
    /// requests with body content. The content digest uses SHA-512 hashing
    /// as required by the Open Payments specification.
    ///
    /// ## Arguments
    ///
    /// * `body` - Optional request body content
    ///
    /// ## Returns
    ///
    /// Returns `Some((content_length, content_digest))` if a body is present,
    /// or `None` if no body content.
    fn create_content_headers(body: &Option<String>) -> Option<(usize, String)> {
        match body {
            Some(body) => {
                let content_length = body.len();
                let mut hasher = Sha512::new();
                hasher.update(body.as_bytes());
                let digest = general_purpose::STANDARD.encode(hasher.finalize());
                let content_digest = format!("sha-512=:{digest}:");

                Some((content_length, content_digest))
            }
            None => None,
        }
    }
}

impl UnauthenticatedRequest<'_> {
    /// Builds and executes an unauthenticated HTTP request.
    ///
    /// This method builds and executes a request without authentication headers
    /// or signatures. It's suitable for public endpoints that don't require
    /// authentication.
    ///
    /// ## Returns
    ///
    /// Returns the deserialized response of type `T`, or an error if the request fails.
    ///
    /// ## Errors
    ///
    /// Returns an `OpClientError` with:
    /// - `description`: Human-readable error message
    /// - `status`: HTTP status text (for HTTP errors)
    /// - `code`: HTTP status code (for HTTP errors)
    /// - `validation_errors`: List of validation errors (if applicable)
    /// - `details`: Additional error details (if applicable)
    pub async fn build_and_execute<T: DeserializeOwned + 'static>(self) -> Result<T> {
        let req = build_request(&self)?;
        execute_request(self.client, req).await
    }
}

impl<C: BaseClient> BaseClient for HttpRequest<'_, C> {
    fn http_client(&self) -> &reqwest::Client {
        self.client.http_client()
    }
}

/// Builds a reqwest request from the HTTP request builder.
///
/// This function creates a reqwest request with the appropriate method, URL,
/// and body content. It also sets the `Content-Type` header to `application/json`.
///
/// ## Arguments
///
/// * `req` - The HTTP request builder
///
/// ## Returns
///
/// Returns a built reqwest request, or an error if the request cannot be built.
fn build_request<C: BaseClient>(req: &HttpRequest<C>) -> Result<reqwest::Request> {
    let mut builder = req
        .http_client()
        .request(req.method.clone(), &req.url)
        .header("Content-Type", "application/json");

    if let Some(body) = &req.body {
        builder = builder.body(body.clone());
    }

    builder
        .build()
        .map_err(|e| Box::new(OpClientError::from(e)))
}

/// Executes a reqwest request and deserializes the response.
///
/// This function handles the HTTP request execution, status code checking,
/// and response deserialization. It includes special handling for 204 No Content
/// responses.
///
/// ## Arguments
///
/// * `client` - The reqwest client to use for execution
/// * `req` - The reqwest request to execute
///
/// ## Returns
///
/// Returns the deserialized response of type `T`, or an error if the request fails.
///
/// ## Errors
///
/// Returns an `OpClientError` with:
/// - `description`: Human-readable error message
/// - `status`: HTTP status text (for HTTP errors)
/// - `code`: HTTP status code (for HTTP errors)
/// - `validation_errors`: List of validation errors (if applicable)
/// - `details`: Additional error details (if applicable)
async fn execute_request<T: DeserializeOwned + 'static>(
    client: &Client,
    req: reqwest::Request,
) -> Result<T> {
    let resp = client.execute(req).await.map_err(OpClientError::from)?;

    if !resp.status().is_success() {
        return Err(Box::new(OpClientError::http(
            "HTTP request failed".to_string(),
            Some(
                resp.status()
                    .canonical_reason()
                    .unwrap_or("Unknown")
                    .to_string(),
            ),
            Some(resp.status().as_u16()),
        )));
    }

    if resp.status() == reqwest::StatusCode::NO_CONTENT
        && std::any::TypeId::of::<T>() == std::any::TypeId::of::<()>()
    {
        return Ok(serde_json::from_str::<T>("null")
            .expect("Deserializing unit type from null should never fail"));
    }

    let result: T = resp.json().await.map_err(OpClientError::from)?;

    Ok(result)
}
