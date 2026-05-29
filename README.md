<div align="center">

### cqlls (CQL Language Server) 

[![Crates.io](https://img.shields.io/crates/v/cqlls.svg)](https://crates.io/crates/cqlls)</br>

The Best Open Source **language server** for CQL (Cassandra Query Language) ^_^ 

  <img width="354" height="373.5" alt="image" src="https://media.tenor.com/YOphBzO0MfoAAAAi/japanese-animation.gif" />
</div>

------------

### List of IDE Integrations via plugins | extensions

- [Nvim](https://github.com/Akzestia/nvim-cql-v2)

### Installation

Install Language Server binary using cargo

```sh
cargo install cqlls
```

### Config

To configure cqlls you can add `.cqlls` file in the project root

```
db {
    type: "datastax_hcd|scylla|dynamo"
 
    preferred_dc: "us-east-1"
    known_nodes: {
        "127.0.0.1:9042",
        "127.0.0.1:9043"
    }

    tls: "none|tls|mtls"
    ca_cert: "/path/to/ca_cert"

    user: "cassandra"
    pswd: "cassandra"
}

fmt {
    type_padding: 0-255
    indent: 0-255
}

features {
    context_aware_completions: true|false
    diagnostics: true|false
}

debug {
    logging: true|false
}
```

If no config file was provided, server will defailt to the following

```
db {
    type: "scylla"

    preferred_dc: ""
    known_nodes: {
        "127.0.0.1:9042"
    }

    tls: "none"
    ca_cert: ""

    user: "cassandra"
    pswd: "cassandra"
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
    logging: false
}
```


> [!TIP]
> Diagnostics are currently available as an experimental feature, </br>
> and aren't stable yet ^_^

