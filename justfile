default: xpp

xpp:
    cargo build

run:
    cargo run

test:
    cargo test

scylla-setup:
    #!/bin/bash
    set -e

    docker stop scylla-tls scylla-notls 2>/dev/null || true
    docker rm scylla-tls scylla-notls 2>/dev/null || true

    docker run -it --rm --privileged --pid=host debian nsenter -t 1 -m -u -n -i sh -c "sysctl -w fs.aio-max-nr=1048576"

    mkdir -p certs

    openssl genrsa -out certs/ca.key 4096
    openssl req -x509 -new -nodes -key certs/ca.key -sha256 -days 365 -out certs/ca.crt -subj "/CN=TestCA"

    cat >certs/server.ext <<EOF
    subjectAltName = DNS:localhost, IP:127.0.0.1
    EOF

    openssl genrsa -out certs/server.key 4096
    openssl req -new -key certs/server.key -out certs/server.csr -subj "/CN=localhost"
    openssl x509 -req -in certs/server.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial \
    -out certs/server.crt -days 365 -sha256 -extfile certs/server.ext

    openssl genrsa -out certs/client.key 4096
    openssl req -new -key certs/client.key -out certs/client.csr -subj "/CN=client"
    openssl x509 -req -in certs/client.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial \
    -out certs/client.crt -days 365 -sha256

    docker run --name scylla-notls -d -p 9044:9042 \
        scylladb/scylla-enterprise \
        --smp 1 \
        --memory 750M \
        --overprovisioned 1 \
        --developer-mode 1

    docker run --name scylla-tls -d \
        -p 9043:9042 \
        -v "${PWD}/certs:/etc/scylla/certs" \
        -v "${PWD}/scylla/scylla.yaml:/etc/scylla/scylla.yaml" \
        scylladb/scylla-enterprise \
        --smp 1 \
        --memory 750M \
        --overprovisioned 1 \
        --developer-mode 1

    sleep 5

    docker exec scylla-tls mkdir -p /var/lib/scylla/.cassandra
    docker cp ./scylla/cqlshrc scylla-tls:/var/lib/scylla/.cassandra/cqlshrc

cql name:
    touch ./tests/cql/provided/{{name}}.txt
    touch ./tests/cql/expected/{{name}}.cql
