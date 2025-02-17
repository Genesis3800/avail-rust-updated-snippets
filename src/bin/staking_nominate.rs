use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;
use core::str::FromStr;

pub async fn staking_nominate() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;
    
    //Setting the Validator to be nominated, max 16 targets
    let targets = [
        AccountId::from_str("5DqMavSQikX9eMzwHKiC8xS6VWB2yCd5gGQuQq7KheM2Mgc7").expect("Invalid account ID"),
        AccountId::from_str("5FphMk7DhSdq7jXsQCVQthw7XTiCWxdA9ZS6V43rKeJzvya9").expect("Invalid account ID"),
    ];

    //Executing the transaction
    let tx = sdk.tx.staking.nominate(&targets);
    let res = tx.execute_and_watch_inclusion(&account, Options::default()).await?;
    assert_eq!(res.is_successful(), Some(true));
    
    println!(
        "Block Hash: {:?}, Block Number: {}, Tx Hash: {:?}, Tx Index: {}",
        res.block_hash, res.block_number, res.tx_hash, res.tx_index
    );

    Ok(())
}




#[tokio::main]
async fn main() {
    if let Err(e) = staking_nominate().await {
        eprintln!("Error: {:?}", e);
    }
}
