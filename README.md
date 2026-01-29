# cqlls

[![Crates.io](https://img.shields.io/crates/v/cqlls.svg)](https://crates.io/crates/cqlls)</br>
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dakzestia%26type%3Dpatrons&style=for-the-badge)](https://patreon.com/akzestia)

> [!TIP]
> OpenSource grammar, highlights and general IDE integration

- [Nvim](https://github.com/Akzestia/nvim-cql-v2)
- [Zed](https://github.com/Akzestia/zed-cql)

The Best Open Source **language server** for CQL (Cassandra Query Language) ^_^

https://github.com/user-attachments/assets/780f9005-d571-489d-93e3-e61f91dcb0fe

> [!TIP]
> CQL is now supported by GitHub | [github-linguist v9.4.0](https://github.com/github-linguist/linguist/releases/tag/v9.4.0)
> Example using `cql` instad of `sql`
>
> ```cql
> USE "japanese";
>
> INSERT INTO users (
>     ユーザーID,
>     ユーザー名,
>     連絡先,
>     住所一覧,
>     タグ,
>     設定,
>     最終ログイン,
>     作成日時
> )
> VALUES (
>     uuid(),
>     '不思議の国のアリス',
>     {
>         メール: 'アリス@example.com',
>         電話番号: '+81-90-1234-5678',
>         希望連絡時間: {
>             '朝',
>             '夕方'
>         }
>     },
>     {
>         '自宅': {
>             通り: '東京都渋谷区神宮前1-2-3',
>             市区町村: '東京',
>             郵便番号: '150-0001',
>             国: '日本'
>         },
>         '職場': {
>             通り: '東京都千代田区丸の内4-5-6',
>             市区町村: '東京',
>             郵便番号: '100-0005',
>             国: '日本'
>         }
>     },
>     {
>         'プレミアム',
>         '認証済み',
>         '早期採用者'
>     },
>     {
>         'テーマ': 'ダーク',
>         '言語': '日本語',
>         '通知': '有効'
>     },
>     toTimestamp(now()),
>     toTimestamp(now())
> );
> ```

# cqlls vs Corpo

- Free
- Open Source (under MIT License)
- The Best Experience
- Seamless Integration with Zed && Nvim
- Written in Rust :D

# Installation

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

## License

This project is licensed under the [MIT License](LICENSE).
