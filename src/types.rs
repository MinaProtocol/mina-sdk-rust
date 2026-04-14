use serde::{Deserialize, Serialize};

use crate::Currency;

/// Sync status of a Mina daemon node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncStatus {
    Connecting,
    Listening,
    Offline,
    Bootstrap,
    Synced,
    Catchup,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connecting => write!(f, "CONNECTING"),
            Self::Listening => write!(f, "LISTENING"),
            Self::Offline => write!(f, "OFFLINE"),
            Self::Bootstrap => write!(f, "BOOTSTRAP"),
            Self::Synced => write!(f, "SYNCED"),
            Self::Catchup => write!(f, "CATCHUP"),
        }
    }
}

/// Information about a connected peer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerInfo {
    pub peer_id: String,
    pub host: String,
    pub port: i64,
}

/// Comprehensive daemon status.
#[derive(Debug, Clone)]
pub struct DaemonStatus {
    pub sync_status: SyncStatus,
    pub blockchain_length: Option<i64>,
    pub highest_block_length_received: Option<i64>,
    pub uptime_secs: Option<i64>,
    pub state_hash: Option<String>,
    pub commit_id: Option<String>,
    pub peers: Option<Vec<PeerInfo>>,
}

/// Account balance with total, liquid, and locked amounts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountBalance {
    pub total: Currency,
    pub liquid: Option<Currency>,
    pub locked: Option<Currency>,
}

/// Account data returned by the daemon.
#[derive(Debug, Clone)]
pub struct AccountData {
    pub public_key: String,
    pub nonce: u64,
    pub balance: AccountBalance,
    pub delegate: Option<String>,
    pub token_id: Option<String>,
}

/// Block info from the best chain.
#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub state_hash: String,
    pub height: u64,
    pub global_slot_since_hard_fork: u64,
    pub global_slot_since_genesis: u64,
    pub creator_pk: String,
    pub command_transaction_count: i64,
}

/// Result of a send_payment mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendPaymentResult {
    pub id: String,
    pub hash: String,
    pub nonce: u64,
}

/// Result of a send_delegation mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendDelegationResult {
    pub id: String,
    pub hash: String,
    pub nonce: u64,
}

/// Parameters for sending a payment transaction.
///
/// Built using a fluent API where each method documents its purpose:
///
/// ```
/// use mina_sdk::{Payment, Currency};
///
/// let payment = Payment::sender("B62qsender...")
///     .to("B62qreceiver...")
///     .amount(Currency::from_nanomina(1_500_000_000))
///     .fee(Currency::from_nanomina(10_000_000))
///     .memo("coffee payment")
///     .nonce(42);
/// ```
#[derive(Debug, Clone)]
pub struct Payment {
    pub sender: String,
    pub receiver: String,
    pub amount: Currency,
    pub fee: Currency,
    pub memo: Option<String>,
    pub nonce: Option<u64>,
}

impl Payment {
    /// Start building a payment with the sender's public key.
    pub fn sender(sender: &str) -> Self {
        Self {
            sender: sender.to_string(),
            receiver: String::new(),
            amount: Currency::from_nanomina(0),
            fee: Currency::from_nanomina(0),
            memo: None,
            nonce: None,
        }
    }

    /// Set the receiver's public key.
    pub fn to(mut self, receiver: &str) -> Self {
        self.receiver = receiver.to_string();
        self
    }

    /// Set the payment amount.
    pub fn amount(mut self, amount: Currency) -> Self {
        self.amount = amount;
        self
    }

    /// Set the transaction fee.
    pub fn fee(mut self, fee: Currency) -> Self {
        self.fee = fee;
        self
    }

    /// Set an optional memo.
    pub fn memo(mut self, memo: &str) -> Self {
        self.memo = Some(memo.to_string());
        self
    }

    /// Set an explicit nonce (otherwise the daemon picks the next nonce).
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }
}

/// Parameters for sending a stake delegation transaction.
///
/// ```
/// use mina_sdk::{Delegation, Currency};
///
/// let delegation = Delegation::sender("B62qsender...")
///     .to("B62qdelegate...")
///     .fee(Currency::from_nanomina(10_000_000))
///     .memo("staking");
/// ```
#[derive(Debug, Clone)]
pub struct Delegation {
    pub sender: String,
    pub delegate_to: String,
    pub fee: Currency,
    pub memo: Option<String>,
    pub nonce: Option<u64>,
}

impl Delegation {
    /// Start building a delegation with the sender's public key.
    pub fn sender(sender: &str) -> Self {
        Self {
            sender: sender.to_string(),
            delegate_to: String::new(),
            fee: Currency::from_nanomina(0),
            memo: None,
            nonce: None,
        }
    }

    /// Set the delegate's public key.
    pub fn to(mut self, delegate_to: &str) -> Self {
        self.delegate_to = delegate_to.to_string();
        self
    }

    /// Set the transaction fee.
    pub fn fee(mut self, fee: Currency) -> Self {
        self.fee = fee;
        self
    }

    /// Set an optional memo.
    pub fn memo(mut self, memo: &str) -> Self {
        self.memo = Some(memo.to_string());
        self
    }

    /// Set an explicit nonce (otherwise the daemon picks the next nonce).
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }
}

/// A pooled user command from the transaction pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PooledUserCommand {
    pub id: String,
    pub hash: String,
    pub kind: String,
    pub nonce: String,
    pub amount: String,
    pub fee: String,
    pub from: String,
    pub to: String,
}
