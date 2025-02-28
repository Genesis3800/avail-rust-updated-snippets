use avail_rust::prelude::*;

pub async fn read_data_by_hash() -> Result<(), ClientError> {

    // Create a new SDK instance
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;
    
    let block_hash = new_h256_from_hex("0xd5f95593d91a581d7ce7b8717789298345be4be47e75ba93e7159cfe23083a7b")?;

    let block = Block::new(&sdk.client, block_hash).await?;

    // All Block Blobs by Hash
    let tx_hash = new_h256_from_hex("0x70d8cc521c341d717f5b11d1898fc7a21f9d894c3617929aaabaea71c4814911")?;
    let blobs = block.data_submissions(Filter::new().tx_hash(tx_hash));
    assert_eq!(blobs.len(), 1, "");

    let blob = &blobs[0];

    // Printout All Block Blobs by Hash
    let blob_data = blob.to_ascii().unwrap();
    assert_eq!(blob.tx_hash, tx_hash, "Tx Hash must be the same");

    println!(
        "Tx Hash: {:?}, Tx Index: {}, Data: {:?}, App Id: {}, Tx Singer: {:?}",
        blob.tx_hash,
        blob.tx_index,
        blob_data,
        blob.app_id,
        blob.ss58address(),
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = read_data_by_hash().await {
        eprintln!("Error: {:?}", e);
    }
}