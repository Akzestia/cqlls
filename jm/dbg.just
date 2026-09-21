set working-directory := '../'

fmt file="":
    bazel run --config=nightly //:cqlls -- --debug fmt {{file}}

cmt file="" line="0" char="0":
    bazel run //:cqlls -- --debug cmt {{file}} {{line}} {{char}}
