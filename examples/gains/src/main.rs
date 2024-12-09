pub mod bindings;

use bindings::gns_pairs_storage::{GNSPairsStorage, Pair};
use dotenv::dotenv;
use ethers::addressbook::Address;
use ethers::contract::Multicall;
use ethers::prelude::{Middleware, Provider, Ws, U256};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL is not set");
    info!("Connecting to {}", url);

    let provider = Provider::<Ws>::connect_with_reconnects(url, 10).await?;
    let provider = provider.interval(Duration::from_secs(5));
    let client = Arc::new(provider);

    let chain_id = client.get_chainid().await?;
    info!("chain_id: {chain_id:?}");

    let address = env::var("GNS_MULTI_COLLAT_DIAMOND")
        .expect("GNS_MULTI_COLLAT_DIAMOND is not set")
        .parse::<Address>()?;
    let contract = GNSPairsStorage::new(address, client.clone());

    let count = contract.pairs_count().await?.as_usize();
    info!("pairs count: {count}");

    // for index in 0..count {
    //     let pair = contract.pairs(U256::from(index)).call().await?;
    //     println!("m.insert({}, \"{}/{}\");", index, pair.from, pair.to);
    // }

    let mut multicall = Multicall::new(client.clone(), None).await?;
    for pair_index in 0..count {
        multicall.add_call(contract.pairs(U256::from(pair_index)), false);
    }
    let pairs: Vec<Pair> = multicall.call_array().await?;
    for (index, pair) in pairs.iter().enumerate() {
        println!("m.insert({}, \"{}/{}\");", index, pair.from, pair.to);
    }

    Ok(())
}
