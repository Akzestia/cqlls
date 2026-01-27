use cql_lsp::cqlsh::{CqlSettings, TlsMode, check_connection};

#[tokio::test]
async fn test_connection_no_tls() {
    let config = CqlSettings::new();

    let result = check_connection(&config).await;

    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
}

#[tokio::test]
async fn test_connection_with_tls() {
    let ca_cert_file = std::env::var("CQL_LSP_TLS_CA_CERT_FILE").unwrap_or_else(|_| {
        "".to_string()
    });
    let config = CqlSettings::from_env("172.17.0.2:9042", "cassandra", "cassandra")
        .with_tls(ca_cert_file);

    let result = check_connection(&config).await;

    assert!(
        result.is_ok(),
        "Failed to connect with TLS: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_connection_invalid_host() {
    let config = CqlSettings::from_env("invalid-host:9042", "cassandra", "cassandra");

    let result = check_connection(&config).await;

    assert!(result.is_err(), "Should fail with invalid host");
}

#[tokio::test]
async fn test_connection_wrong_credentials() {
    let config = CqlSettings::from_env("127.0.0.1:9042", "wrong_password", "wrong_user");

    let result = check_connection(&config).await;

    assert!(result.is_err(), "Should fail with wrong credentials");
}
