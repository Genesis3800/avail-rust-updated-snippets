use avail_rust::prelude::*;

pub async fn read_data_by_block() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;
    let block_hash = new_h256_from_hex("0x94746ba186876d7407ee618d10cb6619befc59eeb173cacb00c14d1ff492fc58")?;

    let block = Block::new(&sdk.client, block_hash).await?;

    // All Block Blobs
    let blobs = block.data_submissions(Filter::default());

    // Printout All Block Blobs
    for blob in blobs {
        let blob_data = blob.to_ascii().unwrap();

        println!(
            "Tx Hash: {:?}, Tx Index: {}, Data: {:?}, App Id: {}, Tx Singer: {:?}",
            blob.tx_hash,
            blob.tx_index,
            blob_data,
            blob.app_id,
            blob.ss58address(),
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = read_data_by_block().await {
        eprintln!("Error: {:?}", e);
    }
}