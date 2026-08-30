set dotenv-load

mod cargo 'jm/cargo.just'
mod debug 'jm/dbg.just'

default: cargo::build

build: cargo::build 

release: cargo::release

build-stable: cargo::build-stable

release-stable: cargo::release-stable

clean: cargo::clean

run *args: (cargo::run args)
    
test: cargo::test

fmt file="": (debug::fmt file)

cmt file="" line="0" char="0": (debug::cmt file line char)
