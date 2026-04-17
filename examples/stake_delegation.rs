//! Delegate stake to a validator on the Mina network.
//!
//! Run with: cargo run --example stake_delegation
//!
//! Requires a running Mina daemon with the delegator account unlocked.
//!
//! ## Providing keys
//!
//! Uses the same env vars as `send_payment` (see that example for details on
//! how to generate keys and unlock accounts):
//!
//! - `MINA_TEST_SENDER_KEY` — the delegator account (must be unlocked in the
//!   daemon's wallet).
//! - `MINA_TEST_RECEIVER_KEY` — the validator to delegate to.
//!
//! Without both set, the example short-circuits.

use mina_sdk::{Currency, Delegation, MinaClient};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::default();

    let (Ok(delegator), Ok(validator)) = (
        std::env::var("MINA_TEST_SENDER_KEY"),
        std::env::var("MINA_TEST_RECEIVER_KEY"),
    ) else {
        println!("Set MINA_TEST_SENDER_KEY and MINA_TEST_RECEIVER_KEY to run");
        return Ok(());
    };

    // Build the delegation, then submit it.
    let delegation = Delegation::sender(&delegator)
        .to(&validator)
        .fee(Currency::from_mina("0.01")?)
        .memo("staking");

    let result = client.send_delegation(delegation).await?;

    println!("Delegation submitted!");
    println!("  Hash:  {}", result.hash);
    println!("  ID:    {}", result.id);
    println!("  Nonce: {}", result.nonce);

    Ok(())
}
