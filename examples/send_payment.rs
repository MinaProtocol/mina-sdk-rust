//! Send a payment transaction on the Mina network.
//!
//! Run with: cargo run --example send_payment
//!
//! Requires a running Mina daemon with the sender account unlocked.
//!
//! ## Providing keys
//!
//! This example reads two env vars — both are base58 Mina public keys like
//! `B62qjVQLxt9nYMWGn45mkgwYfcz8e8jvjNCBo11VKJb7vxDNwv5QLPS`:
//!
//! - `MINA_TEST_SENDER_KEY` — the sender account. Must be imported **and
//!   unlocked** in the daemon's wallet (via `importAccount` + `unlockAccount`
//!   GraphQL mutations, or `mina accounts import` + `mina accounts unlock`).
//!   You also need its secret key file on disk for the daemon to sign with.
//! - `MINA_TEST_RECEIVER_KEY` — any destination public key; no wallet import
//!   needed.
//!
//! Generate a fresh keypair locally with:
//!
//!     mina advanced generate-keypair --privkey-path <file>
//!
//! Or, on a lightnet node, acquire a pre-funded unlocked account:
//!
//!     curl 'http://127.0.0.1:8181/acquire-account?unlockAccount=true'
//!
//! Without both env vars set, the example short-circuits so it stays safe to
//! run as a smoke test.

use mina_sdk::{Currency, MinaClient, Payment};

#[tokio::main]
async fn main() -> mina_sdk::Result<()> {
    let client = MinaClient::default();

    let (Ok(sender), Ok(receiver)) = (
        std::env::var("MINA_TEST_SENDER_KEY"),
        std::env::var("MINA_TEST_RECEIVER_KEY"),
    ) else {
        println!("Set MINA_TEST_SENDER_KEY and MINA_TEST_RECEIVER_KEY to run");
        return Ok(());
    };

    let fee = Currency::from_mina("0.01")?;

    // Build the payment, then submit it.
    let payment = Payment::sender(&sender)
        .to(&receiver)
        .amount(Currency::from_mina("1.5")?)
        .fee(fee)
        .memo("coffee payment");

    let result = client.send_payment(payment).await?;

    println!("Payment submitted!");
    println!("  Hash:  {}", result.hash);
    println!("  ID:    {}", result.id);
    println!("  Nonce: {}", result.nonce);

    // Same pattern with an explicit nonce — useful for submitting a second
    // transaction before the first has been included in a block.
    let next_payment = Payment::sender(&sender)
        .to(&receiver)
        .amount(Currency::from_mina("2.0")?)
        .fee(fee)
        .memo("second payment")
        .nonce(result.nonce + 1);

    let result2 = client.send_payment(next_payment).await?;
    println!("Second payment: {}", result2.hash);

    Ok(())
}
