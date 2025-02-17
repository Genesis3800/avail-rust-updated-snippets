use dotenvy::dotenv;
use std::env;
use avail_rust::transactions::staking::Commission;
use avail_rust::prelude::*;
pub async fn staking_validate() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;

    //Setting your Validator's commission and blocking status
    let commission = Commission::new(5)?; // 5%
    let blocked = false; // Not blocked

    //Executing the transaction
    let tx = sdk.tx.staking.validate(commission, blocked);
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
    if let Err(e) = staking_validate().await {
        eprintln!("Error: {:?}", e);
    }
}
