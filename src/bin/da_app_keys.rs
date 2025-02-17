use avail_rust::prelude::*;
use avail::data_availability::storage::types::app_keys::Param0;

pub async fn da_app_keys() -> Result<(), ClientError> {
    
    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    let key = String::from("My Application Key").as_bytes().to_vec();
    let key = Param0 { 0: key };

    let block_hash = sdk.client.best_block_hash().await?;
    let storage = sdk.client.storage().at(block_hash);
    let address = avail::storage().data_availability().app_keys(key);
    let result = storage.fetch(&address).await?;

    dbg!(result);

    Ok(())
}

// Add a main function to call da_app_keys
#[tokio::main]
async fn main() {
    if let Err(e) = da_app_keys().await {
        eprintln!("Error: {:?}", e);
    }
}