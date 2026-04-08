//! Rust SDK for interacting with Mina Protocol nodes via GraphQL.
//!
//! # Quick Start
//!
//! ```no_run
//! # async fn example() -> mina_sdk::Result<()> {
//! use mina_sdk::{MinaClient, Currency};
//!
//! let client = MinaClient::new("http://127.0.0.1:3085/graphql");
//!
//! // Query node status
//! let status = client.get_sync_status().await?;
//! println!("Node is {status}");
//!
//! // Send a payment
//! let result = client.send_payment(
//!     "B62q..sender..",
//!     "B62q..receiver..",
//!     Currency::from_mina("1.5")?,
//!     Currency::from_mina("0.01")?,
//!     Some("hello"),
//!     None,
//! ).await?;
//! println!("Payment hash: {}", result.hash);
//! # Ok(())
//! # }
//! ```

mod client;
mod currency;
pub mod error;
pub mod queries;
mod types;

pub use client::{ClientConfig, MinaClient};
pub use currency::Currency;
pub use error::{Error, Result};
pub use types::*;
