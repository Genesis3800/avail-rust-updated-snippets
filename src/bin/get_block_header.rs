use avail_rust::prelude::*;

pub async fn get_block_header() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // 1. Gets the block header for a specific block if argument is provided
    // 2. Gets the latest block header if no argument is provided
    let block_hash = new_h256_from_hex("0x75a6c54bb5ea904e47fa151956992d7cf543bc7c936d78488e311db8e10397c1")?;

    // Get the latest block header
    let latest_block_header = rpc::chain::get_header(&sdk.client, None).await?;

    // Get the block header for a specific block
    let block_header = rpc::chain::get_header(&sdk.client, Some(block_hash)).await?;

    println!("Latest Block Header: {:?}", latest_block_header);
    println!("Block Header: {:?}", block_header);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    get_block_header().await?;
    Ok(())
}
