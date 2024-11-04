use dotenv_codegen::dotenv;
use ethers::abi::RawLog;
use ethers::middleware::Middleware;
use ethers::prelude::{
    Address, BlockNumber, ConnectionDetails, EthEvent, EthLogDecode, Filter, Provider, StreamExt,
    ValueOrArray, Ws,
};
use gmx::bindings::event_emitter::{EventEmitterEvents, EventLog1Filter};
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url: &str = dotenv!("ETH_RPC_URL");
    let opts = ConnectionDetails::new(url, None);
    let provider = Provider::<Ws>::connect_with_reconnects(opts, 10).await?;
    let provider = provider.interval(Duration::from_secs(5));
    let chain_id = provider.get_chainid().await?;
    println!("chain_id: {}", chain_id);

    let latest_block = provider
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap();
    println!("latest_block: {}", latest_block);

    // use std::sync::Arc;
    // use gmx::bindings::EventEmitter;
    // let address = Address::from_str(dotenv!("GMX_EVENT_EMITTER_ADDRESS"))?;
    // let client = Arc::new(provider);
    // let contract = EventEmitter::new(address, client);
    // let event = contract.events().from_block(latest_block);
    // let mut stream = event.stream().await?.with_meta();
    // while let Some(Ok((evt, meta))) = stream.next().await {
    //     match evt {
    //         EventEmitterEvents::EventLog1Filter(filter) => {
    //             let event_name = filter.event_name.to_string();
    //             if ["PositionIncrease", "PositionDecrease"].contains(&event_name.as_str()) {
    //                 println!("Txn Hash: {:?} {:?}", meta.transaction_hash, filter);
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    let address = Address::from_str(dotenv!("GMX_EVENT_EMITTER_ADDRESS"))?;
    let filter = Filter::new()
        .address(ValueOrArray::Array(vec![address]))
        .topic0(ValueOrArray::Array(vec![EventLog1Filter::signature()]))
        .from_block(latest_block);
    let mut stream = provider.subscribe_logs(&filter).await?;
    while let Some(log) = stream.next().await {
        let txn_hash = log.transaction_hash.unwrap();
        if let Ok(event) = EventEmitterEvents::decode_log(&RawLog {
            topics: log.topics,
            data: log.data.to_vec(),
        }) {
            match event {
                EventEmitterEvents::EventLog1Filter(filter) => {
                    let event_name = filter.event_name.to_string();
                    if ["PositionIncrease", "PositionDecrease"].contains(&event_name.as_str()) {
                        println!("Txn Hash: {:?} {:?}", txn_hash, filter);
                    }
                }
                _ => {}
            }
        };
    }

    Ok(())
}
