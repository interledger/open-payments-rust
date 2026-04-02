use http::Request;

/// Creates the signature base string per RFC 9421 Section 2.5.
///
/// The signature base string is the input to the signing and verification
/// algorithms. It consists of the canonicalized component values followed
/// by the signature parameters line.
pub(crate) fn create_signature_base_string(
    request: &Request<Option<String>>,
    components: &[&str],
    created: i64,
    keyid: &str,
) -> String {
    let mut parts = Vec::new();

    for component in components {
        let value = match *component {
            "@method" => request.method().as_str(),
            "@target-uri" => &request.uri().to_string(),
            "authorization" => request
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            "content-digest" => request
                .headers()
                .get("Content-Digest")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            "content-length" => request
                .headers()
                .get("Content-Length")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            "content-type" => request
                .headers()
                .get("Content-Type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            _ => "",
        };
        parts.push(format!("\"{component}\": {value}"));
    }

    let sig_params = format!(
        "({});created={};keyid=\"{}\"",
        components.join(" "),
        created,
        keyid
    );
    parts.push(format!("\"@signature-params\": {sig_params}"));

    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{Method, Request, Uri};

    #[test]
    fn test_base_string_with_method_and_uri() {
        let mut request = Request::new(None);
        *request.method_mut() = Method::GET;
        *request.uri_mut() = Uri::from_static("https://example.com/resource");

        let base = create_signature_base_string(
            &request,
            &["@method", "@target-uri"],
            1618884473,
            "test-key",
        );

        assert!(base.contains("\"@method\": GET"));
        assert!(base.contains("\"@target-uri\": https://example.com/resource"));
        assert!(base.contains("\"@signature-params\": (@method @target-uri);created=1618884473;keyid=\"test-key\""));
    }

    #[test]
    fn test_base_string_with_content_headers() {
        let mut request = Request::new(Some("body".to_string()));
        *request.method_mut() = Method::POST;
        *request.uri_mut() = Uri::from_static("https://example.com/api");
        request
            .headers_mut()
            .insert("Content-Type", "application/json".parse().unwrap());
        request
            .headers_mut()
            .insert("Content-Length", "4".parse().unwrap());

        let base = create_signature_base_string(
            &request,
            &["@method", "content-type", "content-length"],
            1618884473,
            "key-1",
        );

        assert!(base.contains("\"content-type\": application/json"));
        assert!(base.contains("\"content-length\": 4"));
    }

    #[test]
    fn test_base_string_missing_header_defaults_to_empty() {
        let mut request = Request::new(None);
        *request.method_mut() = Method::GET;
        *request.uri_mut() = Uri::from_static("https://example.com");

        let base = create_signature_base_string(
            &request,
            &["@method", "authorization"],
            1618884473,
            "key-1",
        );

        assert!(base.contains("\"authorization\": "));
    }

    #[test]
    fn test_base_string_unknown_component_defaults_to_empty() {
        let mut request = Request::new(None);
        *request.method_mut() = Method::GET;
        *request.uri_mut() = Uri::from_static("https://example.com");

        let base = create_signature_base_string(
            &request,
            &["@method", "x-custom-header"],
            1618884473,
            "key-1",
        );

        assert!(base.contains("\"x-custom-header\": "));
    }
}
