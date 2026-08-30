/*
    Copyright (c) 2026 アクゼスティア. All Rights Reserved.
*/

use once_cell::sync::Lazy;
use tower_lsp::lsp_types::*;

pub static CQL_KEYWORDS_LWC: &[&str] = &[
    "alter",
    "begin",
    "apply",
    "create",
    "drop",
    "grant",
    "list",
    "revoke",
    "select",
    "truncate",
    "use",
    "delete",
    "insert",
    "update",
    "restrcit",
    "unrestrict",
];

pub static CQL_TYPES_LWC: &[&str] = &[
    "ascii",
    "bigint",
    "blob",
    "boolean",
    "counter",
    "date",
    "decimal",
    "double",
    "float",
    "frozen",
    "inet",
    "int",
    "list",
    "map",
    "set",
    "smallint",
    "text",
    "time",
    "timestamp",
    "timeuuid",
    "tinyint",
    "tuple",
    "uuid",
    "varchar",
    "varint",
];

pub static KEYWORDS_STRINGS_LWC: &[&str] = &[
    "use",
    "alter",
    "create",
    "keyspace",
    "table",
    "with",
    "where",
    "if",
    "and",
    "set",
    "in",
    "to",
    "from",
    "using",
    "timestamp",
    "ttl",
    "exists",
    "not",
    "type",
    "view",
    "materialized",
    "replication",
    "durable_writes",
    "batch",
    "apply",
    "begin",
    "unlogged",
    "logged",
    "counter",
    "truncate",
    "insert",
    "into",
    "values",
    "update",
    "delete",
    "role",
    "password",
    "user",
    "superuser",
    "nosuperuser",
    "add",
    "drop",
    "rename",
    "compact",
    "storage",
    "contains",
    "key",
    "login",
    "options",
    "or",
    "replace",
    "sfunc",
    "stype",
    "finalfunc",
    "initcond",
    "language",
    "input",
    "on",
    "function",
    "called",
    "returns",
    "filtering",
    "distinct",
    "as",
    "keys",
    "group",
    "by",
    "json",
    "null",
    "custom",
    "aggregate",
    "all",
    "allow",
    "asc",
    "authorize",
    "clustering",
    "desc",
    "describe",
    "entries",
    "full",
    "grant",
    "index",
    "keyspaces",
    "limit",
    "modify",
    "norecursive",
    "of",
    "order",
    "partition",
    "per",
    "permissions",
    "primary",
    "revoke",
    "select",
    "users",
    "commit",
    "search",
    "roles",
    "deterministic",
    "monotonic",
    "java",
    "javascript",
    "is",
    "hashed",
    "access",
    "datacenters",
    "cidrs",
    "columns",
    "profiles",
    "config",
    "rows",
    "functions",
    "mbeans",
    "mbean",
    "pattern",
    "execute",
    "proxy",
    "id",
    "like",
    "ann",
    "offset",
    "list",
    "max",
    "min",
    "sum",
    "avg",
    "token",
    "writetime",
    "count",
    "infinity",
    "nan",
    "static",
    "any",
    "having",
    "consistency",
    "level",
    "one",
    "two",
    "three",
    "quorum",
    "local_one",
    "local_quorum",
    "each_quorum",
];

// (label, detail, documentation, insert_text)
type FnCompletionItem = (&'static str, &'static str, &'static str, &'static str);

const CQL_NATIVE_FUNCTIONS_DATA: &[FnCompletionItem] = &[
    // Scalar functions
    (
        "CAST",
        "Upper case CAST functions",
        "CAST function",
        r#"CAST($0 AS )"#,
    ),
    (
        "cast",
        "Lower case CAST functions",
        "CAST function",
        r#"cast($0 AS )"#,
    ),
    (
        "TOKEN",
        "Upper case TOKEN functions",
        "TOKEN function",
        r#"TOKEN($0)"#,
    ),
    (
        "token",
        "Lower case TOKEN functions",
        "TOKEN function",
        r#"token($0)"#,
    ),
    (
        "TTL",
        "Upper case TTL functions",
        "TTL function",
        r#"TTL($0)"#,
    ),
    (
        "ttl",
        "Lower case TTL functions",
        "TTL function",
        r#"ttl($0)"#,
    ),
    (
        "UUID",
        "Upper case UUID functions",
        "UUID function",
        r#"UUID() $0"#,
    ),
    (
        "uuid",
        "Lower case UUID functions",
        "UUID function",
        r#"uuid() $0"#,
    ),
    (
        "WRITETIME",
        "Upper case WRITETIME functions",
        "WRITETIME function",
        r#"WRITETIME($0)"#,
    ),
    (
        "writetime",
        "Lower case WRITETIME functions",
        "WRITETIME function",
        r#"writetime($0)"#,
    ),
    // Date/time functions
    (
        "CURRENT_DATE",
        "Upper case CURRENT_DATE functions",
        "CURRENT_DATE function",
        r#"CURRENT_DATE() $0"#,
    ),
    (
        "current_date",
        "Lower case CURRENT_DATE functions",
        "CURRENT_DATE function",
        r#"current_date() $0"#,
    ),
    (
        "CURRENT_TIME",
        "Upper case CURRENT_TIME functions",
        "CURRENT_TIME function",
        r#"CURRENT_TIME() $0"#,
    ),
    (
        "current_time",
        "Lower case CURRENT_TIME functions",
        "CURRENT_TIME function",
        r#"current_time() $0"#,
    ),
    (
        "CURRENT_TIMESTAMP",
        "Upper case CURRENT_TIMESTAMP functions",
        "CURRENT_TIMESTAMP function",
        r#"CURRENT_TIMESTAMP() $0"#,
    ),
    (
        "current_timestamp",
        "Lower case CURRENT_TIMESTAMP functions",
        "CURRENT_TIMESTAMP function",
        r#"current_timestamp() $0"#,
    ),
    (
        "CURRENT_TIMEUUID",
        "Upper case CURRENT_TIMEUUID functions",
        "CURRENT_TIMEUUID function",
        r#"CURRENT_TIMEUUID() $0"#,
    ),
    (
        "current_timeuuid",
        "Lower case CURRENT_TIMEUUID functions",
        "CURRENT_TIMEUUID function",
        r#"current_timeuuid() $0"#,
    ),
    (
        "FLOOR",
        "Upper case FLOOR functions",
        "FLOOR function",
        r#"FLOOR($0)"#,
    ),
    (
        "floor",
        "Lower case FLOOR functions",
        "FLOOR function",
        r#"floor($0)"#,
    ),
    (
        "NOW",
        "Upper case NOW functions",
        "NOW function",
        r#"NOW() $0"#,
    ),
    (
        "now",
        "Lower case NOW functions",
        "NOW function",
        r#"now() $0"#,
    ),
    (
        "MIN_TIMEUUID",
        "Upper case MIN_TIMEUUID functions",
        "MIN_TIMEUUID function",
        r#"MIN_TIMEUUID($0)"#,
    ),
    (
        "min_timeuuid",
        "Lower case MIN_TIMEUUID functions",
        "MIN_TIMEUUID function",
        r#"min_timeuuid($0)"#,
    ),
    (
        "MAX_TIMEUUID",
        "Upper case MAX_TIMEUUID functions",
        "MAX_TIMEUUID function",
        r#"MAX_TIMEUUID($0)"#,
    ),
    (
        "max_timeuuid",
        "Lower case MAX_TIMEUUID functions",
        "MAX_TIMEUUID function",
        r#"max_timeuuid($0)"#,
    ),
    // Date/time conversion
    (
        "TODATE",
        "Upper case TODATE functions",
        "TODATE function",
        r#"TODATE($0)"#,
    ),
    (
        "todate",
        "Lower case TODATE functions",
        "TODATE function",
        r#"todate($0)"#,
    ),
    (
        "TOTIMESTAMP",
        "Upper case TOTIMESTAMP functions",
        "TOTIMESTAMP function",
        r#"TOTIMESTAMP($0)"#,
    ),
    (
        "totimestamp",
        "Lower case TOTIMESTAMP functions",
        "TOTIMESTAMP function",
        r#"totimestamp($0)"#,
    ),
    (
        "TOUNIXTIMESTAMP",
        "Upper case TOUNIXTIMESTAMP functions",
        "TOUNIXTIMESTAMP function",
        r#"TOUNIXTIMESTAMP($0)"#,
    ),
    (
        "tounixtimestamp",
        "Lower case TOUNIXTIMESTAMP functions",
        "TOUNIXTIMESTAMP function",
        r#"tounixtimestamp($0)"#,
    ),
    // Blob conversion
    (
        "blobAs",
        "blobAs functions",
        "blobAs function",
        r#"blobAs<$0>()"#,
    ),
    (
        "AsBlob",
        "AsBlob functions",
        "AsBlob function",
        r#"<$0>AsBlob()"#,
    ),
];

