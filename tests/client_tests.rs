use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use mina_sdk::*;

fn test_config(uri: &str) -> ClientConfig {
    ClientConfig {
        graphql_uri: uri.to_string(),
        retries: 2,
        retry_delay: Duration::from_millis(10),
        timeout: Duration::from_secs(5),
    }
}

fn gql_response(data: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({ "data": data }))
}

fn gql_error(message: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "errors": [{ "message": message }]
    }))
}

#[tokio::test]
async fn test_sync_status_synced() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({ "syncStatus": "SYNCED" })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let status = client.get_sync_status().await.unwrap();
    assert_eq!(status, SyncStatus::Synced);
}

#[tokio::test]
async fn test_sync_status_bootstrap() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({ "syncStatus": "BOOTSTRAP" })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let status = client.get_sync_status().await.unwrap();
    assert_eq!(status, SyncStatus::Bootstrap);
}

#[tokio::test]
async fn test_daemon_status() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "daemonStatus": {
                "syncStatus": "SYNCED",
                "blockchainLength": 100,
                "highestBlockLengthReceived": 100,
                "uptimeSecs": 3600,
                "stateHash": "3NL...",
                "commitId": "abc123",
                "peers": [
                    { "peerId": "peer1", "host": "1.2.3.4", "libp2pPort": 8302 }
                ]
            }
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let status = client.get_daemon_status().await.unwrap();
    assert_eq!(status.sync_status, SyncStatus::Synced);
    assert_eq!(status.blockchain_length, Some(100));
    assert_eq!(status.uptime_secs, Some(3600));
    assert_eq!(status.state_hash.as_deref(), Some("3NL..."));
    assert_eq!(status.commit_id.as_deref(), Some("abc123"));
    let peers = status.peers.unwrap();
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].peer_id, "peer1");
    assert_eq!(peers[0].host, "1.2.3.4");
    assert_eq!(peers[0].port, 8302);
}

#[tokio::test]
async fn test_network_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({ "networkID": "mina:testnet" })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let id = client.get_network_id().await.unwrap();
    assert_eq!(id, "mina:testnet");
}

#[tokio::test]
async fn test_get_account() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "account": {
                "publicKey": "B62qtest",
                "nonce": "5",
                "delegate": "B62qdelegate",
                "tokenId": "1",
                "balance": {
                    "total": "10000000000",
                    "liquid": "8000000000",
                    "locked": "2000000000"
                }
            }
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let account = client.get_account("B62qtest", None).await.unwrap();
    assert_eq!(account.public_key, "B62qtest");
    assert_eq!(account.nonce, 5);
    assert_eq!(account.delegate.as_deref(), Some("B62qdelegate"));
    assert_eq!(account.balance.total.nanomina(), 10_000_000_000);
    assert_eq!(account.balance.liquid.unwrap().nanomina(), 8_000_000_000);
    assert_eq!(account.balance.locked.unwrap().nanomina(), 2_000_000_000);
}

#[tokio::test]
async fn test_get_account_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({ "account": null })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let result = client.get_account("B62qnonexistent", None).await;
    assert!(matches!(result, Err(Error::AccountNotFound(_))));
}

#[tokio::test]
async fn test_best_chain() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "bestChain": [{
                "stateHash": "3NLhash",
                "commandTransactionCount": 2,
                "creatorAccount": { "publicKey": "B62qcreator" },
                "protocolState": {
                    "consensusState": {
                        "blockHeight": "42",
                        "slotSinceGenesis": "100",
                        "slot": "50"
                    }
                }
            }]
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let chain = client.get_best_chain(Some(1)).await.unwrap();
    assert_eq!(chain.len(), 1);
    assert_eq!(chain[0].state_hash, "3NLhash");
    assert_eq!(chain[0].height, 42);
    assert_eq!(chain[0].global_slot_since_genesis, 100);
    assert_eq!(chain[0].global_slot_since_hard_fork, 50);
    assert_eq!(chain[0].creator_pk, "B62qcreator");
    assert_eq!(chain[0].command_transaction_count, 2);
}

