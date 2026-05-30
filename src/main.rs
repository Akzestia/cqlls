/*
    Copyright (c) 2026 アクゼスティア. All Rights Reserved.
*/

use cqlls::logger::setup_logger;
use cqlls::lsp::Backend;
use cqlls::{cmd, config::*};
use std::collections::HashMap;
use std::panic;
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 2 {
        return cmd::exec(&args).await;
    }

    let config = CqllsConfig::try_from_config_file();

    if config.logging {
        println!("Setting up logger");
        setup_logger().unwrap_or_else(|e| println!("{e}"));
        panic::set_hook(Box::new(|info| {
            let msg = format!("{info}\n");
            let _ = std::fs::write("panic.log", msg);
        }));
    }

    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
        current_document: RwLock::new(None),
        config,
    });

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
