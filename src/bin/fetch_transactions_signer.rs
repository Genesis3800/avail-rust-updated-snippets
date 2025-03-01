use avail_rust::prelude::*;

pub async fn fetch_transactions_by_signer() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Enter the block hash you want to fetch transactions from
    let block_hash = new_h256_from_hex("0x75a6c54bb5ea904e47fa151956992d7cf543bc7c936d78488e311db8e10397c1")?;
    // Fetch the Block
    let block = Block::new(&sdk.client, block_hash).await?;

    // Fetch All Transaction filtered by Signer
    let account_id = account_id_from_str("5CqgQkrDcdg5QrtuxT3H7WszrqgrBMhdwRbmMVXQzc4VSiEg")?;
    let block_transactions = block.transactions(Filter::new().tx_signer(account_id.clone()));

    // Printout Block Transactions made by Signer
    for tx in block_transactions.iter() {

        println!(
            "Pallet Name: {:?}, Pallet Index: {}, Call Name: {:?}, Call Index: {:?}, Tx Hash: {:?}, Tx Index: {}",
            tx.pallet_name(),
            tx.pallet_index(),
            tx.call_name(),
            tx.call_index(),
            tx.tx_hash(),
            tx.tx_index()
        );

        println!(
            "Tx Signer: {:?}, App Id: {:?}, Tip: {:?}, Mortality: {:?}, Nonce: {:?}",
            tx.ss58address(),
            tx.app_id(),
            tx.tip(),
            tx.mortality(),
            tx.nonce(),
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    fetch_transactions_by_signer().await?;
    Ok(())
}
