# Mina Rust SDK

Rust SDK for interacting with [Mina Protocol](https://minaprotocol.com) nodes via GraphQL.

## Features

- **Async GraphQL client** — query node status, accounts, blocks; send payments and delegations
- Typed response structs with `Currency` arithmetic
- Automatic retry with configurable backoff
- Public `execute_query()` for custom GraphQL queries
- `tracing` instrumentation

## Installation

```toml
[dependencies]
mina-sdk = { git = "https://github.com/MinaProtocol/mina-sdk-rust.git" }
```

## Quick Start

```rust
use mina_sdk::{MinaClient, Currency};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    // Check sync status
    let status = client.get_sync_status().await?;
    println!("Sync status: {status}");

    // Query an account
    let account = client.get_account("B62q...", None).await?;
    println!("Balance: {} MINA", account.balance.total);

    // Send a payment
    let result = client.send_payment(
        "B62qsender...",
        "B62qreceiver...",
        Currency::from_mina("1.5")?,
        Currency::from_mina("0.01")?,
        Some("hello from SDK"),
        None,
    ).await?;
    println!("Tx hash: {}", result.hash);

    Ok(())
}
```

## Configuration

```rust
use mina_sdk::{MinaClient, ClientConfig};
use std::time::Duration;

let client = MinaClient::with_config(ClientConfig {
    graphql_uri: "http://127.0.0.1:3085/graphql".to_string(),
    retries: 3,
    retry_delay: Duration::from_secs(5),
    timeout: Duration::from_secs(30),
});
```

## API Reference

### Queries

| Method | Description |
|--------|-------------|
| `get_sync_status()` | Node sync status (Synced, Bootstrap, etc.) |
| `get_daemon_status()` | Comprehensive daemon status |
| `get_network_id()` | Network identifier |
| `get_account(public_key, token_id)` | Account balance, nonce, delegate |
| `get_best_chain(max_length)` | Recent blocks from best chain |
| `get_peers()` | Connected peers |
| `get_pooled_user_commands(public_key)` | Pending transactions |
| `execute_query(query, variables, name)` | Run a custom GraphQL query |

### Mutations

| Method | Description |
|--------|-------------|
| `send_payment(sender, receiver, amount, fee, memo, nonce)` | Send a payment |
| `send_delegation(sender, delegate_to, fee, memo, nonce)` | Delegate stake |
| `set_snark_worker(public_key)` | Set/unset SNARK worker |
| `set_snark_work_fee(fee)` | Set SNARK work fee |

### Currency

```rust
use mina_sdk::Currency;

let a = Currency::from_mina("10")?;              // 10 MINA
let b = Currency::from_mina("1.5")?;             // 1.5 MINA
let c = Currency::from_nanomina(1_000_000_000);   // 1 MINA
let d = Currency::from_graphql("1500000000")?;     // from GraphQL response

println!("{}", (a + b));        // 11.500000000
println!("{}", a.nanomina());   // 10000000000
assert!(a > b);
```

## Development

```bash
git clone https://github.com/MinaProtocol/mina-sdk-rust.git
cd mina-sdk-rust
cargo test
cargo clippy
```

## License

Apache License 2.0
