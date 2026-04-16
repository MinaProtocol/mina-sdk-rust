//! Execute a custom GraphQL query against the Mina daemon.
//!
//! Run with: cargo run --example custom_query
//!
//! The SDK exposes `execute_query()` for running arbitrary GraphQL.
//! This is useful for queries not covered by the typed API methods.

use mina_sdk::MinaClient;
use serde_json::json;

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    // Example 1: Query the node's version
    let data = client
        .execute_query(r#"query { version }"#, None, "get_version")
        .await?;
    println!("Node version: {}", data["version"]);

    // Example 2: Query with variables
    let query = r#"query ($maxLength: Int) {
        bestChain(maxLength: $maxLength) {
            stateHash
            protocolState {
                consensusState {
                    blockHeight
                    epoch
                    slot
                }
            }
        }
    }"#;

    let data = client
        .execute_query(query, Some(json!({ "maxLength": 3 })), "best_chain_custom")
        .await?;

    if let Some(chain) = data["bestChain"].as_array() {
        for block in chain {
            let consensus = &block["protocolState"]["consensusState"];
            println!(
                "Block {} (epoch {}, slot {}): {}",
                consensus["blockHeight"],
                consensus["epoch"],
                consensus["slot"],
                &block["stateHash"].as_str().unwrap_or("?")[..20],
            );
        }
    }

    // Example 3: The built-in query constants are also available
    let data = client
        .execute_query(mina_sdk::queries::DAEMON_STATUS, None, "daemon_status")
        .await?;
    println!("Raw daemon status JSON: {}", data["daemonStatus"]);

    Ok(())
}
