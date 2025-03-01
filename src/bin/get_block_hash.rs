use avail_rust::prelude::*;

pub async fn get_block_hash() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // 1. Get the block hash for a specific block if argument is provided
    // 2. Get the latest block hash if no argument is provided
    let block_hash = rpc::chain::get_block_hash(&sdk.client, None).await?;

    println!("Latest Block Hash: {:?}", block_hash);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    get_block_hash().await?;
    Ok(())
}