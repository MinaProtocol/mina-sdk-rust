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
