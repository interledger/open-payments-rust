use crate::OpClientError;
use crate::Result;
use url::Url;

pub fn get_resource_server_url(wallet_address_url: &str) -> Result<String> {
    let url = Url::parse(wallet_address_url).map_err(|e| OpClientError::Url(e))?;

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
        resource_url.set_path(&format!("/{}", resource_server_path));
    }

    Ok(resource_url.to_string())
}

pub fn join_url_paths(base_url: &str, path: &str) -> Result<String> {
    let mut url = Url::parse(base_url).map_err(|e| OpClientError::Url(e))?;

    if path.is_empty() {
        return Ok(url.to_string());
    }

    if !url.path().ends_with('/') {
        url.set_path(&format!("{}/", url.path()));
    }

    let joined_url = url.join(path).map_err(|e| OpClientError::Url(e))?;
    Ok(joined_url.to_string())
}
