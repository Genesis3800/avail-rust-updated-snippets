use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;

pub async fn nomination_pools_join() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;

    //Setting the amount to bond and the pool id
	let amount = 1_00_000_000_000_000_000_000_000u128; // 1_000_00 Avail tokens
    let pool_id = 1;

    //Executing the transaction
    let tx = sdk.tx.nomination_pools.join(amount, pool_id);
    let res = tx.execute_and_watch_inclusion(&account, Options::new()).await?;
    assert_eq!(res.is_successful(), Some(true));
    
    println!(
        "Block Hash: {:?}, Block Number: {}, Tx Hash: {:?}, Tx Index: {}",
        res.block_hash, res.block_number, res.tx_hash, res.tx_index
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = nomination_pools_join().await {
        eprintln!("Error: {:?}", e);
    }
}
