<div align="center">

### cqlls (CQL Language Server) 

[![Crates.io](https://img.shields.io/crates/v/cqlls.svg)](https://crates.io/crates/cqlls)</br>

The Best Open Source **language server** for CQL (Cassandra Query Language) ^_^ 

  <img width="354" height="373.5" alt="image" src="https://media.tenor.com/YOphBzO0MfoAAAAi/japanese-animation.gif" />
</div>

------------

### IDE Integrations

- [Nvim](https://github.com/Akzestia/nvim-cql-v2)

### Installation

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

> [!TIP]
> All parts of the config are independent from each other, so if, for example, you only want to configure `formatting settings` and `db url`, you can do something like this:
> ```txt
> db {
>     known_nodes: {
>         "172.0.0.2:9042"
>     }
> }
>
> fmt {
>     indent: 2
> }
> ``` 

If no config file was provided, server will default to the following

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

### db

Just configures your database connection.

### fmt

Indent vs padding

```cql
-- Inside create table
CREATE TABLE IF NOT EXISTS users (
/*Indent*/ id           /*Padding*/ uuid,
/*Indent*/ display_name /*Padding*/ text,
);
```

```cql
-- Inside insert
INSERT INTO users (
/*Indent*/ user_id,
/*Indent*/ username,
/*Indent*/ contact,
/*Indent*/ addresses,
/*Indent*/ tags,
/*Indent*/ preferences,
/*Indent*/ last_login,
/*Indent*/ created_at
)
VALUES (
/*Indent*/ uuid(),
/*Indent*/ 'bob_builder',
/*Indent*/ {
/*Indent*/ /*Indent*/ email: 'bob@example.com',
/*Indent*/ /*Indent*/ phone: '+1-555-0202',
/*Indent*/ /*Indent*/ preferred_time: {
/*Indent*/ /*Indent*/ /*Indent*/ 'morning'
/*Indent*/ /*Indent*/ }
/*Indent*/ },
/*Indent*/ {
/*Indent*/ /*Indent*/ 'home': {
/*Indent*/ /*Indent*/ /*Indent*/ street: '789 Oak Ave',
/*Indent*/ /*Indent*/ /*Indent*/ city: 'Austin',
/*Indent*/ /*Indent*/ /*Indent*/ zip_code: '73301',
/*Indent*/ /*Indent*/ /*Indent*/ country: 'USA'
/*Indent*/ /*Indent*/ }
/*Indent*/ },
/*Indent*/ {
/*Indent*/ /*Indent*/ 'verified',
/*Indent*/ /*Indent*/ 'beta-tester'
/*Indent*/ },
/*Indent*/ {
/*Indent*/ /*Indent*/ 'theme': 'light',
/*Indent*/ /*Indent*/ 'language': 'en',
/*Indent*/ /*Indent*/ 'notifications': 'disabled'
/*Indent*/ },
/*Indent*/ toTimestamp(now()),
/*Indent*/ toTimestamp(now())
);
```

Padding is calculated in the following way for each field:

curr_field_padding = padding + max_field_len - curr_field_len;

```cql
-- Assuming padding is set to 8, max_field_len = 12 (display_name)
CREATE TABLE IF NOT EXISTS users (
/*Indent*/  id           /* 8 + 12 - 2  = 18 */  uuid,
/*Indent*/  display_name /* 8 + 12 - 12 = 8  */  text,
);
```

### features

Used to enable/disable a specific feature.

Currently `cqlls` has 2 features which are

- context_aware_completions
  - When enabled, `cqlls` will attempt to query the database, and provide a context aware completions, such as table names, keyspaces, table fields etc.
- diagnostics
  - When enabled, shows diagnostics. 

### debug

Enable/Disable logging.

Log file can be found at:

- MacOS: `~/Library/Application\ Support/cqlls/cqlls.log`
- Linux: `~/.local/share/cqlls/cqlls.log`

-------

> [!TIP]
> Diagnostics are currently available as an experimental feature, </br>
> and aren't stable yet ^_^

