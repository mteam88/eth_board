use ethers::{
    providers::{Middleware, Provider},
    types::Transaction,
};
use ethers_providers::{Http, ProviderExt, StreamExt};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::connect("https://ethereum.publicnode.com").await;
    let mut stream = provider.watch_blocks().await?;
    // call digest on every block in stream
    while let Some(block) = stream.next().await {
        let block = provider.get_block_with_txs(block).await?.unwrap();
        block.transactions.iter().for_each(|tx| {
            tokio::spawn(digest(tx.clone()));
        });
    }

    Ok(())
}

async fn digest(tx: Transaction) -> Result<()> {
    // do something with block
    println!(
        "{:?} --> {:?}",
        tx.hash,
        String::from_utf8_lossy(&tx.input.to_vec())
    );
    Ok(())
}
