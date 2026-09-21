set dotenv-load

mod bazel 'jm/bazel.just'
mod debug 'jm/dbg.just'

default: bazel::build

build: bazel::build 

release: bazel::release

build-stable: bazel::build-stable

release-stable: bazel::release-stable

clean: bazel::clean

run *args: (bazel::run args)
    
test: bazel::test

test-scylla: bazel::test-scylla

fmt file="": (debug::fmt file)

cmt file="" line="0" char="0": (debug::cmt file line char)
