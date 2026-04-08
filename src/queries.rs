//! GraphQL query and mutation strings for the Mina daemon API.

pub const SYNC_STATUS: &str = r#"query { syncStatus }"#;

pub const DAEMON_STATUS: &str = r#"query {
    daemonStatus {
        syncStatus
        blockchainLength
        highestBlockLengthReceived
        uptimeSecs
        stateHash
        commitId
        peers {
            peerId
            host
            libp2pPort
        }
    }
}"#;

pub const NETWORK_ID: &str = r#"query { networkID }"#;

pub const GET_ACCOUNT: &str = r#"query ($publicKey: PublicKey!, $token: TokenId) {
    account(publicKey: $publicKey, token: $token) {
        publicKey
        nonce
        delegate
        tokenId
        balance {
            total
            liquid
            locked
        }
    }
}"#;

pub const BEST_CHAIN: &str = r#"query ($maxLength: Int) {
    bestChain(maxLength: $maxLength) {
        stateHash
        commandTransactionCount
        creatorAccount {
            publicKey
        }
        protocolState {
            consensusState {
                blockHeight
                slotSinceGenesis
                slot
            }
        }
    }
}"#;

pub const GET_PEERS: &str = r#"query {
    getPeers {
        peerId
        host
        libp2pPort
    }
}"#;

pub const POOLED_USER_COMMANDS: &str = r#"query ($publicKey: PublicKey) {
    pooledUserCommands(publicKey: $publicKey) {
        id
        hash
        kind
        nonce
        amount
        fee
        from
        to
    }
}"#;

pub const SEND_PAYMENT: &str = r#"mutation ($input: SendPaymentInput!) {
    sendPayment(input: $input) {
        payment {
            id
            hash
            nonce
        }
    }
}"#;

pub const SEND_DELEGATION: &str = r#"mutation ($input: SendDelegationInput!) {
    sendDelegation(input: $input) {
        delegation {
            id
            hash
            nonce
        }
    }
}"#;

pub const SET_SNARK_WORKER: &str = r#"mutation ($input: SetSnarkWorkerInput!) {
    setSnarkWorker(input: $input) {
        lastSnarkWorker
    }
}"#;

pub const SET_SNARK_WORK_FEE: &str = r#"mutation ($fee: UInt64!) {
    setSnarkWorkFee(input: {fee: $fee}) {
        lastFee
    }
}"#;
