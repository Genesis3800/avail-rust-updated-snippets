use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;
use core::str::FromStr;

pub async fn nomination_pools_create() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;

    //Setting the amount to bond and the pool id
	let amount = 10_000_000_000_000_000_000_000u128; // 10_000 Avail tokens

    //Setting the root, nominator and bouncer
    let root = account.public_key().to_account_id(); //Setting the root to be the account itself

    let nominator = AccountId::from_str("5DDY2yzh8uCysYFAiRSTeQVwtZSKNF49CkQkyPH852xvrYKk").unwrap();
    let bouncer = AccountId::from_str("5D7LgWT5J6MVRa6PvTTXUfd9VvggEcguvPnnWAGK44CJKFEq").unwrap();

    //Executing the transaction
    let tx = sdk.tx.nomination_pools.create(amount, root, nominator, bouncer);
    let res = tx.execute_and_watch_inclusion(&account, Options::new()).await?;
    assert_eq!(res.is_successful(), Some(true));
    
    Ok(())
}


#[tokio::main]
async fn main() {
    if let Err(e) = nomination_pools_create().await {
        eprintln!("Error: {:?}", e);
    }
}