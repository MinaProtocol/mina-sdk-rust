//! Send a payment transaction on the Mina network.
//!
//! Run with: cargo run --example send_payment
//!
//! Requires a running Mina daemon with the sender account unlocked.

use mina_sdk::{Currency, MinaClient, Payment};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    let sender = "B62q...sender...";
    let receiver = "B62q...receiver...";
    let fee = Currency::from_mina("0.01")?;

    // Send with a memo
    let result = client
        .send_payment(
            Payment::sender(sender)
                .to(receiver)
                .amount(Currency::from_mina("1.5")?)
                .fee(fee)
                .memo("coffee payment"),
        )
        .await?;

    println!("Payment submitted!");
    println!("  Hash:  {}", result.hash);
    println!("  ID:    {}", result.id);
    println!("  Nonce: {}", result.nonce);

    // Send with an explicit nonce (useful for sending multiple transactions)
    let result2 = client
        .send_payment(
            Payment::sender(sender)
                .to(receiver)
                .amount(Currency::from_mina("2.0")?)
                .fee(fee)
                .memo("second payment")
                .nonce(result.nonce + 1),
        )
        .await?;

    println!("Second payment: {}", result2.hash);

    Ok(())
}
