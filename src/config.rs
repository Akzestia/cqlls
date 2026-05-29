/*
    Copyright (c) 2026 アクゼスティア. All Rights Reserved.
*/

#[derive(Debug)]
pub enum TlsMode {
    None,
    Tls,
    MTls,
}

#[derive(Debug)]
pub enum DbType {
    DataStaxHCD,
    Scylla,
    Dynamo,
}

#[derive(Debug)]
pub struct CqllsConfig {
    pub db_type: DbType,
    pub preferred_dc: String,
    pub known_nodes: Vec<String>,
    pub tls: TlsMode,
    pub ca_cert: String,
    pub user: String,
    pub pswd: String,
    pub type_padding: u8,
    pub indent: u8,
    pub features: Vec<String>,
    pub logging: bool,
}

impl Default for CqllsConfig {
    fn default() -> Self {
        Self {
            db_type: DbType::Scylla,
            preferred_dc: String::new(),
            known_nodes: vec!["127.0.0.1:9042".to_string()],
            tls: TlsMode::None,
            ca_cert: String::new(),
            user: "cassandra".to_string(),
            pswd: "cassandra".to_string(),
            type_padding: 8,
            indent: 4,
            features: vec!["context_aware_completions".to_string()],
            logging: false,
        }
    }
}

impl CqllsConfig {
    pub fn with_knodes(nodes: Vec<String>) -> Self {
        Self {
            db_type: DbType::Scylla,
            preferred_dc: String::new(),
            known_nodes: nodes,
            tls: TlsMode::None,
            ca_cert: String::new(),
            user: "cassandra".to_string(),
            pswd: "cassandra".to_string(),
            type_padding: 8,
            indent: 4,
            features: vec!["context_aware_completions".to_string()],
            logging: false,
        }
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub msg: String,
}

impl ParseError {
    fn new(line: usize, msg: impl Into<String>) -> Self {
        Self {
            line,
            msg: msg.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error at line {}: {}", self.line, self.msg)
    }
}

impl std::error::Error for ParseError {}

fn meaningful_lines(src: &str) -> impl Iterator<Item = (usize, &str)> {
    src.lines()
        .enumerate()
        .map(|(i, line)| {
            let stripped = line
                .find("//")
                .or_else(|| line.find('#'))
                .or_else(|| line.find("--"))
                .map(|pos| &line[..pos])
                .unwrap_or(line);
            (i + 1, stripped.trim())
        })
        .filter(|(_, l)| !l.is_empty())
}

fn unquote(s: &str, line: usize) -> Result<&str, ParseError> {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        Ok(&s[1..s.len() - 1])
    } else {
        Err(ParseError::new(
            line,
            format!("expected quoted string, got `{s}`"),
        ))
    }
}

fn split_kv(line_str: &str, line: usize) -> Result<(&str, &str), ParseError> {
    let (key, rest) = line_str
        .split_once(':')
        .ok_or_else(|| ParseError::new(line, format!("expected `key: value`, got `{line_str}`")))?;
    Ok((key.trim(), rest.trim().trim_end_matches(',')))
}

pub fn parse_config(src: &str) -> Result<CqllsConfig, ParseError> {
    let mut cfg = CqllsConfig::default();
    let lines: Vec<(usize, &str)> = meaningful_lines(src).collect();
    let mut i = 0;

    while i < lines.len() {
        let (ln, text) = lines[i];

        let block_name = text.trim_end_matches('{').trim();

        if !text.ends_with('{') {
            return Err(ParseError::new(
                ln,
                format!("expected block opening `{{`, got `{text}`"),
            ));
        }

        i += 1;

        match block_name {
            "db" => {
                while i < lines.len() {
                    let (ln, text) = lines[i];
                    if text == "}" {
                        i += 1;
                        break;
                    }

                    if text.starts_with("known_nodes") {
                        cfg.known_nodes.clear();
                        i += 1;
                        while i < lines.len() {
                            let (ln, inner) = lines[i];
                            let inner = inner.trim_end_matches(',');
                            if inner == "}" || inner == "}," {
                                i += 1;
                                break;
                            }
                            let node = unquote(inner, ln)?;
                            cfg.known_nodes.push(node.to_string());
                            i += 1;
                        }
                        continue;
                    }

                    let (key, val) = split_kv(text, ln)?;
                    match key {
                        "type" => {
                            cfg.db_type = match unquote(val, ln)? {
                                "datastax_hcd" => DbType::DataStaxHCD,
                                "scylla" => DbType::Scylla,
                                "dynamo" => DbType::Dynamo,
                                other => {
                                    return Err(ParseError::new(
                                        ln,
                                        format!("unknown db type `{other}`"),
                                    ));
                                }
                            };
                        }
                        "preferred_dc" => cfg.preferred_dc = unquote(val, ln)?.to_string(),
                        "tls" => {
                            cfg.tls = match unquote(val, ln)? {
                                "none" => TlsMode::None,
                                "tls" => TlsMode::Tls,
                                "mtls" => TlsMode::MTls,
                                other => {
                                    return Err(ParseError::new(
                                        ln,
                                        format!("unknown tls mode `{other}`"),
                                    ));
                                }
                            };
                        }
                        "ca_cert" => cfg.ca_cert = unquote(val, ln)?.to_string(),
                        "user" => cfg.user = unquote(val, ln)?.to_string(),
                        "pswd" => cfg.pswd = unquote(val, ln)?.to_string(),
                        other => {
                            return Err(ParseError::new(ln, format!("unknown db key `{other}`")));
                        }
                    }
                    i += 1;
                }
            }

            "fmt" => {
                while i < lines.len() {
                    let (ln, text) = lines[i];
                    if text == "}" {
                        i += 1;
                        break;
                    }
                    let (key, val) = split_kv(text, ln)?;
                    match key {
                        "type_padding" => {
                            cfg.type_padding = val.parse::<u8>().map_err(|_| {
                                ParseError::new(
                                    ln,
                                    format!("`type_padding` must be u8, got `{val}`"),
                                )
                            })?;
                        }
                        "indent" => {
                            cfg.indent = val.parse::<u8>().map_err(|_| {
                                ParseError::new(ln, format!("`indent` must be u8, got `{val}`"))
                            })?;
                        }
                        other => {
                            return Err(ParseError::new(ln, format!("unknown fmt key `{other}`")));
                        }
                    }
                    i += 1;
                }
            }

            "features" => {
                cfg.features.clear();
                while i < lines.len() {
                    let (ln, text) = lines[i];
                    if text == "}" {
                        i += 1;
                        break;
                    }
                    let (key, val) = split_kv(text, ln)?;
                    match val {
                        "true" => cfg.features.push(key.to_string()),
                        "false" => {}
                        other => {
                            return Err(ParseError::new(
                                ln,
                                format!("feature value must be true/false, got `{other}`"),
                            ));
                        }
                    }
                    i += 1;
                }
            }

            "debug" => {
                while i < lines.len() {
                    let (ln, text) = lines[i];
                    if text == "}" {
                        i += 1;
                        break;
                    }
                    let (key, val) = split_kv(text, ln)?;
                    match key {
                        "logging" => {
                            cfg.logging = match val {
                                "true" => true,
                                "false" => false,
                                other => {
                                    return Err(ParseError::new(
                                        ln,
                                        format!("`logging` must be true/false, got `{other}`"),
                                    ));
                                }
                            };
                        }
                        other => {
                            return Err(ParseError::new(
                                ln,
                                format!("unknown debug key `{other}`"),
                            ));
                        }
                    }
                    i += 1;
                }
            }

            other => return Err(ParseError::new(ln, format!("unknown block `{other}`"))),
        }
    }

    Ok(cfg)
}
