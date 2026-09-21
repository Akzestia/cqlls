set working-directory := '../'

rebuild: clean build

build target="//...":
    bazel build --config=nightly {{target}}

build-stable target="//...":
    bazel build {{target}}

release target="//:cqlls":
    bazel build --config=opt --config=nightly {{target}}

release-stable target="//:cqlls":
    bazel build --config=opt {{target}}

clean:
    bazel clean

run *args:
    bazel run --config=nightly //:cqlls -- {{args}}

test target="//...":
    bazel test --config=nightly {{target}}

test-scylla:
    bazel test --config=nightly //:scylla_no_tls //:scylla_tls //:scylla_schema_correctness
