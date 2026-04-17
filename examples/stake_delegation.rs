//! Delegate stake to a validator on the Mina network.
//!
//! Run with: cargo run --example stake_delegation
//!
//! Requires a running Mina daemon with the delegator account unlocked.

use mina_sdk::{Currency, Delegation, MinaClient};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    // Set MINA_TEST_SENDER_KEY (unlocked delegator) and MINA_TEST_RECEIVER_KEY
    // (target validator) to run. Without them, this example short-circuits.
    let (Ok(delegator), Ok(validator)) = (
        std::env::var("MINA_TEST_SENDER_KEY"),
        std::env::var("MINA_TEST_RECEIVER_KEY"),
    ) else {
        println!("Set MINA_TEST_SENDER_KEY and MINA_TEST_RECEIVER_KEY to run");
        return Ok(());
    };
    let fee = Currency::from_mina("0.01")?;

    let result = client
        .send_delegation(
            Delegation::sender(&delegator)
                .to(&validator)
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
