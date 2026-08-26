set working-directory := '../'

rebuild: clean build

build:
    RUSTFLAGS="-Z threads=10" cargo +nightly build 

build-stable:
    cargo +stable build 

release:
    RUSTFLAGS="-Z threads=10" cargo +nightly build --release

release-stable:
    cargo +stable build --release

clean:
    cargo +nightly clean

run:
    cargo +nightly run

test:
    cargo +nightly test
