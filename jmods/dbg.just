set working-directory := '../'

fmt file="":
    cargo +nightly run -- --debug fmt {{file}}

cmt file="" line="0" char="0":
    cargo run -- --debug cmt {{file}} {{line}} {{char}}
