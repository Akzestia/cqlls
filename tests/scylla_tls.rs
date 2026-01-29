use cql_lsp::cqlsh::{CqlSettings, check_connection, query_keyspaces};

const SCYLLA_HOST: &str = "127.0.0.1:9042";

#[tokio::test]
async fn test_connection_with_tls() {
    let config = CqlSettings::new().with_tls("./certs/ca.crt");

    let session = check_connection(&config).await;

    match &session {
        Ok(s) => {
            println!("Keyspaces: {:?}", s);
        }
        _ => {}
    }

    assert!(session.is_ok(), "Failed to connect: {:?}", session.err());
    println!("Connected successfully with TLS!");
}

#[tokio::test]
async fn test_query_data() {
    let ca_cert_file = std::env::var("CQL_LSP_TLS_CA_CERT_FILE").unwrap_or_else(|_| "".to_string());
    let config =
        CqlSettings::from_env(SCYLLA_HOST, "cassandra", "cassandra").with_tls(ca_cert_file);
    let result = check_connection(&config).await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let ksp = query_keyspaces(&config).await;
    println!("Keyspaces: {:?}", ksp.as_ref().unwrap().iter().clone());
    assert!(ksp.is_ok(), "Failed to query keyspaces: {:?}", ksp.err());
}
