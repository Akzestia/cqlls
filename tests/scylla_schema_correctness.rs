/*
    Copyright (c) 2026 アクゼスティア. All Rights Reserved.
*/

use cqlls::config::CqllsConfig;
use cqlls::cqlsh::{
    check_connection, query_aggregates, query_functions, query_g_fields, query_g_tables,
    query_hard_scoped_fields, query_indexes, query_keyspace_scoped_fields,
    query_keyspace_scoped_tables, query_keyspaces, query_types, query_views,
};

fn test_config() -> CqllsConfig {
    CqllsConfig::with_knodes(vec!["127.0.0.1:9042".to_string()])
}

#[tokio::test]
async fn test_connection_schema_ok() {
    let config = test_config();
    let result = check_connection(&config).await;
    assert!(
        result.is_ok(),
        "check_connection failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_query_keyspaces_schema_ok() {
    let config = test_config();
    let result = query_keyspaces(&config).await;
    assert!(
        result.is_ok(),
        "query_keyspaces schema mismatch or error: {:?}",
        result.err()
    );
    let keyspaces = result.unwrap();
    assert!(
        keyspaces
            .iter()
            .any(|ks| ks.keyspace_name == "system_schema"),
        "expected system_schema keyspace, got: {:?}",
        keyspaces
            .iter()
            .map(|k| &k.keyspace_name)
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_query_g_tables_schema_ok() {
    let config = test_config();
    let result = query_g_tables(&config).await;
    assert!(
        result.is_ok(),
        "query_g_tables schema mismatch or error: {:?}",
        result.err()
    );
    let tables = result.unwrap();
    assert!(!tables.is_empty(), "expected at least some system tables");
}

#[tokio::test]
async fn test_query_keyspace_scoped_tables_schema_ok() {
    let config = test_config();
    let result = query_keyspace_scoped_tables(&config, "system_schema").await;
    assert!(
        result.is_ok(),
        "query_keyspace_scoped_tables schema mismatch or error: {:?}",
        result.err()
    );
    let tables = result.unwrap();
    assert!(
        tables.iter().any(|t| t.table_name == "keyspaces"),
        "expected system_schema.keyspaces, got: {:?}",
        tables.iter().map(|t| t.united()).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_query_g_fields_schema_ok() {
    let config = test_config();
    let result = query_g_fields(&config).await;
    assert!(
        result.is_ok(),
        "query_g_fields schema mismatch or error: {:?}",
        result.err()
    );
    let columns = result.unwrap();
    assert!(
        !columns.is_empty(),
        "expected some columns from system tables"
    );
}

#[tokio::test]
async fn test_query_keyspace_scoped_fields_schema_ok() {
    let config = test_config();
    let result = query_keyspace_scoped_fields(&config, "system_schema").await;
    assert!(
        result.is_ok(),
        "query_keyspace_scoped_fields schema mismatch or error: {:?}",
        result.err()
    );
    let columns = result.unwrap();
    assert!(
        columns.iter().any(|c| c.column_name == "keyspace_name"),
        "expected keyspace_name column in system_schema.*, got: {:?}",
        columns.iter().map(|c| &c.column_name).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_query_hard_scoped_fields_schema_ok() {
    let config = test_config();
    let result = query_hard_scoped_fields(&config, "system_schema", "keyspaces").await;
    assert!(
        result.is_ok(),
        "query_hard_scoped_fields schema mismatch or error: {:?}",
        result.err()
    );
    let columns = result.unwrap();
    let names: Vec<&str> = columns.iter().map(|c| c.column_name.as_str()).collect();
    assert!(
        names.contains(&"keyspace_name") && names.contains(&"durable_writes"),
        "expected core columns on system_schema.keyspaces, got: {:?}",
        names
    );
}

#[tokio::test]
async fn test_query_aggregates_schema_ok() {
    let config = test_config();
    let result = query_aggregates(&config).await;
    assert!(
        result.is_ok(),
        "query_aggregates schema mismatch or error: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_query_functions_schema_ok() {
    let config = test_config();
    let result = query_functions(&config).await;
    assert!(
        result.is_ok(),
        "query_functions schema mismatch or error: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_query_indexes_schema_ok() {
    let config = test_config();
    let result = query_indexes(&config).await;
    assert!(
        result.is_ok(),
        "query_indexes schema mismatch or error: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_query_types_schema_ok() {
    let config = test_config();
    let result = query_types(&config).await;
    assert!(
        result.is_ok(),
        "query_types schema mismatch or error: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_query_views_schema_ok() {
    let config = test_config();
    let result = query_views(&config).await;
    assert!(
        result.is_ok(),
        "query_views schema mismatch or error: {:?}",
        result.err()
    );
}
