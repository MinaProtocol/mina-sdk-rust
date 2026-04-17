//! Handling errors from the Mina SDK.
//!
//! Run with: cargo run --example error_handling
//!
//! Demonstrates matching on specific error variants for robust applications.

use mina_sdk::{Currency, Error, MinaClient, Payment};

#[tokio::main]
async fn main() {
    let client = MinaClient::new("http://127.0.0.1:3085/graphql");

    // ---- AccountNotFound ----
    match client.get_account("B62qNONEXISTENT", None).await {
        Ok(account) => println!("Balance: {}", account.balance.total),
        Err(Error::AccountNotFound(key)) => {
            println!("Account {key} does not exist on chain")
        }
        Err(e) => eprintln!("Unexpected error: {e}"),
    }

    // ---- Connection errors (node not reachable) ----
    let bad_client = MinaClient::new("http://127.0.0.1:9999/graphql");
    match bad_client.get_sync_status().await {
        Ok(status) => println!("Status: {status}"),
        Err(Error::Connection { attempts, .. }) => {
            eprintln!("Could not reach node after {attempts} attempts")
        }
        Err(e) => eprintln!("Other error: {e}"),
    }

    // ---- GraphQL errors (e.g., invalid mutation input) ----
    match client
        .send_payment(
            Payment::sender("INVALID_KEY")
                .to("INVALID_KEY")
                .amount(Currency::from_nanomina(0))
                .fee(Currency::from_nanomina(0)),
        )
        .await
    {
        Ok(r) => println!("Unexpected success: {}", r.hash),
        Err(Error::Graphql { messages, .. }) => {
            eprintln!("GraphQL rejected the request: {messages}")
        }
        Err(e) => eprintln!("Other error: {e}"),
    }

    // ---- Currency validation errors (no node needed) ----
    match Currency::from_mina("not_a_number") {
        Ok(_) => println!("Unexpected success"),
        Err(Error::InvalidCurrency(input)) => {
            eprintln!("Bad currency input: {input}")
        }
        Err(e) => eprintln!("Other: {e}"),
    }

    // ---- Currency underflow ----
    let small = Currency::from_mina("1.0").unwrap();
    let large = Currency::from_mina("999.0").unwrap();
    match small.checked_sub(large) {
        Ok(result) => println!("Result: {result}"),
        Err(Error::CurrencyUnderflow(a, b)) => {
            eprintln!("Cannot subtract {b} from {a} (would go negative)")
        }
        Err(e) => eprintln!("Other: {e}"),
    }
}
