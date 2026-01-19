# cqlls

[![Crates.io](https://img.shields.io/crates/v/cql_lsp.svg)](https://crates.io/crates/cql_lsp)</br> 
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dakzestia%26type%3Dpatrons&style=for-the-badge)](https://patreon.com/akzestia)

> [!TIP]
> OpenSource grammar, highlights and general IDE integration

- [Nvim](https://github.com/Akzestia/nvim-cql-v2)
- [Zed](https://github.com/Akzestia/zed-cql)
- [VS Code](https://github.com/Akzestia/cqlTextMate)

The 1nonly Open Source **language server** for CQL (Cassandra Query Language) ^_^

https://github.com/user-attachments/assets/780f9005-d571-489d-93e3-e61f91dcb0fe

> [!TIP]
> CQL is now supported by GitHub | [github-linguist v9.4.0](https://github.com/github-linguist/linguist/releases/tag/v9.4.0)
> Example using `cql` instad of `sql`
> ```cql
> ALTER MATERIALIZED VIEW cycling.cyclist_by_age
> ALTER MATERIALIZED VIEW cycling.cyclist_by_age
> WITH comment = 'A most excellent and useful view'
> AND bloom_filter_fp_chance = 0.02;
> 
> ALTER MATERIALIZED VIEW cycling.cyclist_by_age
> WITH compression = {
>    'sstable_compression' : 'DeflateCompressor',
>   'chunk_length_kb' : 64
> }
> AND compaction = {
>    'class' : 'SizeTieredCompactionStrategy',
>    'max_threshold' : 64
> };
>
> ALTER MATERIALIZED VIEW cycling.cyclist_by_age
> WITH caching = {
>    'keys' : 'NONE',
>    'rows_per_partition' : '15'
> };
> ```

# cqlls vs Corpo 

- Free
- Open Source language server (under MIT License)
- Aiming to provide the best experience
- Seamless Integration with Zed && Nvim
- Written in Rust :D

# Installation

Install Language Server binary using cargo
```sh
cargo install cql_lsp
```

Add env variables to your shell config

```sh
export PATH="$HOME/.cargo/bin:$PATH"

export CQL_LSP_DB_URL="172.17.0.2"
export CQL_LSP_DB_PASSWD="cassandra"
export CQL_LSP_DB_USER="cassandra"
export CQL_LSP_ENABLE_LOGGING="false"
export CQL_LSP_TYPE_ALIGNMENT_OFFSET="7"
```

## License

This project is licensed under the [MIT License](LICENSE).
