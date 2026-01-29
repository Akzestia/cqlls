use cql_lsp::cqlsh::{CqlSettings, check_connection, query_keyspaces};

const SCYLLA_HOST: &str = "127.0.0.1:9042";


#[tokio::test]
async fn test_connection_no_tls() {
    let config = CqlSettings::new();

    let result = check_connection(&config).await;

    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
}

#[tokio::test]
async fn test_query_keyspaces_no_tls() {
    let config = CqlSettings::new();

    let result = check_connection(&config).await;

    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let keyspaces = query_keyspaces(&config).await;

    assert!(
        keyspaces.is_ok(),
        "Failed to query keyspaces: {:?}",
        keyspaces.err()
    );
}
