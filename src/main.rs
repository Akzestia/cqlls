use cql_lsp::cqlsh::CqlSettings;
use cql_lsp::lsp::{Backend, FormattingSettings};
use cql_lsp::setup::setup_logger;
use log::info;
use std::collections::HashMap;
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use tower_lsp::{LspService, Server};

/*
    Based on DataStax HCD && CQL versions 3.4+

    [HCD]
    https://docs.datastax.com/en/cql/hcd/reference/cql-reference-about.html
    [CQL]
    https://cassandra.apache.org/doc/latest/cassandra/developing/cql/cql_singlefile.html

    Note!

    Some of the default CQL functions will be different because of DataStax HCD extensions
*/

/*
    Default values for localhosted DB (Tested With ScyllaDB)

    [LocalHost]
    CQL_LSP_DB_URL = "127.0.0.1:9042"
    CQL_LSP_DB_PASSWD = "cassandra"
    CQL_LSP_DB_USER = "cassandra"
    CQL_LSP_ENABLE_LOGGING = false | Used for development
    CQL_LSP_TYPE_ALIGNMENT_OFFSET = 7

    [Dockerults]
    CQL_LSP_DB_URL = "172.17.0.2:9042"
    CQL_LSP_DB_PASSWD = "cassandra"
    CQL_LSP_DB_USER = "cassandra"
    CQL_LSP_ENABLE_LOGGING = false | Used for development
    CQL_LSP_TYPE_ALIGNMENT_OFFSET = 7
*/

/*
    Lowercase keyword support

    This CQL LSP implementation supports lowercase usage for almost
    all keyword types, even though not all lowercase keywords are
    valid in standard CQL syntax. This approach helps future-proof
    the LSP implementation.
*/

/// Docs https://akzestia.dev/cqlls
///
///
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starting LSP
    let enable_logging = std::env::var("CQL_LSP_ENABLE_LOGGING").unwrap_or_else(|_| {
        info!("Logging mode wasn't provided. Setting Logging mode to default(false)");
        "false".to_string()
    });

    if enable_logging == "true" {
        println!("Setting up logger");
        setup_logger().unwrap_or_else(|e| println!("{e}"));
    }

    let url = std::env::var("CQL_LSP_DB_URL").unwrap_or_else(|_| {
        info!("Db url wasn't provided. Setting url to default(127.0.0.1)");
        "127.0.0.1".to_string()
    });
    let pswd = std::env::var("CQL_LSP_DB_PASSWD").unwrap_or_else(|_| {
        info!("Db pswd wasn't provided.\nSetting pswd to default(cassandra)");
        "cassandra".to_string()
    });
    let user = std::env::var("CQL_LSP_DB_USER").unwrap_or_else(|_| {
        info!("Db user wasn't provided.\nSetting user to default(cassandra)");
        "cassandra".to_string()
    });
    let type_alignment_offset = std::env::var("CQL_LSP_TYPE_ALIGNMENT_OFFSET").unwrap_or_else(|_| {
       info!("Type alignment offset wasn't provided.\n Setting type alignment offset to default 7");
       "7".to_string()
    });

    let settings = CqlSettings::from_env(&url, &pswd, &user);
    let formatting_settings = FormattingSettings::from_env(&type_alignment_offset);

    let stdin = stdin();
    let stdout = stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
        current_document: RwLock::new(None),
        config: settings,
        formatting_config: formatting_settings,
        indent: "    ".to_string(),
        max_line_length: 20,
    });

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
