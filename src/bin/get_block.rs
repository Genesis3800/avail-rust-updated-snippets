use avail_rust::prelude::*;

pub async fn get_block() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // 1. Get the block for a specific block if argument is provided
    // 2. Get the latest block if no argument is provided
    let block_hash = new_h256_from_hex("0x75a6c54bb5ea904e47fa151956992d7cf543bc7c936d78488e311db8e10397c1")?;

    // Get the latest block
    let latest_block = rpc::chain::get_block(&sdk.client, None).await?;

    // Get the block for a specific block
    let specific_block = rpc::chain::get_block(&sdk.client, Some(block_hash)).await?;

    println!("Latest Block: {:?}", latest_block);

    // println!(" Specific Block: {:?}", block);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    get_block().await?;
    Ok(())
}
