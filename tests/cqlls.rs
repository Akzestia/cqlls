/*
    Copyright (c) 2026 アクゼスティア. All Rights Reserved.
*/

use cqlls::config::*;
use cqlls::test_base::{get_pt_pe, run_format};

const SAMPLE: &str = r#"
    db {
        type: "scylla"
        preferred_dc: "us-east-1"
        known_nodes: {
            "127.0.0.1:9042",
            "127.0.0.2:9042"
        },
        tls: "mtls"
        ca_cert: "/path/to/ca_cert"
    }
    fmt {
        type_padding: 8
        indent: 4
    }
    features {
        context_aware_completions: true
        diagnostics: false
    }
    debug {
        logging: true
    }
"#;

#[test]
fn test_parse_full() {
    let cfg = parse_config(SAMPLE).expect("parse failed");
    assert!(matches!(cfg.db_type, DbType::Scylla));
    assert_eq!(cfg.preferred_dc, "us-east-1");
    assert_eq!(cfg.known_nodes, vec!["127.0.0.1:9042", "127.0.0.2:9042"]);
    assert!(matches!(cfg.tls, TlsMode::MTls));
    assert_eq!(cfg.ca_cert, "/path/to/ca_cert");
    assert_eq!(cfg.type_padding, 8);
    assert_eq!(cfg.indent, 4);
    assert_eq!(cfg.has_feature("context_aware_completions"), true);
    assert_eq!(cfg.has_feature("diagnostics"), false);
    assert!(cfg.logging);
}

#[test]
fn test_unknown_block() {
    let err = parse_config("bogus {\n}\n").unwrap_err();
    assert!(err.msg.contains("unknown block"));
}

#[test]
fn test_bad_db_type() {
    let err = parse_config("db {\ntype: \"oracle\"\n}\n").unwrap_err();
    assert!(err.msg.contains("unknown db type"));
}

#[test]
fn test_defaults_preserved() {
    let cfg = parse_config("db {\n}\n").unwrap();
    assert!(matches!(cfg.db_type, DbType::Scylla));
    assert_eq!(cfg.type_padding, 8);
    assert!(!cfg.logging);
}

#[tokio::test]
async fn test_alter_keyspace() {
    let (pt, pe) = get_pt_pe("alter_keyspace");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_alter_materialized_view() {
    let (pt, pe) = get_pt_pe("alter_materialized_view");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_alter_role() {
    let (pt, pe) = get_pt_pe("alter_role");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_alter_table() {
    let (pt, pe) = get_pt_pe("alter_table");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_alter_type() {
    let (pt, pe) = get_pt_pe("alter_type");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_alter_user() {
    let (pt, pe) = get_pt_pe("alter_user");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_batch() {
    let (pt, pe) = get_pt_pe("batch");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_comments() {
    let (pt, pe) = get_pt_pe("comments");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_commit_search_index() {
    let (pt, pe) = get_pt_pe("commit_search_index");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_consistency_level() {
    let (pt, pe) = get_pt_pe("consistency_level");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_aggregate() {
    let (pt, pe) = get_pt_pe("create_aggregate");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_function() {
    let (pt, pe) = get_pt_pe("create_function");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_index() {
    let (pt, pe) = get_pt_pe("create_index");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_keyspace() {
    let (pt, pe) = get_pt_pe("create_keyspace");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_materialized_view() {
    let (pt, pe) = get_pt_pe("create_materialized_view");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_role() {
    let (pt, pe) = get_pt_pe("create_role");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_search_index() {
    let (pt, pe) = get_pt_pe("create_search_index");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_table() {
    let (pt, pe) = get_pt_pe("create_table");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_type() {
    let (pt, pe) = get_pt_pe("create_type");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_create_user() {
    let (pt, pe) = get_pt_pe("create_user");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_delete() {
    let (pt, pe) = get_pt_pe("delete");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_aggregate() {
    let (pt, pe) = get_pt_pe("drop_aggregate");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_function() {
    let (pt, pe) = get_pt_pe("drop_function");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_index() {
    let (pt, pe) = get_pt_pe("drop_index");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_keyspace() {
    let (pt, pe) = get_pt_pe("drop_keyspace");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_materialized_view() {
    let (pt, pe) = get_pt_pe("drop_materialized_view");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_role() {
    let (pt, pe) = get_pt_pe("drop_role");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_search_index() {
    let (pt, pe) = get_pt_pe("drop_search_index");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_table() {
    let (pt, pe) = get_pt_pe("drop_table");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_type() {
    let (pt, pe) = get_pt_pe("drop_type");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_drop_user() {
    let (pt, pe) = get_pt_pe("drop_user");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_grant_permission() {
    let (pt, pe) = get_pt_pe("grant_permission");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_grant_role() {
    let (pt, pe) = get_pt_pe("grant_role");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_having() {
    let (pt, pe) = get_pt_pe("having");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_insert() {
    let (pt, pe) = get_pt_pe("insert");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_list_permissions() {
    let (pt, pe) = get_pt_pe("list_permissions");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_list_roles() {
    let (pt, pe) = get_pt_pe("list_roles");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_list_users() {
    let (pt, pe) = get_pt_pe("list_users");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_primary_key() {
    let (pt, pe) = get_pt_pe("primary_key");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_revoke_permissions() {
    let (pt, pe) = get_pt_pe("revoke_permissions");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_revoke_role() {
    let (pt, pe) = get_pt_pe("revoke_role");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_select() {
    let (pt, pe) = get_pt_pe("select");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_truncate() {
    let (pt, pe) = get_pt_pe("truncate");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_update() {
    let (pt, pe) = get_pt_pe("update");
    assert!(run_format(&pt, &pe).await)
}

#[tokio::test]
async fn test_use() {
    let (pt, pe) = get_pt_pe("use");
    assert!(run_format(&pt, &pe).await)
}
