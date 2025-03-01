use dotenvy::dotenv;
use std::env;
use avail_rust::prelude::*;

type ApplicationKeyCreatedEvent = avail::data_availability::events::ApplicationKeyCreated;

pub async fn create_application_key() -> Result<(), ClientError> {
    
    // Create a new SDK instance
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;
 
    // Loading seed phrase and creating an account derived from the seed
    dotenv().ok();
    let seed = env::var("SEED").expect("SEED environment variable is not set");
    let account = account::from_secret_uri(&seed)?;
    println!("Account Address: {}", account.public_key().to_account_id());

    // Application Key Creation
    // Please note that if an application key with the same `key` already exists, the transaction will fail.

    let key = "My Application Ke298292y".as_bytes().to_vec();
    let tx = sdk.tx.data_availability.create_application_key(key);
    let res = tx.execute_and_watch_inclusion(&account, Options::default()).await?;
    assert_eq!(res.is_successful(), Some(true), "Transactions must be successful");

    let events = res.events.as_ref().unwrap();
    let event = events.find_first::<ApplicationKeyCreatedEvent>().unwrap();
    let Some(event) = event else {
        return Err("Failed to get Application Key Created Event".into());
    };
    let app_id = event.id.0;
    println!("Application Key Created: {}", app_id);

    Ok(())
}

// Add a main function to call create_application_key
#[tokio::main]
async fn main() {
    if let Err(e) = create_application_key().await {
        eprintln!("Error: {:?}", e);
    }
}
