use crate::config::*;
use crate::lsp::Document;
use dirs::data_dir;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio::sync::RwLock;

use crate::lsp::Backend;
use tower_lsp::LspService;
use url::Url;

pub async fn debug_format(debug_target: &str) {
    let pt = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(debug_target)
        .canonicalize()
        .expect("Debug target not found!");

    let config = match std::fs::read_to_string(".cqlls") {
        Ok(contents) => parse_config(&contents).unwrap_or_default(),
        _ => Default::default(),
    };

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
        current_document: RwLock::new(None),
        config,
    });

    let test_url = Url::from_file_path(&pt).unwrap();

    let text_test = fs::read_to_string(test_url.to_file_path().unwrap()).unwrap();
    let backend = service.inner();

    {
        let mut docs = backend.documents.write().await;
        docs.insert(test_url.clone(), text_test.clone());
    }

    let mut log_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    log_path.push("cqlls");
    std::fs::create_dir_all(&log_path).expect("Failed to create log directory");
    log_path.push("cqlls.log");

    // Open the file in write mode, create if missing, and truncate (overwrite) if it exists
    let log_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&log_path)
        .expect("Failed to open log file");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(log_file) // Pass the file handle directly instead of fern::log_file()
        .apply()
        .unwrap();

    let formatted_str_test = backend
        .format_test(&text_test.lines().collect(), &test_url)
        .await;

    println!("Formatted: {formatted_str_test}");
}

pub async fn run_format(cql_test: &Url, cql_expected: &Url) -> bool {
    let config = match std::fs::read_to_string(".cqlls") {
        Ok(contents) => parse_config(&contents).unwrap_or_default(),
        _ => Default::default(),
    };

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: RwLock::new(HashMap::new()),
        current_document: RwLock::new(None),
        config,
    });

    let text_test = fs::read_to_string(cql_test.to_file_path().unwrap()).unwrap();
    let text_expected = fs::read_to_string(cql_expected.to_file_path().unwrap()).unwrap();
    let backend = service.inner();

    {
        let mut docs = backend.documents.write().await;
        docs.insert(cql_test.clone(), text_test.clone());
        docs.insert(cql_expected.clone(), text_expected.clone());
    }

    let formatted_str_test = backend
        .format_test(&text_test.lines().collect(), cql_test)
        .await;

    let formatted_str_expected = backend
        .format_test(&text_expected.lines().collect(), cql_expected)
        .await;

    println!("Eq: {}", formatted_str_test == formatted_str_expected);
    formatted_str_test == formatted_str_expected
}

pub fn get_pt_pe(test_name: &str) -> (Url, Url) {
    let pt = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/cql/provided/{}.txt", test_name))
        .canonicalize()
        .expect("dwdw.cql not found — check it exists in project root");

    let test_url = Url::from_file_path(&pt).unwrap();

    let pe = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/cql/expected/{}.cql", test_name))
        .canonicalize()
        .expect("dwdw.cql not found — check it exists in project root");

    let expected_url = Url::from_file_path(&pe).unwrap();

    (test_url, expected_url)
}