pub static CQL_NATIVE_FUNCTIONS: Lazy<Vec<CompletionItem>> = Lazy::new(|| {
    CQL_NATIVE_FUNCTIONS_DATA
        .iter()
        .map(|&(label, detail, doc, insert)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(detail.to_string()),
            documentation: Some(Documentation::String(doc.to_string())),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
});

const KEYWORDS_DATA: &[FnCompletionItem] = &[
    // USE
    (
        "USE",
        "Upper case USE keyword",
        "USE keyword",
        r#"USE "$0";"#,
    ),
    (
        "use",
        "Lower case use keyword",
        "USE keyword",
        r#"use "$0";"#,
    ),
    // ALTER
    (
        "ALTER",
        "Upper case ALTER keyword",
        "ALTER keyword",
        r#"ALTER $0"#,
    ),
    (
        "alter",
        "Lower case alter keyword",
        "ALTER keyword",
        r#"alter $0"#,
    ),
    // CREATE
    (
        "CREATE",
        "Upper case CREATE keyword",
        "CREATE keyword",
        r#"CREATE $0"#,
    ),
    (
        "create",
        "Lower case create keyword",
        "CREATE keyword",
        r#"create $0"#,
    ),
    // KEYSPACE
    (
        "KEYSPACE",
        "Upper case KEYSPACE keyword",
        "KEYSPACE keyword",
        r#"KEYSPACE $0"#,
    ),
    (
        "keyspace",
        "Lower case keyspace keyword",
        "KEYSPACE keyword",
        r#"keyspace $0"#,
    ),
    // TABLE
    (
        "TABLE",
        "Upper case TABLE keyword",
        "TABLE keyword",
        r#"TABLE $0"#,
    ),
    (
        "table",
        "Lower case table keyword",
        "TABLE keyword",
        r#"table $0"#,
    ),
    // WITH
    (
        "WITH",
        "Upper case WITH keyword",
        "WITH keyword",
        r#"WITH $0"#,
    ),
    (
        "with",
        "Lower case with keyword",
        "WITH keyword",
        r#"with $0"#,
    ),
    // WHERE
    (
        "WHERE",
        "Upper case WHERE keyword",
        "WHERE keyword",
        r#"WHERE $0"#,
    ),
    (
        "where",
        "Lower case where keyword",
        "WHERE keyword",
        r#"where $0"#,
    ),
    // IF
    ("IF", "Upper case IF keyword", "IF keyword", r#"IF $0"#),
    ("if", "Lower case if keyword", "IF keyword", r#"if $0"#),
    // AND
    ("AND", "Upper case AND keyword", "AND keyword", r#"AND $0"#),
    ("and", "Lower case and keyword", "AND keyword", r#"and $0"#),
    // SET
    ("SET", "Upper case SET keyword", "SET keyword", r#"SET $0"#),
    ("set", "Lower case set keyword", "SET keyword", r#"set $0"#),
    // IN
    ("IN", "Upper case IN keyword", "IN keyword", r#"IN $0"#),
    ("in", "Lower case in keyword", "IN keyword", r#"in $0"#),
    // TO
    ("TO", "Upper case TO keyword", "TO keyword", r#"TO $0"#),
    ("to", "Lower case to keyword", "TO keyword", r#"to $0"#),
    // FROM
    (
        "FROM",
        "Upper case FROM keyword",
        "FROM keyword",
        r#"FROM $0"#,
    ),
    (
        "from",
        "Lower case from keyword",
        "FROM keyword",
        r#"from $0"#,
    ),
    // USING
    (
        "USING",
        "Upper case USING keyword",
        "USING keyword",
        r#"USING $0"#,
    ),
    (
        "using",
        "Lower case using keyword",
        "USING keyword",
        r#"using $0"#,
    ),
    // TIMESTAMP
    (
        "TIMESTAMP",
        "Upper case TIMESTAMP keyword",
        "TIMESTAMP keyword",
        r#"TIMESTAMP $0"#,
    ),
    (
        "timestamp",
        "Lower case timestamp keyword",
        "TIMESTAMP keyword",
        r#"timestamp $0"#,
    ),
    // TTL
    ("TTL", "Upper case TTL keyword", "TTL keyword", r#"TTL $0"#),
    ("ttl", "Lower case ttl keyword", "TTL keyword", r#"ttl $0"#),
    // EXISTS
    (
        "EXISTS",
        "Upper case EXISTS keyword",
        "EXISTS keyword",
        r#"EXISTS $0"#,
    ),
    (
        "exists",
        "Lower case exists keyword",
        "EXISTS keyword",
        r#"exists $0"#,
    ),
    // NOT
    ("NOT", "Upper case NOT keyword", "NOT keyword", r#"NOT $0"#),
    ("not", "Lower case not keyword", "NOT keyword", r#"not $0"#),
    // TYPE
    (
        "TYPE",
        "Upper case TYPE keyword",
        "TYPE keyword",
        r#"TYPE $0"#,
    ),
    (
        "type",
        "Lower case type keyword",
        "TYPE keyword",
        r#"type $0"#,
    ),
    // VIEW
    (
        "VIEW",
        "Upper case VIEW keyword",
        "VIEW keyword",
        r#"VIEW $0"#,
    ),
    (
        "view",
        "Lower case view keyword",
        "VIEW keyword",
        r#"view $0"#,
    ),
    // MATERIALIZED
    (
        "MATERIALIZED",
        "Upper case MATERIALIZED keyword",
        "MATERIALIZED keyword",
        r#"MATERIALIZED $0"#,
    ),
    (
        "materialized",
        "Lower case materialized keyword",
        "MATERIALIZED keyword",
        r#"materialized $0"#,
    ),
    // REPLICATION
    (
        "REPLICATION",
        "Upper case REPLICATION keyword",
        "REPLICATION keyword",
        r#"REPLICATION $0"#,
    ),
    (
        "replication",
        "Lower case replication keyword",
        "REPLICATION keyword",
        r#"replication $0"#,
    ),
    // DURABLE_WRITES
    (
        "DURABLE_WRITES",
        "Upper case DURABLE_WRITES keyword",
        "DURABLE_WRITES keyword",
        r#"DURABLE_WRITES $0"#,
    ),
    (
        "durable_writes",
        "Lower case durable_writes keyword",
        "DURABLE_WRITES keyword",
        r#"durable_writes $0"#,
    ),
    // BATCH
    (
        "BATCH",
        "Upper case BATCH keyword",
        "BATCH keyword",
        r#"BATCH $0"#,
    ),
    (
        "batch",
        "Lower case batch keyword",
        "BATCH keyword",
        r#"batch $0"#,
    ),
    // APPLY
    (
        "APPLY",
        "Upper case APPLY keyword",
        "APPLY keyword",
        r#"APPLY $0"#,
    ),
    (
        "apply",
        "Lower case apply keyword",
        "APPLY keyword",
        r#"apply $0"#,
    ),
    // BEGIN
    (
        "BEGIN",
        "Upper case BEGIN keyword",
        "BEGIN keyword",
        r#"BEGIN $0"#,
    ),
    (
        "begin",
        "Lower case begin keyword",
        "BEGIN keyword",
        r#"begin $0"#,
    ),
    // UNLOGGED
    (
        "UNLOGGED",
        "Upper case UNLOGGED keyword",
        "UNLOGGED keyword",
        r#"UNLOGGED $0"#,
    ),
    (
        "unlogged",
        "Lower case unlogged keyword",
        "UNLOGGED keyword",
        r#"unlogged $0"#,
    ),
    // LOGGED
    (
        "LOGGED",
        "Upper case LOGGED keyword",
        "LOGGED keyword",
        r#"LOGGED $0"#,
    ),
    (
        "logged",
        "Lower case logged keyword",
        "LOGGED keyword",
        r#"logged $0"#,
    ),
    // COUNTER
    (
        "COUNTER",
        "Upper case COUNTER keyword",
        "COUNTER keyword",
        r#"COUNTER $0"#,
    ),
    (
        "counter",
        "Lower case counter keyword",
        "COUNTER keyword",
        r#"counter $0"#,
    ),
    // TRUNCATE
    (
        "TRUNCATE",
        "Upper case TRUNCATE keyword",
        "TRUNCATE keyword",
        r#"TRUNCATE $0"#,
    ),
    (
        "truncate",
        "Lower case truncate keyword",
        "TRUNCATE keyword",
        r#"truncate $0"#,
    ),
    // INSERT
    (
        "INSERT",
        "Upper case INSERT keyword",
        "INSERT keyword",
        r#"INSERT INTO $0"#,
    ),
    (
        "insert",
        "Lower case insert keyword",
        "INSERT keyword",
        r#"insert into $0"#,
    ),
    // INTO
    (
        "INTO",
        "Upper case INTO keyword",
        "INTO keyword",
        r#"INTO $0"#,
    ),
    (
        "into",
        "Lower case into keyword",
        "INTO keyword",
        r#"into $0"#,
    ),
    // VALUES
    (
        "VALUES",
        "Upper case VALUES keyword",
        "VALUES keyword",
        r#"VALUES ($0)"#,
    ),
    (
        "values",
        "Lower case values keyword",
        "VALUES keyword",
        r#"values ($0)"#,
    ),
    // UPDATE
    (
        "UPDATE",
        "Upper case UPDATE keyword",
        "UPDATE keyword",
        r#"UPDATE $0"#,
    ),
    (
        "update",
        "Lower case update keyword",
        "UPDATE keyword",
        r#"update $0"#,
    ),
    // DELETE
    (
        "DELETE",
        "Upper case DELETE keyword",
        "DELETE keyword",
        r#"DELETE $0"#,
    ),
    (
        "delete",
        "Lower case delete keyword",
        "DELETE keyword",
        r#"delete $0"#,
    ),
    // ROLE
    (
        "ROLE",
        "Upper case ROLE keyword",
        "ROLE keyword",
        r#"ROLE $0"#,
    ),
    (
        "role",
        "Lower case role keyword",
        "ROLE keyword",
        r#"role $0"#,
    ),
    // PASSWORD
    (
        "PASSWORD",
        "Upper case PASSWORD keyword",
        "PASSWORD keyword",
        r#"PASSWORD $0"#,
    ),
    (
        "password",
        "Lower case password keyword",
        "PASSWORD keyword",
        r#"password $0"#,
    ),
    // USER
    (
        "USER",
        "Upper case USER keyword",
        "USER keyword",
        r#"USER $0"#,
    ),
    (
        "user",
        "Lower case user keyword",
        "USER keyword",
        r#"user $0"#,
    ),
    // SUPERUSER
    (
        "SUPERUSER",
        "Upper case SUPERUSER keyword",
        "SUPERUSER keyword",
        r#"SUPERUSER $0"#,
    ),
    (
        "superuser",
        "Lower case superuser keyword",
        "SUPERUSER keyword",
        r#"superuser $0"#,
    ),
    // NOSUPERUSER
    (
        "NOSUPERUSER",
        "Upper case NOSUPERUSER keyword",
        "NOSUPERUSER keyword",
        r#"NOSUPERUSER $0"#,
    ),
    (
        "nosuperuser",
        "Lower case nosuperuser keyword",
        "NOSUPERUSER keyword",
        r#"nosuperuser $0"#,
    ),
    // ADD
    ("ADD", "Upper case ADD keyword", "ADD keyword", r#"ADD $0"#),
    ("add", "Lower case add keyword", "ADD keyword", r#"add $0"#),
    // DROP
    (
        "DROP",
        "Upper case DROP keyword",
        "DROP keyword",
        r#"DROP $0"#,
    ),
    (
        "drop",
        "Lower case drop keyword",
        "DROP keyword",
        r#"drop $0"#,
    ),
    // RENAME
    (
        "RENAME",
        "Upper case RENAME keyword",
        "RENAME keyword",
        r#"RENAME $0"#,
    ),
    (
        "rename",
        "Lower case rename keyword",
        "RENAME keyword",
        r#"rename $0"#,
    ),
    // COMPACT
    (
        "COMPACT",
        "Upper case COMPACT keyword",
        "COMPACT keyword",
        r#"COMPACT $0"#,
    ),
    (
        "compact",
        "Lower case compact keyword",
        "COMPACT keyword",
        r#"compact $0"#,
    ),
    // STORAGE
    (
        "STORAGE",
        "Upper case STORAGE keyword",
        "STORAGE keyword",
        r#"STORAGE $0"#,
    ),
    (
        "storage",
        "Lower case storage keyword",
        "STORAGE keyword",
        r#"storage $0"#,
    ),
    // CONTAINS
    (
        "CONTAINS",
        "Upper case CONTAINS keyword",
        "CONTAINS keyword",
        r#"CONTAINS $0"#,
    ),
    (
        "contains",
        "Lower case contains keyword",
        "CONTAINS keyword",
        r#"contains $0"#,
    ),
    // KEY
    ("KEY", "Upper case KEY keyword", "KEY keyword", r#"KEY $0"#),
    ("key", "Lower case key keyword", "KEY keyword", r#"key $0"#),
    // LOGIN
    (
        "LOGIN",
        "Upper case LOGIN keyword",
        "LOGIN keyword",
        r#"LOGIN $0"#,
    ),
    (
        "login",
        "Lower case login keyword",
        "LOGIN keyword",
        r#"login $0"#,
    ),
    // OPTIONS
    (
        "OPTIONS",
        "Upper case OPTIONS keyword",
        "OPTIONS keyword",
        r#"OPTIONS $0"#,
    ),
    (
        "options",
        "Lower case options keyword",
        "OPTIONS keyword",
        r#"options $0"#,
    ),
    // OR
    ("OR", "Upper case OR keyword", "OR keyword", r#"OR $0"#),
    ("or", "Lower case or keyword", "OR keyword", r#"or $0"#),
    // REPLACE
    (
        "REPLACE",
        "Upper case REPLACE keyword",
        "REPLACE keyword",
        r#"REPLACE $0"#,
    ),
    (
        "replace",
        "Lower case replace keyword",
        "REPLACE keyword",
        r#"replace $0"#,
    ),
    // SFUNC
    (
        "SFUNC",
        "Upper case SFUNC keyword",
        "SFUNC keyword",
        r#"SFUNC $0"#,
    ),
    (
        "sfunc",
        "Lower case sfunc keyword",
        "SFUNC keyword",
        r#"sfunc $0"#,
    ),
    // STYPE
    (
        "STYPE",
        "Upper case STYPE keyword",
        "STYPE keyword",
        r#"STYPE $0"#,
    ),
    (
        "stype",
        "Lower case stype keyword",
        "STYPE keyword",
        r#"stype $0"#,
    ),
    // FINALFUNC
    (
        "FINALFUNC",
        "Upper case FINALFUNC keyword",
        "FINALFUNC keyword",
        r#"FINALFUNC $0"#,
    ),
    (
        "finalfunc",
        "Lower case finalfunc keyword",
        "FINALFUNC keyword",
        r#"finalfunc $0"#,
    ),
    // INITCOND
    (
        "INITCOND",
        "Upper case INITCOND keyword",
        "INITCOND keyword",
        r#"INITCOND $0"#,
    ),
    (
        "initcond",
        "Lower case initcond keyword",
        "INITCOND keyword",
        r#"initcond $0"#,
    ),
    // LANGUAGE
    (
        "LANGUAGE",
        "Upper case LANGUAGE keyword",
        "LANGUAGE keyword",
        r#"LANGUAGE $0"#,
    ),
    (
        "language",
        "Lower case language keyword",
        "LANGUAGE keyword",
        r#"language $0"#,
    ),
    // INPUT
    (
        "INPUT",
        "Upper case INPUT keyword",
        "INPUT keyword",
        r#"INPUT $0"#,
    ),
    (
        "input",
        "Lower case input keyword",
        "INPUT keyword",
        r#"input $0"#,
    ),
    // ON
    ("ON", "Upper case ON keyword", "ON keyword", r#"ON $0"#),
    ("on", "Lower case on keyword", "ON keyword", r#"on $0"#),
    // FUNCTION
    (
        "FUNCTION",
        "Upper case FUNCTION keyword",
        "FUNCTION keyword",
        r#"FUNCTION $0"#,
    ),
    (
        "function",
        "Lower case function keyword",
        "FUNCTION keyword",
        r#"function $0"#,
    ),
    // CALLED
    (
        "CALLED",
        "Upper case CALLED keyword",
        "CALLED keyword",
        r#"CALLED $0"#,
    ),
    (
        "called",
        "Lower case called keyword",
        "CALLED keyword",
        r#"called $0"#,
    ),
    // RETURNS
    (
        "RETURNS",
        "Upper case RETURNS keyword",
        "RETURNS keyword",
        r#"RETURNS $0"#,
    ),
    (
        "returns",
        "Lower case returns keyword",
        "RETURNS keyword",
        r#"returns $0"#,
    ),
    // FILTERING
    (
        "FILTERING",
        "Upper case FILTERING keyword",
        "FILTERING keyword",
        r#"FILTERING $0"#,
    ),
    (
        "filtering",
        "Lower case filtering keyword",
        "FILTERING keyword",
        r#"filtering $0"#,
    ),
    // DISTINCT
    (
        "DISTINCT",
        "Upper case DISTINCT keyword",
        "DISTINCT keyword",
        r#"DISTINCT $0"#,
    ),
    (
        "distinct",
        "Lower case distinct keyword",
        "DISTINCT keyword",
        r#"distinct $0"#,
    ),
    // AS
    ("AS", "Upper case AS keyword", "AS keyword", r#"AS $0"#),
    ("as", "Lower case as keyword", "AS keyword", r#"as $0"#),
    // KEYS
    (
        "KEYS",
        "Upper case KEYS keyword",
        "KEYS keyword",
        r#"KEYS $0"#,
    ),
    (
        "keys",
        "Lower case keys keyword",
        "KEYS keyword",
        r#"keys $0"#,
    ),
    // GROUP
    (
        "GROUP",
        "Upper case GROUP keyword",
        "GROUP keyword",
        r#"GROUP $0"#,
    ),
    (
        "group",
        "Lower case group keyword",
        "GROUP keyword",
        r#"group $0"#,
    ),
    // BY
    ("BY", "Upper case BY keyword", "BY keyword", r#"BY $0"#),
    ("by", "Lower case by keyword", "BY keyword", r#"by $0"#),
    // JSON
    (
        "JSON",
        "Upper case JSON keyword",
        "JSON keyword",
        r#"JSON $0"#,
    ),
    (
        "json",
        "Lower case json keyword",
        "JSON keyword",
        r#"json $0"#,
    ),
    // NULL
    (
        "NULL",
        "Upper case NULL keyword",
        "NULL keyword",
        r#"NULL $0"#,
    ),
    (
        "null",
        "Lower case null keyword",
        "NULL keyword",
        r#"null $0"#,
    ),
    // CUSTOM
    (
        "CUSTOM",
        "Upper case CUSTOM keyword",
        "CUSTOM keyword",
        r#"CUSTOM $0"#,
    ),
    (
        "custom",
        "Lower case custom keyword",
        "CUSTOM keyword",
        r#"custom $0"#,
    ),
    // AGGREGATE
    (
        "AGGREGATE",
        "Upper case AGGREGATE keyword",
        "AGGREGATE keyword",
        r#"AGGREGATE $0"#,
    ),
    (
        "aggregate",
        "Lower case aggregate keyword",
        "AGGREGATE keyword",
        r#"aggregate $0"#,
    ),
    // ALL
    ("ALL", "Upper case ALL keyword", "ALL keyword", r#"ALL $0"#),
    ("all", "Lower case all keyword", "ALL keyword", r#"all $0"#),
    // ALLOW
    (
        "ALLOW",
        "Upper case ALLOW keyword",
        "ALLOW keyword",
        r#"ALLOW $0"#,
    ),
    (
        "allow",
        "Lower case allow keyword",
        "ALLOW keyword",
        r#"allow $0"#,
    ),
    // ASC
    ("ASC", "Upper case ASC keyword", "ASC keyword", r#"ASC $0"#),
    ("asc", "Lower case asc keyword", "ASC keyword", r#"asc $0"#),
    // AUTHORIZE
    (
        "AUTHORIZE",
        "Upper case AUTHORIZE keyword",
        "AUTHORIZE keyword",
        r#"AUTHORIZE $0"#,
    ),
    (
        "authorize",
        "Lower case authorize keyword",
        "AUTHORIZE keyword",
        r#"authorize $0"#,
    ),
    // CLUSTERING
    (
        "CLUSTERING",
        "Upper case CLUSTERING keyword",
        "CLUSTERING keyword",
        r#"CLUSTERING $0"#,
    ),
    (
        "clustering",
        "Lower case clustering keyword",
        "CLUSTERING keyword",
        r#"clustering $0"#,
    ),
    // DESC
    (
        "DESC",
        "Upper case DESC keyword",
        "DESC keyword",
        r#"DESC $0"#,
    ),
    (
        "desc",
        "Lower case desc keyword",
        "DESC keyword",
        r#"desc $0"#,
    ),
    // DESCRIBE
    (
        "DESCRIBE",
        "Upper case DESCRIBE keyword",
        "DESCRIBE keyword",
        r#"DESCRIBE $0"#,
    ),
    (
        "describe",
        "Lower case describe keyword",
        "DESCRIBE keyword",
        r#"describe $0"#,
    ),
    // ENTRIES
    (
        "ENTRIES",
        "Upper case ENTRIES keyword",
        "ENTRIES keyword",
        r#"ENTRIES $0"#,
    ),
    (
        "entries",
        "Lower case entries keyword",
        "ENTRIES keyword",
        r#"entries $0"#,
    ),
    // FULL
    (
        "FULL",
        "Upper case FULL keyword",
        "FULL keyword",
        r#"FULL $0"#,
    ),
    (
        "full",
        "Lower case full keyword",
        "FULL keyword",
        r#"full $0"#,
    ),
    // GRANT
    (
        "GRANT",
        "Upper case GRANT keyword",
        "GRANT keyword",
        r#"GRANT $0"#,
    ),
    (
        "grant",
        "Lower case grant keyword",
        "GRANT keyword",
        r#"grant $0"#,
    ),
    // INDEX
    (
        "INDEX",
        "Upper case INDEX keyword",
        "INDEX keyword",
        r#"INDEX $0"#,
    ),
    (
        "index",
        "Lower case index keyword",
        "INDEX keyword",
        r#"index $0"#,
    ),
    // KEYSPACES
    (
        "KEYSPACES",
        "Upper case KEYSPACES keyword",
        "KEYSPACES keyword",
        r#"KEYSPACES $0"#,
    ),
    (
        "keyspaces",
        "Lower case keyspaces keyword",
        "KEYSPACES keyword",
        r#"keyspaces $0"#,
    ),
    // LIMIT
    (
        "LIMIT",
        "Upper case LIMIT keyword",
        "LIMIT keyword",
        r#"LIMIT $0"#,
    ),
    (
        "limit",
        "Lower case limit keyword",
        "LIMIT keyword",
        r#"limit $0"#,
    ),
    // MODIFY
    (
        "MODIFY",
        "Upper case MODIFY keyword",
        "MODIFY keyword",
        r#"MODIFY $0"#,
    ),
    (
        "modify",
        "Lower case modify keyword",
        "MODIFY keyword",
        r#"modify $0"#,
    ),
    // NORECURSIVE
    (
        "NORECURSIVE",
        "Upper case NORECURSIVE keyword",
        "NORECURSIVE keyword",
        r#"NORECURSIVE $0"#,
    ),
    (
        "norecursive",
        "Lower case norecursive keyword",
        "NORECURSIVE keyword",
        r#"norecursive $0"#,
    ),
    // OF
    ("OF", "Upper case OF keyword", "OF keyword", r#"OF $0"#),
    ("of", "Lower case of keyword", "OF keyword", r#"of $0"#),
    // ORDER
    (
        "ORDER",
        "Upper case ORDER keyword",
        "ORDER keyword",
        r#"ORDER $0"#,
    ),
    (
        "order",
        "Lower case order keyword",
        "ORDER keyword",
        r#"order $0"#,
    ),
    // PARTITION
    (
        "PARTITION",
        "Upper case PARTITION keyword",
        "PARTITION keyword",
        r#"PARTITION $0"#,
    ),
    (
        "partition",
        "Lower case partition keyword",
        "PARTITION keyword",
        r#"partition $0"#,
    ),
    // PER
    ("PER", "Upper case PER keyword", "PER keyword", r#"PER $0"#),
    ("per", "Lower case per keyword", "PER keyword", r#"per $0"#),
    // PERMISSIONS
    (
        "PERMISSIONS",
        "Upper case PERMISSIONS keyword",
        "PERMISSIONS keyword",
        r#"PERMISSIONS $0"#,
    ),
    (
        "permissions",
        "Lower case permissions keyword",
        "PERMISSIONS keyword",
        r#"permissions $0"#,
    ),
    // PRIMARY
    (
        "PRIMARY",
        "Upper case PRIMARY keyword",
        "PRIMARY keyword",
        r#"PRIMARY $0"#,
    ),
    (
        "primary",
        "Lower case primary keyword",
        "PRIMARY keyword",
        r#"primary $0"#,
    ),
    // REVOKE
    (
        "REVOKE",
        "Upper case REVOKE keyword",
        "REVOKE keyword",
        r#"REVOKE $0"#,
    ),
    (
        "revoke",
        "Lower case revoke keyword",
        "REVOKE keyword",
        r#"revoke $0"#,
    ),
    // SELECT
    (
        "SELECT",
        "Upper case SELECT keyword",
        "SELECT keyword",
        r#"SELECT $0"#,
    ),
    (
        "select",
        "Lower case select keyword",
        "SELECT keyword",
        r#"select $0"#,
    ),
    // USERS
    (
        "USERS",
        "Upper case USERS keyword",
        "USERS keyword",
        r#"USERS $0"#,
    ),
    (
        "users",
        "Lower case users keyword",
        "USERS keyword",
        r#"users $0"#,
    ),
    // COMMIT
    (
        "COMMIT",
        "Upper case COMMIT keyword",
        "COMMIT keyword",
        r#"COMMIT $0"#,
    ),
    (
        "commit",
        "Lower case commit keyword",
        "COMMIT keyword",
        r#"commit $0"#,
    ),
    // SEARCH
    (
        "SEARCH",
        "Upper case SEARCH keyword",
        "SEARCH keyword",
        r#"SEARCH $0"#,
    ),
    (
        "search",
        "Lower case search keyword",
        "SEARCH keyword",
        r#"search $0"#,
    ),
    // ROLES
    (
        "ROLES",
        "Upper case ROLES keyword",
        "ROLES keyword",
        r#"ROLES $0"#,
    ),
    (
        "roles",
        "Lower case roles keyword",
        "ROLES keyword",
        r#"roles $0"#,
    ),
    // DETERMINISTIC
    (
        "DETERMINISTIC",
        "Upper case DETERMINISTIC keyword",
        "DETERMINISTIC keyword",
        r#"DETERMINISTIC $0"#,
    ),
    (
        "deterministic",
        "Lower case deterministic keyword",
        "DETERMINISTIC keyword",
        r#"deterministic $0"#,
    ),
    // MONOTONIC
    (
        "MONOTONIC",
        "Upper case MONOTONIC keyword",
        "MONOTONIC keyword",
        r#"MONOTONIC $0"#,
    ),
    (
        "monotonic",
        "Lower case monotonic keyword",
        "MONOTONIC keyword",
        r#"monotonic $0"#,
    ),
    // JAVA
    (
        "JAVA",
        "Upper case JAVA keyword",
        "JAVA keyword",
        r#"JAVA $0"#,
    ),
    (
        "java",
        "Lower case java keyword",
        "JAVA keyword",
        r#"java $0"#,
    ),
    // JAVASCRIPT
    (
        "JAVASCRIPT",
        "Upper case JAVASCRIPT keyword",
        "JAVASCRIPT keyword",
        r#"JAVASCRIPT $0"#,
    ),
    (
        "javascript",
        "Lower case javascript keyword",
        "JAVASCRIPT keyword",
        r#"javascript $0"#,
    ),
    // IS
    ("IS", "Upper case IS keyword", "IS keyword", r#"IS $0"#),
    ("is", "Lower case is keyword", "IS keyword", r#"is $0"#),
    // HASHED
    (
        "HASHED",
        "Upper case HASHED keyword",
        "HASHED keyword",
        r#"HASHED $0"#,
    ),
    (
        "hashed",
        "Lower case hashed keyword",
        "HASHED keyword",
        r#"hashed $0"#,
    ),
    // ACCESS
    (
        "ACCESS",
        "Upper case ACCESS keyword",
        "ACCESS keyword",
        r#"ACCESS $0"#,
    ),
    (
        "access",
        "Lower case access keyword",
        "ACCESS keyword",
        r#"access $0"#,
    ),
    // DATACENTERS
    (
        "DATACENTERS",
        "Upper case DATACENTERS keyword",
        "DATACENTERS keyword",
        r#"DATACENTERS $0"#,
    ),
    (
        "datacenters",
        "Lower case datacenters keyword",
        "DATACENTERS keyword",
        r#"datacenters $0"#,
    ),
    // CIDRS
    (
        "CIDRS",
        "Upper case CIDRS keyword",
        "CIDRS keyword",
        r#"CIDRS $0"#,
    ),
    (
        "cidrs",
        "Lower case cidrs keyword",
        "CIDRS keyword",
        r#"cidrs $0"#,
    ),
    // COLUMNS
    (
        "COLUMNS",
        "Upper case COLUMNS keyword",
        "COLUMNS keyword",
        r#"COLUMNS $0"#,
    ),
    (
        "columns",
        "Lower case columns keyword",
        "COLUMNS keyword",
        r#"columns $0"#,
    ),
    // PROFILES
    (
        "PROFILES",
        "Upper case PROFILES keyword",
        "PROFILES keyword",
        r#"PROFILES $0"#,
    ),
    (
        "profiles",
        "Lower case profiles keyword",
        "PROFILES keyword",
        r#"profiles $0"#,
    ),
    // CONFIG
    (
        "CONFIG",
        "Upper case CONFIG keyword",
        "CONFIG keyword",
        r#"CONFIG $0"#,
    ),
    (
        "config",
        "Lower case config keyword",
        "CONFIG keyword",
        r#"config $0"#,
    ),
    // ROWS
    (
        "ROWS",
        "Upper case ROWS keyword",
        "ROWS keyword",
        r#"ROWS $0"#,
    ),
    (
        "rows",
        "Lower case rows keyword",
        "ROWS keyword",
        r#"rows $0"#,
    ),
    // FUNCTIONS
    (
        "FUNCTIONS",
        "Upper case FUNCTIONS keyword",
        "FUNCTIONS keyword",
        r#"FUNCTIONS $0"#,
    ),
    (
        "functions",
        "Lower case functions keyword",
        "FUNCTIONS keyword",
        r#"functions $0"#,
    ),
    // MBEANS
    (
        "MBEANS",
        "Upper case MBEANS keyword",
        "MBEANS keyword",
        r#"MBEANS $0"#,
    ),
    (
        "mbeans",
        "Lower case mbeans keyword",
        "MBEANS keyword",
        r#"mbeans $0"#,
    ),
    // MBEAN
    (
        "MBEAN",
        "Upper case MBEAN keyword",
        "MBEAN keyword",
        r#"MBEAN $0"#,
    ),
    (
        "mbean",
        "Lower case mbean keyword",
        "MBEAN keyword",
        r#"mbean $0"#,
    ),
    // PATTERN
    (
        "PATTERN",
        "Upper case PATTERN keyword",
        "PATTERN keyword",
        r#"PATTERN $0"#,
    ),
    (
        "pattern",
        "Lower case pattern keyword",
        "PATTERN keyword",
        r#"pattern $0"#,
    ),
    // EXECUTE
    (
        "EXECUTE",
        "Upper case EXECUTE keyword",
        "EXECUTE keyword",
        r#"EXECUTE $0"#,
    ),
    (
        "execute",
        "Lower case execute keyword",
        "EXECUTE keyword",
        r#"execute $0"#,
    ),
    // PROXY
    (
        "PROXY",
        "Upper case PROXY keyword",
        "PROXY keyword",
        r#"PROXY $0"#,
    ),
    (
        "proxy",
        "Lower case proxy keyword",
        "PROXY keyword",
        r#"proxy $0"#,
    ),
    // ID
    ("ID", "Upper case ID keyword", "ID keyword", r#"ID $0"#),
    ("id", "Lower case id keyword", "ID keyword", r#"id $0"#),
    // LIKE
    (
        "LIKE",
        "Upper case LIKE keyword",
        "LIKE keyword",
        r#"LIKE $0"#,
    ),
    (
        "like",
        "Lower case like keyword",
        "LIKE keyword",
        r#"like $0"#,
    ),
    // ANN
    ("ANN", "Upper case ANN keyword", "ANN keyword", r#"ANN $0"#),
    ("ann", "Lower case ann keyword", "ANN keyword", r#"ann $0"#),
    // OFFSET
    (
        "OFFSET",
        "Upper case OFFSET keyword",
        "OFFSET keyword",
        r#"OFFSET $0"#,
    ),
    (
        "offset",
        "Lower case offset keyword",
        "OFFSET keyword",
        r#"offset $0"#,
    ),
    // LIST
    (
        "LIST",
        "Upper case LIST keyword",
        "LIST keyword",
        r#"LIST $0"#,
    ),
    (
        "list",
        "Lower case list keyword",
        "LIST keyword",
        r#"list $0"#,
    ),
    // MAX
    ("MAX", "Upper case MAX keyword", "MAX keyword", r#"MAX($0)"#),
    ("max", "Lower case max keyword", "MAX keyword", r#"max($0)"#),
    // MIN
    ("MIN", "Upper case MIN keyword", "MIN keyword", r#"MIN($0)"#),
    ("min", "Lower case min keyword", "MIN keyword", r#"min($0)"#),
    // SUM
    ("SUM", "Upper case SUM keyword", "SUM keyword", r#"SUM($0)"#),
    ("sum", "Lower case sum keyword", "SUM keyword", r#"sum($0)"#),
    // AVG
    ("AVG", "Upper case AVG keyword", "AVG keyword", r#"AVG($0)"#),
    ("avg", "Lower case avg keyword", "AVG keyword", r#"avg($0)"#),
    // TOKEN
    (
        "TOKEN",
        "Upper case TOKEN keyword",
        "TOKEN keyword",
        r#"TOKEN($0)"#,
    ),
    (
        "token",
        "Lower case token keyword",
        "TOKEN keyword",
        r#"token($0)"#,
    ),
    // WRITETIME
    (
        "WRITETIME",
        "Upper case WRITETIME keyword",
        "WRITETIME keyword",
        r#"WRITETIME($0)"#,
    ),
    (
        "writetime",
        "Lower case writetime keyword",
        "WRITETIME keyword",
        r#"writetime($0)"#,
    ),
    // COUNT
    (
        "COUNT",
        "Upper case COUNT keyword",
        "COUNT keyword",
        r#"COUNT($0)"#,
    ),
    (
        "count",
        "Lower case count keyword",
        "COUNT keyword",
        r#"count($0)"#,
    ),
    // INFINITY
    (
        "INFINITY",
        "Upper case INFINITY keyword",
        "INFINITY keyword",
        r#"INFINITY $0"#,
    ),
    (
        "infinity",
        "Lower case infinity keyword",
        "INFINITY keyword",
        r#"infinity $0"#,
    ),
    // NAN
    ("NAN", "Upper case NAN keyword", "NAN keyword", r#"NAN $0"#),
    ("nan", "Lower case nan keyword", "NAN keyword", r#"nan $0"#),
    // STATIC
    (
        "STATIC",
        "Upper case STATIC keyword",
        "STATIC keyword",
        r#"STATIC $0"#,
    ),
    (
        "static",
        "Lower case static keyword",
        "STATIC keyword",
        r#"static $0"#,
    ),
    // ANY
    ("ANY", "Upper case ANY keyword", "ANY keyword", r#"ANY $0"#),
    ("any", "Lower case any keyword", "ANY keyword", r#"any $0"#),
    // HAVING
    (
        "HAVING",
        "Upper case HAVING keyword",
        "HAVING keyword",
        r#"HAVING $0"#,
    ),
    (
        "having",
        "Lower case having keyword",
        "HAVING keyword",
        r#"having $0"#,
    ),
    // CONSISTENCY
    (
        "CONSISTENCY",
        "Upper case CONSISTENCY keyword",
        "CONSISTENCY keyword",
        r#"CONSISTENCY $0"#,
    ),
    (
        "consistency",
        "Lower case consistency keyword",
        "CONSISTENCY keyword",
        r#"consistency $0"#,
    ),
    // LEVEL
    (
        "LEVEL",
        "Upper case LEVEL keyword",
        "LEVEL keyword",
        r#"LEVEL $0"#,
    ),
    (
        "level",
        "Lower case level keyword",
        "LEVEL keyword",
        r#"level $0"#,
    ),
    // ONE
    ("ONE", "Upper case ONE keyword", "ONE keyword", r#"ONE $0"#),
    ("one", "Lower case one keyword", "ONE keyword", r#"one $0"#),
    // TWO
    ("TWO", "Upper case TWO keyword", "TWO keyword", r#"TWO $0"#),
    ("two", "Lower case two keyword", "TWO keyword", r#"two $0"#),
    // THREE
    (
        "THREE",
        "Upper case THREE keyword",
        "THREE keyword",
        r#"THREE $0"#,
    ),
    (
        "three",
        "Lower case three keyword",
        "THREE keyword",
        r#"three $0"#,
    ),
    // QUORUM
    (
        "QUORUM",
        "Upper case QUORUM keyword",
        "QUORUM keyword",
        r#"QUORUM $0"#,
    ),
    (
        "quorum",
        "Lower case quorum keyword",
        "QUORUM keyword",
        r#"quorum $0"#,
    ),
    // LOCAL_ONE
    (
        "LOCAL_ONE",
        "Upper case LOCAL_ONE keyword",
        "LOCAL_ONE keyword",
        r#"LOCAL_ONE $0"#,
    ),
    (
        "local_one",
        "Lower case local_one keyword",
        "LOCAL_ONE keyword",
        r#"local_one $0"#,
    ),
    // LOCAL_QUORUM
    (
        "LOCAL_QUORUM",
        "Upper case LOCAL_QUORUM keyword",
        "LOCAL_QUORUM keyword",
        r#"LOCAL_QUORUM $0"#,
    ),
    (
        "local_quorum",
        "Lower case local_quorum keyword",
        "LOCAL_QUORUM keyword",
        r#"local_quorum $0"#,
    ),
    // EACH_QUORUM
    (
        "EACH_QUORUM",
        "Upper case EACH_QUORUM keyword",
        "EACH_QUORUM keyword",
        r#"EACH_QUORUM $0"#,
    ),
    (
        "each_quorum",
        "Lower case each_quorum keyword",
        "EACH_QUORUM keyword",
        r#"each_quorum $0"#,
    ),
];

pub static KEYWORDS: Lazy<Vec<CompletionItem>> = Lazy::new(|| {
    KEYWORDS_DATA
        .iter()
        .map(|&(label, detail, doc, insert)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(detail.to_string()),
            documentation: Some(Documentation::String(doc.to_string())),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
});

const TYPES_DATA: &[FnCompletionItem] = &[
    // Parameterized types
    (
        "LIST<>",
        "LIST type with parameter",
        "LIST<type> - Collection type",
        r#"LIST<$0>"#,
    ),
    (
        "list<>",
        "list type with parameter",
        "LIST<type> - Collection type",
        r#"list<$0>"#,
    ),
    (
        "SET<>",
        "SET type with parameter",
        "SET<type> - Collection type",
        r#"SET<$0>"#,
    ),
    (
        "set<>",
        "set type with parameter",
        "SET<type> - Collection type",
        r#"set<$0>"#,
    ),
    (
        "MAP<>",
        "MAP type with parameters",
        "MAP<key, value> - Collection type",
        r#"MAP<$0, $1>"#,
    ),
    (
        "map<>",
        "map type with parameters",
        "MAP<key, value> - Collection type",
        r#"map<$0, $1>"#,
    ),
    (
        "TUPLE<>",
        "TUPLE type with parameters",
        "TUPLE<type1, type2, ...> - Composite type",
        r#"TUPLE<$0>"#,
    ),
    (
        "tuple<>",
        "tuple type with parameters",
        "TUPLE<type1, type2, ...> - Composite type",
        r#"tuple<$0>"#,
    ),
    (
        "FROZEN<>",
        "FROZEN type with parameter",
        "FROZEN<type> - Frozen collection type",
        r#"FROZEN<$0>"#,
    ),
    (
        "frozen<>",
        "frozen type with parameter",
        "FROZEN<type> - Frozen collection type",
        r#"frozen<$0>"#,
    ),
    // Simple types
    (
        "ASCII",
        "ASCII type",
        "ASCII - Character string type",
        r#"ASCII"#,
    ),
    (
        "ascii",
        "ascii type",
        "ASCII - Character string type",
        r#"ascii"#,
    ),
    (
        "BIGINT",
        "BIGINT type",
        "BIGINT - 64-bit signed integer",
        r#"BIGINT"#,
    ),
    (
        "bigint",
        "bigint type",
        "BIGINT - 64-bit signed integer",
        r#"bigint"#,
    ),
    ("BLOB", "BLOB type", "BLOB - Binary large object", r#"BLOB"#),
    ("blob", "blob type", "BLOB - Binary large object", r#"blob"#),
    (
        "BOOLEAN",
        "BOOLEAN type",
        "BOOLEAN - True or false",
        r#"BOOLEAN"#,
    ),
    (
        "boolean",
        "boolean type",
        "BOOLEAN - True or false",
        r#"boolean"#,
    ),
    (
        "COUNTER",
        "COUNTER type",
        "COUNTER - Distributed counter",
        r#"COUNTER"#,
    ),
    (
        "counter",
        "counter type",
        "COUNTER - Distributed counter",
        r#"counter"#,
    ),
    ("DATE", "DATE type", "DATE - Date without time", r#"DATE"#),
    ("date", "date type", "DATE - Date without time", r#"date"#),
    (
        "DECIMAL",
        "DECIMAL type",
        "DECIMAL - Variable-precision decimal",
        r#"DECIMAL"#,
    ),
    (
        "decimal",
        "decimal type",
        "DECIMAL - Variable-precision decimal",
        r#"decimal"#,
    ),
    (
        "DOUBLE",
        "DOUBLE type",
        "DOUBLE - 64-bit floating point",
        r#"DOUBLE"#,
    ),
    (
        "double",
        "double type",
        "DOUBLE - 64-bit floating point",
        r#"double"#,
    ),
    (
        "FLOAT",
        "FLOAT type",
        "FLOAT - 32-bit floating point",
        r#"FLOAT"#,
    ),
    (
        "float",
        "float type",
        "FLOAT - 32-bit floating point",
        r#"float"#,
    ),
    ("INET", "INET type", "INET - IP address", r#"INET"#),
    ("inet", "inet type", "INET - IP address", r#"inet"#),
    ("INT", "INT type", "INT - 32-bit signed integer", r#"INT"#),
    ("int", "int type", "INT - 32-bit signed integer", r#"int"#),
    (
        "SMALLINT",
        "SMALLINT type",
        "SMALLINT - 16-bit signed integer",
        r#"SMALLINT"#,
    ),
    (
        "smallint",
        "smallint type",
        "SMALLINT - 16-bit signed integer",
        r#"smallint"#,
    ),
    (
        "TEXT",
        "TEXT type",
        "TEXT - UTF-8 encoded string",
        r#"TEXT"#,
    ),
    (
        "text",
        "text type",
        "TEXT - UTF-8 encoded string",
        r#"text"#,
    ),
    ("TIME", "TIME type", "TIME - Time without date", r#"TIME"#),
    ("time", "time type", "TIME - Time without date", r#"time"#),
    (
        "TIMESTAMP",
        "TIMESTAMP type",
        "TIMESTAMP - Date and time",
        r#"TIMESTAMP"#,
    ),
    (
        "timestamp",
        "timestamp type",
        "TIMESTAMP - Date and time",
        r#"timestamp"#,
    ),
    (
        "TIMEUUID",
        "TIMEUUID type",
        "TIMEUUID - Version 1 UUID",
        r#"TIMEUUID"#,
    ),
    (
        "timeuuid",
        "timeuuid type",
        "TIMEUUID - Version 1 UUID",
        r#"timeuuid"#,
    ),
    (
        "TINYINT",
        "TINYINT type",
        "TINYINT - 8-bit signed integer",
        r#"TINYINT"#,
    ),
    (
        "tinyint",
        "tinyint type",
        "TINYINT - 8-bit signed integer",
        r#"tinyint"#,
    ),
    (
        "UUID",
        "UUID type",
        "UUID - Universally unique identifier",
        r#"UUID"#,
    ),
    (
        "uuid",
        "uuid type",
        "UUID - Universally unique identifier",
        r#"uuid"#,
    ),
    (
        "VARCHAR",
        "VARCHAR type",
        "VARCHAR - Variable-length string",
        r#"VARCHAR"#,
    ),
    (
        "varchar",
        "varchar type",
        "VARCHAR - Variable-length string",
        r#"varchar"#,
    ),
    (
        "VARINT",
        "VARINT type",
        "VARINT - Arbitrary-precision integer",
        r#"VARINT"#,
    ),
    (
        "varint",
        "varint type",
        "VARINT - Arbitrary-precision integer",
        r#"varint"#,
    ),
    // Non-parameterized collection types
    ("LIST", "LIST type", "LIST - Collection type", r#"LIST"#),
    ("list", "list type", "LIST - Collection type", r#"list"#),
    (
        "MAP",
        "MAP type",
        "MAP - Key-value collection type",
        r#"MAP"#,
    ),
    (
        "map",
        "map type",
        "MAP - Key-value collection type",
        r#"map"#,
    ),
    ("SET", "SET type", "SET - Unique collection type", r#"SET"#),
    ("set", "set type", "SET - Unique collection type", r#"set"#),
    ("TUPLE", "TUPLE type", "TUPLE - Composite type", r#"TUPLE"#),
    ("tuple", "tuple type", "TUPLE - Composite type", r#"tuple"#),
    (
        "FROZEN",
        "FROZEN type",
        "FROZEN - Frozen collection type",
        r#"FROZEN"#,
    ),
    (
        "frozen",
        "frozen type",
        "FROZEN - Frozen collection type",
        r#"frozen"#,
    ),
];

pub static TYPES: Lazy<Vec<CompletionItem>> = Lazy::new(|| {
    TYPES_DATA
        .iter()
        .map(|&(label, detail, doc, insert)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            detail: Some(detail.to_string()),
            documentation: Some(Documentation::String(doc.to_string())),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
});

const COMMAND_SEQUENCE_DATA: &[FnCompletionItem] = &[
    (
        "ALTER KEYSPACE",
        "ALTER KEYSPACE cql command",
        "ALTER KEYSPACE cql command",
        r#"ALTER KEYSPACE $0;"#,
    ),
    (
        "ALTER MATERIALIZED VIEW",
        "ALTER MATERIALIZED VIEW cql command",
        "ALTER MATERIALIZED VIEW cql command",
        r#"ALTER MATERIALIZED VIEW $0;"#,
    ),
];

pub static COMMAND_SEQUENCE: Lazy<Vec<CompletionItem>> = Lazy::new(|| {
    COMMAND_SEQUENCE_DATA
        .iter()
        .map(|&(label, detail, doc, insert)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(detail.to_string()),
            documentation: Some(Documentation::String(doc.to_string())),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
});

pub static UNION_COMMANDS_KEYWORDS: Lazy<Vec<CompletionItem>> = Lazy::new(|| {
    let mut sequence = Vec::new();
    sequence.extend(COMMAND_SEQUENCE.iter().cloned());
    sequence.extend(KEYWORDS.iter().cloned());
    sequence
});
