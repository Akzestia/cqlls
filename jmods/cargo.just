set working-directory := '../'

rebuild: clean build

build:
    cargo +nightly build

clean:
    cargo +nightly clean

run:
    cargo +nightly run

test:
    cargo +nightly test
