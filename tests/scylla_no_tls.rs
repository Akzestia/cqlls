/*
    MIT License

    Copyright (c) 2026 アクゼスティア
*/

use cqlls::config::CqllsConfig;
use cqlls::cqlsh::{check_connection, query_keyspaces};

#[tokio::test]
async fn test_connection_no_tls() {
    let mut config = CqllsConfig::default();

    config.known_nodes.push("127.0.0.1".to_string());

    let result = check_connection(&config).await;

    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
}

#[tokio::test]
async fn test_query_keyspaces_no_tls() {
    let mut config = CqllsConfig::default();

    config.known_nodes.push("127.0.0.1".to_string());

    let result = check_connection(&config).await;

    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let keyspaces = query_keyspaces(&config).await;

    assert!(
        keyspaces.is_ok(),
        "Failed to query keyspaces: {:?}",
        keyspaces.err()
    );
}
