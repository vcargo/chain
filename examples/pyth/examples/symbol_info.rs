use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use strum::{Display, EnumString};

#[derive(Debug, Deserialize)]
struct SymbolInfo {
    symbol: Vec<String>,
    currency: Vec<String>,
    #[serde(rename = "base-currency")]
    base_currency: Vec<Option<String>>,
    description: Vec<String>,
    #[serde(rename = "type")]
    symbol_type: Vec<String>,
}

async fn get_symbol_info(group: &str) -> Result<SymbolInfo> {
    let client = Client::new();
    let url = format!("https://benchmarks.pyth.network/v1/shims/tradingview/symbol_info?group={group}");
    let response = client.get(url.as_str()).send().await?;
    let status = response.status();
    let body = response.text().await?;

    if status != StatusCode::OK || !body.starts_with(r#"{"s":"ok""#) {
        bail!(
            "failed to get symbol_info from url: {} (status: {} body: {})",
            url,
            status,
            body
        );
    }

    let symbol_info: SymbolInfo = serde_json::from_str(&body)?;
    Ok(symbol_info)
}

async fn get_symbol_info_batch(groups: Vec<&str>) -> Result<Vec<SymbolInfo>> {
    let mut symbol_infos = Vec::new();

    for group in groups {
        let symbol_info = get_symbol_info(group).await?;
        symbol_infos.push(symbol_info);
    }

    Ok(symbol_infos)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    group: Vec<Group>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Display, EnumString)]
enum Group {
    Stock,
    Forex,
    Crypto,
    CFD,
    Bond,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let groups: Vec<String> = cli
        .group
        .into_iter()
        .map(|group| format!("pyth_{}", group.to_string().to_lowercase()))
        .collect();
    let groups: Vec<&str> = groups.iter().map(|s| s.as_str()).collect();

    let symbol_infos = get_symbol_info_batch(groups).await?;
    let formatted_symbol_infos = symbol_infos
        .iter()
        .map(|symbol_info| {
            let mut results = vec![];
            for (i, symbol) in symbol_info.symbol.iter().enumerate() {
                results.push(format!(
                    "symbol: {}\ncurrency: {}\nbase_currency: {:?}\ndescription: {}\ntype: {}",
                    symbol,
                    symbol_info.currency[i],
                    symbol_info.base_currency[i],
                    symbol_info.description[i],
                    symbol_info.symbol_type[i]
                ));
            }
            results.join("\n\n")
        })
        .collect::<Vec<String>>();

    let results = formatted_symbol_infos.join(format!("\n\n{}\n\n", "=".repeat(50)).as_str());
    println!("{}", results);

    Ok(())
}
