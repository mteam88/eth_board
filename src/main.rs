use std::ops::Deref;

use ethers::{providers::{Middleware, Provider}, types::{Block, Transaction}};
use ethers_providers::{Http, ProviderExt, StreamExt};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Http>::connect("https://ethereum.publicnode.com")
            .await;
    let mut stream = provider.watch_blocks().await?;
    // call digest on every block in stream
    while let Some(block) = stream.next().await {
        let block = provider.get_block_with_txs(block).await?;
        digest(block.unwrap()).await?;
    }

    Ok(())
}

async fn digest(block: Block<Transaction>) -> Result<()> {
    // do something with block
    block.transactions.iter().for_each(|tx| {
        println!("{:?} --> {:?}", tx.hash, String::from_utf8_lossy(&tx.deref().input.to_vec()));
    });
    Ok(())
}