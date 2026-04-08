//! Basic usage of the Mina Rust SDK.
//!
//! Run with: cargo run --example basic_usage

use mina_sdk::{ClientConfig, MinaClient};
use std::time::Duration;

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    // Connect to a local Mina daemon (default: http://127.0.0.1:3085/graphql)
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    // Check sync status
    let sync_status = client.get_sync_status().await?;
    println!("Sync status: {sync_status}");

    // Get daemon status with peer info
    let status = client.get_daemon_status().await?;
    println!("Blockchain length: {:?}", status.blockchain_length);
    println!("Peers: {}", status.peers.as_ref().map_or(0, |p| p.len()));

    // Get network ID
    let network_id = client.get_network_id().await?;
    println!("Network: {network_id}");

    // Get recent blocks
    let blocks = client.get_best_chain(Some(5)).await?;
    for block in &blocks {
        println!(
            "Block {}: {}... ({} txns)",
            block.height,
            &block.state_hash[..20.min(block.state_hash.len())],
            block.command_transaction_count,
        );
    }

    // Query an account (replace with a valid public key)
    match client.get_account("B62q...", None).await {
        Ok(account) => {
            println!("Balance: {} MINA", account.balance.total);
            println!("Nonce: {}", account.nonce);
        }
        Err(mina_sdk::Error::AccountNotFound(_)) => {
            println!("Account not found");
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Example: connect to a remote node with custom configuration.
#[allow(dead_code)]
async fn connect_to_remote_node() -> mina_sdk::Result<()> {
    let client = MinaClient::with_config(ClientConfig {
        graphql_uri: "http://my-mina-node:3085/graphql".to_string(),
        retries: 5,
        retry_delay: Duration::from_secs(10),
        timeout: Duration::from_secs(60),
    });

    let status = client.get_sync_status().await?;
    println!("Remote node status: {status}");

    Ok(())
}
