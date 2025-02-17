use avail_rust::prelude::*;
use core::str::FromStr;

pub async fn system_account() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;


    let account_id = AccountId::from_str("5CqgQkrDcdg5QrtuxT3H7WszrqgrBMhdwRbmMVXQzc4VSiEg").unwrap();

    let block_hash = sdk.client.best_block_hash().await?;
    let storage = sdk.client.storage().at(block_hash);
    let address = avail::storage().system().account(account_id);
    let result = storage.fetch(&address).await?;

    if let Some(account) = result {
        println!("Consumers: {}", account.consumers);
        println!("Data: {:?}", account.data);
        println!("Nonce: {}", account.nonce);
        println!("Providers: {}", account.providers);
        println!("Sufficients: {}", account.sufficients);
    }

    Ok(())
}

// Add a main function to call system_account
#[tokio::main]
async fn main() {
    if let Err(e) = system_account().await {
        eprintln!("Error: {:?}", e);
    }
}