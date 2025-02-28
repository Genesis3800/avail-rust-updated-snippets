use avail_rust::prelude::*;

pub async fn read_data_by_appid() -> Result<(), ClientError> {

    // Create a new SDK instance
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;
    let block_hash = new_h256_from_hex("0xd5f95593d91a581d7ce7b8717789298345be4be47e75ba93e7159cfe23083a7b")?;

    let block = Block::new(&sdk.client, block_hash).await?;

    // All Block Blobs by App Id
    let app_id = 89;
    let blobs = block.data_submissions(Filter::new().app_id(app_id));

    // Printout All Block Blobs by App Id
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
    if let Err(e) = read_data_by_appid().await {
        eprintln!("Error: {:?}", e);
    }
}