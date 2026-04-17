//! Execute a custom GraphQL query against the Mina daemon.
//!
//! Run with: cargo run --example custom_query
//!
//! The SDK covers the common queries with typed wrappers (see
//! `basic_usage` and `node_monitoring`). For anything the typed API doesn't
//! expose, `MinaClient::query` returns a builder that runs arbitrary GraphQL
//! through the same retry-aware client.

use mina_sdk::MinaClient;
use serde_json::json;

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::default();

    // Simplest case: no variables.
    let data = client.query("query { version }").send().await?;
    println!("Node version: {}", data["version"]);

    // Query genesisConstants — not wrapped by the SDK, so run it raw.
    // `coinbase` / `accountCreationFee` are returned as nanomina strings.
    let data = client
        .query(
            r#"query {
                genesisConstants {
                    coinbase
                    accountCreationFee
                    genesisTimestamp
                }
            }"#,
        )
        .name("genesis_constants")
        .send()
        .await?;

    let genesis = &data["genesisConstants"];
    println!("Genesis timestamp:    {}", genesis["genesisTimestamp"]);
    println!("Coinbase (nanomina):  {}", genesis["coinbase"]);
    println!("Account fee (nanomina): {}", genesis["accountCreationFee"]);

    // Query with variables — here we look up the transaction pool for a key.
    let key = std::env::var("MINA_TEST_SENDER_KEY")
        .unwrap_or_else(|_| "B62qjVQLxt9nYMWGn45mkgwYfcz8e8jvjNCBo11VKJb7vxDNwv5QLPS".into());
    let data = client
        .query(
            r#"query ($key: PublicKey!) {
                pooledUserCommands(publicKey: $key) { hash }
            }"#,
        )
        .variables(json!({ "key": key }))
        .name("pooled_for_key")
        .send()
        .await?;

    let pool_size = data["pooledUserCommands"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    println!(
        "Pending txns for {}: {}",
        &key[..20.min(key.len())],
        pool_size
    );

    Ok(())
}
