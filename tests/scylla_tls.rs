/*
    Copyright (c) 2026 アクゼスティア. All Rights Reserved.
*/

use cqlls::config::CqllsConfig;
use cqlls::cqlsh::{check_connection, query_keyspaces};

#[tokio::test]
async fn test_connection_with_tls() {
    let config = CqllsConfig::with_knodes(vec!["127.0.0.1:9042".to_string()]);

    let session = check_connection(&config).await;

    match &session {
        Ok(s) => {
            println!("Keyspaces: {:?}", s);
        }
        _ => (),
    }

    assert!(session.is_ok(), "Failed to connect: {:?}", session.err());
    println!("Connected successfully with TLS!");
}

#[tokio::test]
async fn test_query_data() {
    let config = CqllsConfig::with_knodes(vec!["127.0.0.1:9042".to_string()]);

    let result = check_connection(&config).await;

    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let ksp = query_keyspaces(&config).await;
    println!("Keyspaces: {:?}", ksp.as_ref().unwrap().iter().clone());
    assert!(ksp.is_ok(), "Failed to query keyspaces: {:?}", ksp.err());
}
