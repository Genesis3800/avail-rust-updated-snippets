use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;
use core::str::FromStr;

pub async fn transfer_allow_death() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;
    println!("Account Address: {}", account.public_key().to_account_id());

    // Executing the transaction
    let dest = AccountId::from_str("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw").unwrap();
	let amount = 12_345_000_000_000_000_000u128; // 12.345 AVAIL being transferred to the destination account
    let tx = sdk.tx.balances.transfer_allow_death(dest, amount);
    let res = tx.execute_and_watch_inclusion(&account, Options::default()).await?;

    println!(
        "Block Hash: {:?}, Block Number: {}, Tx Hash: {:?}, Tx Index: {}",
        res.block_hash, res.block_number, res.tx_hash, res.tx_index
    );

    Ok(())
    
}

// Add a main function to call transfer_allow_death
#[tokio::main]
async fn main() {
    if let Err(e) = transfer_allow_death().await {
        eprintln!("Error: {:?}", e);
    }
}