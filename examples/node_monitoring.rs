//! Monitor a Mina node — sync status, peers, and recent blocks.
//!
//! Run with: cargo run --example node_monitoring
//!
//! Demonstrates using the SDK for node observability and health checks.

use mina_sdk::{MinaClient, SyncStatus};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::default();

    // ---- Health check ----
    let sync = client.get_sync_status().await?;
    match sync {
        SyncStatus::Synced => println!("[OK]  Node is synced"),
        SyncStatus::Bootstrap => println!("[WARN] Node is bootstrapping"),
        SyncStatus::Catchup => println!("[WARN] Node is catching up"),
        SyncStatus::Connecting => println!("[WARN] Node is connecting to network"),
        SyncStatus::Listening => println!("[WARN] Node is listening (not yet syncing)"),
        SyncStatus::Offline => println!("[ERR]  Node is offline"),
    }

    // ---- Daemon status ----
    let status = client.get_daemon_status().await?;
    println!("\nDaemon Status:");
    println!("  Blockchain length: {:?}", status.blockchain_length);
    println!(
        "  Highest block received: {:?}",
        status.highest_block_length_received
    );
    if let Some(secs) = status.uptime_secs {
        println!("  Uptime: {}h {}m", secs / 3600, (secs % 3600) / 60);
    }
    if let Some(hash) = &status.state_hash {
        println!("  State hash: {}", &hash[..20.min(hash.len())]);
    }

    // ---- Network info ----
    let network_id = client.get_network_id().await?;
    println!("\nNetwork: {network_id}");

    // ---- Peers ----
    let peers = client.get_peers().await?;
    println!("\nConnected peers: {}", peers.len());
    for (i, peer) in peers.iter().enumerate().take(5) {
        println!(
            "  {}: {}:{} ({})",
            i + 1,
            peer.host,
            peer.port,
            peer.peer_id
        );
    }
    if peers.len() > 5 {
        println!("  ... and {} more", peers.len() - 5);
    }

    // ---- Recent blocks ----
    let blocks = client.get_best_chain(Some(5)).await?;
    println!("\nRecent blocks:");
    for block in &blocks {
        println!(
            "  Height {} | {} txns | {}",
            block.height,
            block.command_transaction_count,
            &block.state_hash[..16],
        );
    }

    Ok(())
}
