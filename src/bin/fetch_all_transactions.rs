use avail_rust::prelude::*;

pub async fn fetch_all_transactions() -> Result<(), ClientError> {

    // Create a new SDK instance
    let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await?;

    // Enter the block hash you want to fetch transactions from
    let block_hash = new_h256_from_hex("0x9016d7953c88115534a602f2d2548c70c4f5b378d86f4bedda82be2655467c5d")?;
    let block = Block::new(&sdk.client, block_hash).await?;

    // Fetch All Transactions
    let block_transactions = block.transactions(Filter::default());

    // Printout Block Transactions
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
    fetch_all_transactions().await?;
    Ok(())
}
