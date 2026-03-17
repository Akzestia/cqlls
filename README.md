# cqlls

[![Crates.io](https://img.shields.io/crates/v/cqlls.svg)](https://crates.io/crates/cqlls)</br>
  
The Best Open Source **language server** for CQL (Cassandra Query Language) ^_^

### List of IDE Integrations via plugins|extensions

- [Nvim](https://github.com/Akzestia/nvim-cql-v2)

### Installation

Install Language Server binary using cargo

```sh
cargo install cqlls
```

Add env variables to your shell config

```sh
# Adds cqlls to your path
export PATH="$HOME/.cargo/bin:$PATH"
```

Env variables used by the language server
```sh
# Db related
export CQL_LS_DB_URL="172.17.0.2"
export CQL_LS_DB_PASSWD="cassandra"
export CQL_LS_DB_USER="cassandra"
export CQL_LS_TLS_MODE="none|tls"
export CQL_LS_TLS_CA_CERT_FILE=""

# Logging and debugging
export CQL_LS_ENABLE_LOGGING="false"

# Formatting related settings
export CQL_LS_TYPE_ALIGNMENT_OFFSET="7"

# Diagnostics
export CQL_LS_DIAGNOSTICS="false"
```

> [!TIP]
> Diagnostics are currently available as an experimental feature, </br>
> and aren't stable yet ^_^

### License

This project is licensed under the [MIT License](LICENSE).
