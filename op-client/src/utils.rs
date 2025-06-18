pub fn get_resource_server_url(wallet_address_url: &str) -> Result<String, url::ParseError> {
    let url = url::Url::parse(wallet_address_url)?;
    Ok(format!(
        "{}://{}",
        url.scheme(),
        url.host_str().ok_or(url::ParseError::EmptyHost)?
    ))
}
