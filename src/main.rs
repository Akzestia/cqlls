/*
MIT License

Copyright (c) 2025-2026 アクゼスティア

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use cql_lsp::cqlsh::{CqlSettings, TlsMode};
use cql_lsp::lsp::{Backend, FormattingSettings};
use cql_lsp::setup::setup_logger;
use log::info;
use std::collections::HashMap;
use std::fs::exists;
use std::panic;
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use tower_lsp::{LspService, Server};

/// Docs https://github.com/Akzestia/cqlls
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging Ssettings
    let enable_logging = std::env::var("CQL_LSP_ENABLE_LOGGING").unwrap_or_else(|_| {
        info!("Logging mode wasn't provided. Setting Logging mode to default(false)");
        "false".to_string()
    });

    if enable_logging == "true" {
        println!("Setting up logger");
        setup_logger().unwrap_or_else(|e| println!("{e}"));

        // Creates panic.log inside toor dir if server panics
        panic::set_hook(Box::new(|info| {
            let msg = format!("{info}\n");
            let _ = std::fs::write("panic.log", msg);
        }));
    }

    // DB connection settings
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

    // TLS settings
    let ca_cert_file = std::env::var("CQL_LSP_TLS_CA_CERT_FILE").unwrap_or_else(|_| {
        info!("Cert file wasn't provided, TLS && mTLS are disabled");
        "".to_string()
    });

    let mut tls_mode = std::env::var("CQL_LSP_TLS_MODE").unwrap_or_else(|_| {
        info!("TLS mode wasn't set\nSetting default TLS mode to none");
        "none".to_string()
    });

    // Set tls mode to none if cert wasn't provided
    if !exists(&ca_cert_file).is_ok() {
        tls_mode = "none".to_string();
    }

    // Formatting settings
    let type_alignment_offset = std::env::var("CQL_LSP_TYPE_ALIGNMENT_OFFSET").unwrap_or_else(|_| {
       info!("Type alignment offset wasn't provided.\n Setting type alignment offset to default 7");
       "7".to_string()
    });

    let tls = match tls_mode.as_str() {
        "tls" => TlsMode::Tls {
            ca_cert_path: ca_cert_file,
        },
        _ => TlsMode::None,
    };

    let formatting_settings = FormattingSettings::from_env(&type_alignment_offset);
    let settings = match tls {
        TlsMode::Tls { ca_cert_path } => {
            let settings = CqlSettings::from_env(&url, &pswd, &user).with_tls(ca_cert_path);
            settings
        }
        TlsMode::None => {
            let settings = CqlSettings::from_env(&url, &pswd, &user);
            settings
        }
    };

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
