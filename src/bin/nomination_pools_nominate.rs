use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;
use core::str::FromStr;

pub async fn nomination_pools_nominate() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;

    //Setting the pool id and the validators to nominate
    let pool_id = 1; // Use your pool id here

    //Setting an array of validators to nominate
    let validators = [
        AccountId::from_str("5DqMavSQikX9eMzwHKiC8xS6VWB2yCd5gGQuQq7KheM2Mgc7").expect("Invalid account ID"),
        AccountId::from_str("5FphMk7DhSdq7jXsQCVQthw7XTiCWxdA9ZS6V43rKeJzvya9").expect("Invalid account ID"),
    ];

    //Executing the transaction
    let tx = sdk.tx.nomination_pools.nominate(pool_id, validators.to_vec());
    let res = tx.execute_and_watch_inclusion(&account, Options::new()).await?;
    assert_eq!(res.is_successful(), Some(true));

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = nomination_pools_nominate().await {
        eprintln!("Error: {:?}", e);
    }
}