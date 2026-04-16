//! Send a payment transaction on the Mina network.
//!
//! Run with: cargo run --example send_payment
//!
//! Requires a running Mina daemon with the sender account unlocked.

use mina_sdk::{Currency, MinaClient};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    // Define the payment
    let sender = "B62q...sender...";
    let receiver = "B62q...receiver...";
    let amount = Currency::from_mina("1.5")?;
    let fee = Currency::from_mina("0.01")?;

    // Send with a memo
    let result = client
        .send_payment(sender, receiver, amount, fee, Some("coffee payment"), None)
        .await?;

    println!("Payment submitted!");
    println!("  Hash:  {}", result.hash);
    println!("  ID:    {}", result.id);
    println!("  Nonce: {}", result.nonce);

    // Send with an explicit nonce (useful for sending multiple transactions)
    let result2 = client
        .send_payment(
            sender,
            receiver,
            Currency::from_mina("2.0")?,
            fee,
            Some("second payment"),
            Some(result.nonce + 1),
        )
        .await?;

    println!("Second payment: {}", result2.hash);

    Ok(())
}
