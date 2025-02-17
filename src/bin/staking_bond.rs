use dotenvy::dotenv;
use std::env;
use avail_rust::{
    prelude::*,
    transactions::staking::RewardDestination,
};

pub async fn staking_bond() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;

    //Setting amount of AVAIL to be staked
    let value = 1_000_000_000_000_000_000u128 * 1_000_000u128; // 1,000,000 AVAIL

    //Setting the payee to be a Validator
    let payee = RewardDestination::Staked;

    // Executing the transaction
    let tx = sdk.tx.staking.bond(value, payee);
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
    if let Err(e) = staking_bond().await {
        eprintln!("Error: {:?}", e);
    }
}
