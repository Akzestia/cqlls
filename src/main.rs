/*
    MIT License
    Copyright (c) 2026 アクゼスティア
*/
use cqlls::config::*;
use cqlls::logger::setup_logger;
use cqlls::lsp::Backend;
use cqlls::version;
use log::{error, info};
use std::collections::HashMap;
use std::panic;
use tokio::io::{stdin, stdout};
use tokio::sync::RwLock;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && (args[1] == "version" || args[1] == "-v") {
        println!("{}", version::version());
        return Ok(());
    }

    #[cfg(debug_assertions)]
    if args.len() == 3 && (args[1] == "--debug" || args[1] == "-d") {
        use cqlls::test_base::debug_format;
        println!("{}", version::version());
        debug_format(&args[2]).await;
        return Ok(());
    }

    let config = match std::fs::read_to_string(".cqlls") {
        Ok(contents) => match parse_config(&contents) {
            Ok(cfg) => {
                info!("Config: {:?}", cfg);
                cfg
            }
            Err(_) => {
                error!("Failed to parse .cqlls, using defaults");
                CqllsConfig::default()
            }
        },
        Err(_) => {
            info!("No .cqlls config file found, using defaults");
            CqllsConfig::default()
        }
    };

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
