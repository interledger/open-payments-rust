//! # Open Payments Client Utilities
//!
//! This module provides utility functions for URL manipulation and resource server
//! URL extraction that are commonly used throughout the Open Payments client.
//!
//! ## Functions
//!
//! - [`get_resource_server_url`] - Extract resource server URL from wallet address
//! - [`join_url_paths`] - Safely join URL paths with proper handling
//!
use crate::OpClientError;
use crate::Result;
use url::Url;

/// Extracts the resource server URL from a wallet address URL.
///
/// This function takes a wallet address URL
/// and extracts the base resource server URL by removing the wallet address portion.
///
/// ## Arguments
///
/// * `wallet_address_url` - The complete wallet address URL path
///
/// ## Returns
///
/// Returns the resource server base URL as a string, or an error if the URL cannot be parsed.
///
/// ## Errors
///
/// Returns an `OpClientError` with:
/// - `description`: Human-readable error message
/// - `status`: HTTP status text (for HTTP errors)
/// - `code`: HTTP status code (for HTTP errors)
/// - `validation_errors`: List of validation errors (if applicable)
/// - `details`: Additional error details (if applicable)
pub fn get_resource_server_url(wallet_address_url: &str) -> Result<String> {
    let url = Url::parse(wallet_address_url).map_err(OpClientError::from)?;

    let path_segments: Vec<&str> = url
        .path_segments()
        .map(|segments| segments.collect())
        .unwrap_or_default();

    // Remove the last segment (wallet address) to get the resource server URL
    let resource_server_path = if path_segments.len() > 1 {
        path_segments[..path_segments.len() - 1].join("/")
    } else {
        String::new()
    };

    let mut resource_url = url;

    if resource_server_path.is_empty() {
        resource_url.set_path("/");
    } else {
        resource_url.set_path(&format!("/{resource_server_path}"));
    }

    Ok(resource_url.to_string())
}

/// Safely joins a base URL with a path component.
///
/// This function ensures proper URL path joining by handling trailing slashes
/// and ensuring the resulting URL is valid. It's useful for constructing
/// API endpoint URLs from base URLs and relative paths.
///
/// ## Arguments
///
/// * `base_url` - The base URL to join with the path
/// * `path` - The path component to append to the base URL
///
/// ## Returns
///
/// Returns the joined URL as a string, or an error if URL parsing fails.
///
/// ## Errors
///
/// Returns an `OpClientError` with:
/// - `description`: Human-readable error message
/// - `status`: HTTP status text (for HTTP errors)
/// - `code`: HTTP status code (for HTTP errors)
/// - `validation_errors`: List of validation errors (if applicable)
/// - `details`: Additional error details (if applicable)
pub fn join_url_paths(base_url: &str, path: &str) -> Result<String> {
    let mut url = Url::parse(base_url).map_err(OpClientError::from)?;

    if path.is_empty() {
        return Ok(url.to_string());
    }

    if !url.path().ends_with('/') {
        url.set_path(&format!("{}/", url.path()));
    }

    let joined_url = url.join(path).map_err(OpClientError::from)?;
    Ok(joined_url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── get_resource_server_url ──────────────────────────────────────

    #[test]
    fn test_resource_server_url_single_segment() {
        let url = get_resource_server_url("https://ilp.rafiki.money/alice").unwrap();
        assert_eq!(url, "https://ilp.rafiki.money/");
    }

    #[test]
    fn test_resource_server_url_nested_path() {
        let url =
            get_resource_server_url("https://ilp.rafiki.money/accounts/alice").unwrap();
        assert_eq!(url, "https://ilp.rafiki.money/accounts");
    }

    #[test]
    fn test_resource_server_url_root_path() {
        let url = get_resource_server_url("https://ilp.rafiki.money/").unwrap();
        assert_eq!(url, "https://ilp.rafiki.money/");
    }

    #[test]
    fn test_resource_server_url_deeply_nested() {
        let url =
            get_resource_server_url("https://example.com/a/b/c/wallet").unwrap();
        assert_eq!(url, "https://example.com/a/b/c");
    }

    #[test]
    fn test_resource_server_url_invalid_url() {
        let result = get_resource_server_url("not-a-url");
        assert!(result.is_err());
    }

    // ── join_url_paths ──────────────────────────────────────────────

    #[test]
    fn test_join_simple_path() {
        let url = join_url_paths("https://example.com", "incoming-payments").unwrap();
        assert_eq!(url, "https://example.com/incoming-payments");
    }

    #[test]
    fn test_join_with_trailing_slash() {
        let url = join_url_paths("https://example.com/", "incoming-payments").unwrap();
        assert_eq!(url, "https://example.com/incoming-payments");
    }

    #[test]
    fn test_join_with_existing_path() {
        let url =
            join_url_paths("https://example.com/api/v1", "outgoing-payments").unwrap();
        assert_eq!(url, "https://example.com/api/v1/outgoing-payments");
    }

    #[test]
    fn test_join_empty_path() {
        let url = join_url_paths("https://example.com/api", "").unwrap();
        assert_eq!(url, "https://example.com/api");
    }

    #[test]
    fn test_join_invalid_base_url() {
        let result = join_url_paths("not-a-url", "path");
        assert!(result.is_err());
    }

    #[test]
    fn test_join_with_nested_path() {
        let url = join_url_paths(
            "https://ilp.rafiki.money",
            "incoming-payments/12345",
        )
        .unwrap();
        assert_eq!(
            url,
            "https://ilp.rafiki.money/incoming-payments/12345"
        );
    }
}
