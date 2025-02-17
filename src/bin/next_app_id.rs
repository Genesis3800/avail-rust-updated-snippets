use avail_rust::prelude::*;

pub async fn da_next_app_id() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    let block_hash = sdk.client.best_block_hash().await?;
    let storage = sdk.client.storage().at(block_hash);
    let address = avail::storage().data_availability().next_app_id();
    let result = storage.fetch_or_default(&address).await?;

    println!("Next available App ID is: {:?}", result);
    Ok(())
}

// Add a main function to call da_next_app_id
#[tokio::main]
async fn main() {
    if let Err(e) = da_next_app_id().await {
        eprintln!("Error: {:?}", e);
    }
}