# cqlls

[![Crates.io](https://img.shields.io/crates/v/cqlls.svg)](https://crates.io/crates/cqlls)</br>
  
The Best Open Source **language server** for CQL (Cassandra Query Language) ^_^

### List of supported IDE's

- [Nvim](https://github.com/Akzestia/nvim-cql-v2)

### cqlls vs Corpo

- Free
- Open Source (under MIT License)
- The Best Experience
- Seamless Integration with Nvim
- Written in Rust :D

### Installation

Install Language Server binary using cargo

```sh
cargo install cqlls
```

Add env variables to your shell config

```sh
# Adds cqlls to your path
export PATH="$HOME/.cargo/bin:$PATH"

# Env variables used by the language server

# Db related
export CQL_LSP_DB_URL="172.17.0.2"
export CQL_LSP_DB_PASSWD="cassandra"
export CQL_LSP_DB_USER="cassandra"
export CQL_LSP_TLS_MODE="none|tls"
export CQL_LSP_TLS_CA_CERT_FILE=""

# Logging and debugging
export CQL_LSP_ENABLE_LOGGING="false"

# Formatting related settings
export CQL_LSP_TYPE_ALIGNMENT_OFFSET="7"
```

### License

This project is licensed under the [MIT License](LICENSE).
