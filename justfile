set dotenv-load

mod cargo 'jmods/cargo.just'
mod debug 'jmods/dbg.just'

default: cargo::build

build: cargo::build 

clean: cargo::clean

run: cargo::run
    
test: cargo::test

fmt file="": (debug::fmt file)

cmt file="" line="0" char="0": (debug::cmt file line char)
