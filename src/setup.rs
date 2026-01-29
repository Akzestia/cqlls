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
use dirs::data_dir;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug, Clone)]
pub struct SetupConfig {
    // Db related
    pub password: String,
    pub user_name: String,
    pub ip_addr: String,

    // Formatting & Suggestions
    pub context_based_select: bool,
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    let mut log_path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    log_path.push("cql_lsp");
    std::fs::create_dir_all(&log_path).expect("Failed to create log directory");
    log_path.push("output.log");

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
        .chain(fern::log_file(log_path)?)
        .apply()?;

    Ok(())
}

pub fn setup_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    config_path.push("cql_lsp/config.lsp");
    println!("Config: {:?}", config_path.to_str());

    if !config_path.exists() {
        let mut file =
            File::create_new(config_path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        write!(
            file,
            "[db_context]\npassword = \"{}\"\nuser = \"{}\"\nip = \"{}\"\n",
            "cassandara", "cassandara", "127.0.0.1"
        )?;

        return Ok(());
    }

    Ok(())
}
