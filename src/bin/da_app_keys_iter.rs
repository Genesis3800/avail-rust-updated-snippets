use avail_rust::prelude::*;

pub async fn da_app_keys_iter() -> Result<(), ClientError> {
    
    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    let block_hash = sdk.client.best_block_hash().await?;
    let storage = sdk.client.storage().at(block_hash);
    let address = avail::storage().data_availability().app_keys_iter();
    let mut results = storage.iter(address).await?;

    while let Some(Ok(kv)) = results.next().await {
        let key = (&kv.key_bytes[49..]).to_vec();
        let key = String::from_utf8(key).unwrap();

        println!("Key: {:?}", key);
        println!("Value: {:?}", kv.value);
    }

    Ok(())
}

// Add a main function to call da_app_keys_iter
#[tokio::main]
async fn main() {
    if let Err(e) = da_app_keys_iter().await {
        eprintln!("Error: {:?}", e);
    }
}