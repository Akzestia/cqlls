#!/bin/bash

set -e

echo "=== Stopping and removing existing containers ==="
docker stop scylla-tls scylla-notls 2>/dev/null || true
docker rm scylla-tls scylla-notls 2>/dev/null || true

echo "=== Increasing AIO limit in Docker VM ==="
docker run -it --rm --privileged --pid=host debian nsenter -t 1 -m -u -n -i sh -c "sysctl -w fs.aio-max-nr=1048576"

echo "=== Generating certificates ==="
mkdir -p certs

# Generate CA
openssl genrsa -out certs/ca.key 4096
openssl req -x509 -new -nodes -key certs/ca.key -sha256 -days 365 -out certs/ca.crt -subj "/CN=TestCA"

# Generate server cert with SAN
cat > certs/server.ext << EOF
subjectAltName = DNS:localhost, IP:127.0.0.1
EOF

openssl genrsa -out certs/server.key 4096
openssl req -new -key certs/server.key -out certs/server.csr -subj "/CN=localhost"
openssl x509 -req -in certs/server.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial \
  -out certs/server.crt -days 365 -sha256 -extfile certs/server.ext

# Generate client cert (for mTLS)
openssl genrsa -out certs/client.key 4096
openssl req -new -key certs/client.key -out certs/client.csr -subj "/CN=client"
openssl x509 -req -in certs/client.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial \
  -out certs/client.crt -days 365 -sha256

echo "=== Starting ScyllaDB without TLS on port 9043 ==="
docker run --name scylla-notls -d -p 9043:9042 \
  scylladb/scylla \
  --smp 1 \
  --memory 750M \
  --overprovisioned 1 \
  --developer-mode 1

echo "=== Starting ScyllaDB with TLS on port 9042 ==="
docker run --name scylla-tls -d \
  -p 9042:9042 \
  -v "${PWD}/certs:/etc/scylla/certs" \
  -v "${PWD}/scylla/scylla.yaml:/etc/scylla/scylla.yaml" \
  scylladb/scylla

echo "=== Waiting for scylla-tls container to start ==="
sleep 5

echo "=== Copying cqlshrc to scylla-tls ==="
docker exec scylla-tls mkdir -p /var/lib/scylla/.cassandra
docker cp ./cqlshrc scylla-tls:/var/lib/scylla/.cassandra/cqlshrc

echo "=== Waiting for ScyllaDB to be ready ==="
echo "This may take 30-60 seconds..."

echo -n "Waiting for scylla-notls"
until docker exec scylla-notls cqlsh -e "DESCRIBE KEYSPACES" 2>/dev/null; do
  echo -n "."
  sleep 5
done
echo " Ready!"

echo -n "Waiting for scylla-tls"
until docker exec scylla-tls cqlsh -e "DESCRIBE KEYSPACES" 2>/dev/null; do
  echo -n "."
  sleep 5
done
echo " Ready!"

echo ""
echo "=== Setup Complete ==="
echo "scylla-notls: 127.0.0.1:9043 (no TLS)"
echo "scylla-tls:   127.0.0.1:9042 (TLS enabled)"
echo ""
echo "Certificates in ./certs/"
echo "  CA cert:     ./certs/ca.crt"
echo "  Server cert: ./certs/server.crt"
echo "  Server key:  ./certs/server.key"
echo "  Client cert: ./certs/client.crt (for mTLS)"
echo "  Client key:  ./certs/client.key (for mTLS)"
echo ""
echo "Test commands:"
echo "  docker exec scylla-notls cqlsh -e 'DESCRIBE KEYSPACES'"
echo "  docker exec scylla-tls cqlsh -e 'DESCRIBE KEYSPACES'"
