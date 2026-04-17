//! Delegate stake to a validator on the Mina network.
//!
//! Run with: cargo run --example stake_delegation
//!
//! Requires a running Mina daemon with the delegator account unlocked.

use mina_sdk::{Currency, Delegation, MinaClient};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    let delegator = "B62q...your_key...";
    let validator = "B62q...validator_key...";
    let fee = Currency::from_mina("0.01")?;

    let result = client
        .send_delegation(
            Delegation::sender(delegator)
                .to(validator)
                .fee(fee)
                .memo("staking"),
        )
        .await?;

    println!("Delegation submitted!");
    println!("  Hash:  {}", result.hash);
    println!("  ID:    {}", result.id);
    println!("  Nonce: {}", result.nonce);

    Ok(())
}