#[tokio::test]
async fn test_best_chain_empty() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({ "bestChain": null })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let chain = client.get_best_chain(None).await.unwrap();
    assert!(chain.is_empty());
}

#[tokio::test]
async fn test_get_peers() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "getPeers": [
                { "peerId": "p1", "host": "10.0.0.1", "libp2pPort": 8302 },
                { "peerId": "p2", "host": "10.0.0.2", "libp2pPort": 8303 }
            ]
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let peers = client.get_peers().await.unwrap();
    assert_eq!(peers.len(), 2);
    assert_eq!(peers[0].peer_id, "p1");
    assert_eq!(peers[1].host, "10.0.0.2");
}

#[tokio::test]
async fn test_pooled_user_commands() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "pooledUserCommands": [{
                "id": "cmd1",
                "hash": "CkpHash",
                "kind": "PAYMENT",
                "nonce": "1",
                "amount": "1000000000",
                "fee": "10000000",
                "from": "B62qsender",
                "to": "B62qreceiver"
            }]
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let cmds = client
        .get_pooled_user_commands(Some("B62qsender"))
        .await
        .unwrap();
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0].kind, "PAYMENT");
    assert_eq!(cmds[0].hash, "CkpHash");
}

#[tokio::test]
async fn test_send_payment() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "sendPayment": {
                "payment": {
                    "id": "pay1",
                    "hash": "CkpPayHash",
                    "nonce": "3"
                }
            }
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let result = client
        .send_payment(
            "B62qsender",
            "B62qreceiver",
            Currency::from_mina("1.5").unwrap(),
            Currency::from_mina("0.01").unwrap(),
            Some("test memo"),
            None,
        )
        .await
        .unwrap();
    assert_eq!(result.id, "pay1");
    assert_eq!(result.hash, "CkpPayHash");
    assert_eq!(result.nonce, 3);
}

#[tokio::test]
async fn test_send_delegation() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "sendDelegation": {
                "delegation": {
                    "id": "del1",
                    "hash": "CkpDelHash",
                    "nonce": "7"
                }
            }
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let result = client
        .send_delegation(
            "B62qsender",
            "B62qdelegate",
            Currency::from_mina("0.01").unwrap(),
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(result.id, "del1");
    assert_eq!(result.hash, "CkpDelHash");
    assert_eq!(result.nonce, 7);
}

#[tokio::test]
async fn test_graphql_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_error("field not found"))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let result = client.get_sync_status().await;
    assert!(matches!(result, Err(Error::Graphql { .. })));
    let err = result.unwrap_err();
    assert!(err.to_string().contains("field not found"));
}

#[tokio::test]
async fn test_connection_error_after_retries() {
    // Point to a port that nothing is listening on
    let client = MinaClient::with_config(ClientConfig {
        graphql_uri: "http://127.0.0.1:19999/graphql".to_string(),
        retries: 2,
        retry_delay: Duration::from_millis(10),
        timeout: Duration::from_secs(1),
    });

    let result = client.get_sync_status().await;
    assert!(matches!(result, Err(Error::Connection { .. })));
}

#[tokio::test]
async fn test_set_snark_worker() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "setSnarkWorker": { "lastSnarkWorker": "B62qold" }
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let prev = client.set_snark_worker(Some("B62qnew")).await.unwrap();
    assert_eq!(prev.as_deref(), Some("B62qold"));
}

#[tokio::test]
async fn test_set_snark_work_fee() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({
            "setSnarkWorkFee": { "lastFee": "100000000" }
        })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let prev = client
        .set_snark_work_fee(Currency::from_mina("0.5").unwrap())
        .await
        .unwrap();
    assert_eq!(prev, "100000000");
}

#[tokio::test]
async fn test_execute_custom_query() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(gql_response(json!({ "version": "1.2.3" })))
        .mount(&server)
        .await;

    let client = MinaClient::with_config(test_config(&format!("{}/graphql", server.uri())));
    let data = client
        .execute_query("query { version }", None, "custom")
        .await
        .unwrap();
    assert_eq!(data["version"].as_str(), Some("1.2.3"));
}
