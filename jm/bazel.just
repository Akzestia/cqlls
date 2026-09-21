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

install root="${CARGO_HOME:-$HOME/.cargo}": (_install "--config=nightly" root)

install-stable root="${CARGO_HOME:-$HOME/.cargo}": (_install "" root)

_install flags root:
    #!/usr/bin/env bash
    set -euo pipefail
    bazel build --config=opt {{flags}} //:cqlls
    bin="$(bazel cquery --config=opt {{flags}} --output=files //:cqlls 2>/dev/null)"
    dest="{{root}}/bin"
    mkdir -p "$dest"
    install -m 755 "$bin" "$dest/cqlls"
    echo "Installed cqlls to $dest/cqlls"
