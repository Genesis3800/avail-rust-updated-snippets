use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;
use core::str::FromStr;

pub async fn nomination_pools_create_with_pool_id() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;

    //Setting the amount to bond and the pool id
	let amount = 10_000_000_000_000_000_000_000u128; // 10_000 Avail tokens
    let pool_id = 1; // You need to use a pool id that is not already in use

    //Setting the root, nominator and bouncer
    let root = account.public_key().to_account_id(); //Setting the root to be the account itself

    let nominator = AccountId::from_str("5DDY2yzh8uCysYFAiRSTeQVwtZSKNF49CkQkyPH852xvrYKk").unwrap();
    let bouncer = AccountId::from_str("5D7LgWT5J6MVRa6PvTTXUfd9VvggEcguvPnnWAGK44CJKFEq").unwrap();

    //Executing the transaction
    let tx = sdk.tx.nomination_pools.create_with_pool_id(amount, root, nominator, bouncer, pool_id);
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
    if let Err(e) = nomination_pools_create_with_pool_id().await {
        eprintln!("Error: {:?}", e);
    }
}
