//! Working with Mina currency amounts.
//!
//! Run with: cargo run --example currency_operations
//!
//! This example demonstrates Currency creation, conversion, arithmetic,
//! and comparison — no running node required.

use mina_sdk::Currency;

fn main() -> mina_sdk::Result<()> {
    // ---- Creation ----

    // From a human-readable MINA string
    let one_mina = Currency::from_mina("1.0")?;

    // From nanomina (1 MINA = 1,000,000,000 nanomina)
    let also_one_mina = Currency::from_nanomina(1_000_000_000);

    // From a GraphQL response value (nanomina as string)
    let from_graphql = Currency::from_graphql("1000000000")?;

    assert_eq!(one_mina, also_one_mina);
    assert_eq!(also_one_mina, from_graphql);

    // ---- Display & conversion ----

    let amount = Currency::from_mina("42.5")?;
    println!("Display:     {amount}"); // 42.500000000
    println!("As MINA:     {}", amount.mina()); // 42.500000000
    println!("As nanomina: {}", amount.nanomina()); // 42500000000
    println!("For GraphQL: {}", amount.to_nanomina_str()); // 42500000000

    // ---- Arithmetic ----

    let a = Currency::from_mina("10.0")?;
    let b = Currency::from_mina("3.5")?;

    // Addition
    let sum = a + b;
    println!("{a} + {b} = {sum}");

    // Subtraction
    let diff = a - b;
    println!("{a} - {b} = {diff}");

    // Checked subtraction (returns Error instead of panicking)
    let small = Currency::from_mina("1.0")?;
    let large = Currency::from_mina("999.0")?;
    match small.checked_sub(large) {
        Ok(result) => println!("Result: {result}"),
        Err(e) => println!("Expected underflow: {e}"),
    }

    // Multiplication by scalar
    let fee = Currency::from_mina("0.01")?;
    let ten_fees = fee * 10;
    println!("10 x {fee} = {ten_fees}");

    // Also works in reverse
    let same = 10_u64 * fee;
    assert_eq!(ten_fees, same);

    // Checked overflow-safe variants
    assert!(fee.checked_add(a).is_some());
    assert!(fee.checked_mul(100).is_some());

    // ---- Comparison ----

    let low = Currency::from_mina("0.5")?;
    let high = Currency::from_mina("100.0")?;
    assert!(low < high);
    assert!(high > low);

    // Can be used in collections
    use std::collections::BTreeSet;
    let mut amounts = BTreeSet::new();
    amounts.insert(Currency::from_mina("3.0")?);
    amounts.insert(Currency::from_mina("1.0")?);
    amounts.insert(Currency::from_mina("2.0")?);
    println!(
        "Sorted: {:?}",
        amounts.iter().map(|c| c.mina()).collect::<Vec<_>>()
    );

    // ---- Smallest unit ----

    let one_nanomina = Currency::from_nanomina(1);
    println!("Smallest unit: {one_nanomina}"); // 0.000000001

    Ok(())
}
