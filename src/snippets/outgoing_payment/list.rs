use open_payments::client::api::AuthenticatedResources;
use open_payments::client::utils::get_resource_server_url;
use open_payments::snippets::utils::{create_authenticated_client, get_env_var, load_env};

#[tokio::main]
async fn main() -> open_payments::client::Result<()> {
    load_env()?;

    let client = create_authenticated_client()?;

    let gnap_token = get_env_var("OUTGOING_PAYMENT_ACCESS_TOKEN")?;

    let wallet_address_url = get_env_var("WALLET_ADDRESS_URL")?;
    let resource_server_url = get_resource_server_url(&wallet_address_url)?;

    let response = client
        .outgoing_payments()
        .list(
            &resource_server_url,
            &wallet_address_url,
            None,
            Some(10),
            None,
            Some(&gnap_token),
        )
        .await?;

    println!("Outgoing payments: {:#?}", response.result);
    println!("Pagination info: {:#?}", response.pagination);

    if response.pagination.has_next_page {
        if let Some(end_cursor) = response.pagination.end_cursor {
            let next_page = client
                .outgoing_payments()
                .list(
                    &resource_server_url,
                    &wallet_address_url,
                    Some(&end_cursor),
                    Some(10),
                    None,
                    Some(&gnap_token),
                )
                .await?;
            println!("Next page of outgoing payments: {:#?}", next_page.result);
        }
    }

    Ok(())
}
