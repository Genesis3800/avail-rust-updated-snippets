use avail_rust::prelude::*;

pub async fn staking_active_era() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    let block_hash = sdk.client.best_block_hash().await?;
    let storage = sdk.client.storage().at(block_hash);
    let address = avail::storage().staking().active_era();
    let result = storage.fetch(&address).await?;

    dbg!(result);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = staking_active_era().await {
        eprintln!("Error: {:?}", e);
    }
}