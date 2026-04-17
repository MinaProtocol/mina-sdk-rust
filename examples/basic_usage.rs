//! Basic usage of the Mina Rust SDK.
//!
//! Run with: cargo run --example basic_usage

use mina_sdk::{ClientConfig, Currency, Delegation, MinaClient, Payment};
use std::time::Duration;

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    // Connect to the default local Mina daemon (http://127.0.0.1:3085/graphql).
    // For a custom host/port use MinaClient::from_host_and_port("host", port).
    let client = MinaClient::default();

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

    // Query an account. A Mina public key looks like:
    //   B62qjVQLxt9nYMWGn45mkgwYfcz8e8jvjNCBo11VKJb7vxDNwv5QLPS
    // Set MINA_TEST_SENDER_KEY to a real key to run this section, otherwise we skip.
    let Ok(public_key) = std::env::var("MINA_TEST_SENDER_KEY") else {
        println!("Skipping account query (set MINA_TEST_SENDER_KEY to enable)");
        return Ok(());
    };
    match client.get_account(&public_key, None).await {
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

/// Example: send a payment and delegate stake.
#[allow(dead_code)]
async fn send_transactions() -> mina_sdk::Result<()> {
    let client = MinaClient::default();

    // Send a payment with memo
    let result = client
        .send_payment(
            Payment::sender("B62qsender...")
                .to("B62qreceiver...")
                .amount(Currency::from_mina("1.5")?)
                .fee(Currency::from_mina("0.01")?)
                .memo("coffee payment"),
        )
        .await?;
    println!("Payment hash: {}", result.hash);

    // Delegate stake
    let result = client
        .send_delegation(
            Delegation::sender("B62qsender...")
                .to("B62qdelegate...")
                .fee(Currency::from_mina("0.01")?)
                .memo("staking"),
        )
        .await?;
    println!("Delegation hash: {}", result.hash);

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
