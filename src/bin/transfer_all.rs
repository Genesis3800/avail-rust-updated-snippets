use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;
use core::str::FromStr;

pub async fn transfer_all() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;
    println!("Account Address: {}", account.public_key().to_account_id());

    // Executing the transaction
    let dest = AccountId::from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw").unwrap();

    // Please take note of the `keep_alive` bool parameter
    // If set to true, the transfer transaction will leave the origin account with a small balance
    // that is above the existential deposit and prevents the account from being reaped

    // Set the `keep_alive` parameter to `false` only if you are ok with the origin account being reaped
    let tx = sdk.tx.balances.transfer_all(dest, true);
    let res = tx.execute_and_watch_inclusion(&account, Options::default()).await?;

    println!(
        "Block Hash: {:?}, Block Number: {}, Tx Hash: {:?}, Tx Index: {}",
        res.block_hash, res.block_number, res.tx_hash, res.tx_index
    );

    Ok(())
}

// Add a main function to call transfer_all
#[tokio::main]
async fn main() {
    if let Err(e) = transfer_all().await {
        eprintln!("Error: {:?}", e);
    }
}