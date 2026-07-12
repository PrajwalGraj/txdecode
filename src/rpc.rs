use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

pub async fn fetch_transaction(signature: &str) -> Result<Value> {
    let client = Client::new();

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [
            signature,
            {
                "encoding": "json",
                "maxSupportedTransactionVersion": 0
            }
        ]
    });

    let response = client
        .post(RPC_URL)
        .json(&body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    Ok(json)
}