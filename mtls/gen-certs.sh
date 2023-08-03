#/bin/sh

[ -d ca ] || mkdir ca

# Generate CA cert if it does not already exist
[ -f ca/ca.crt ] && echo "CA cert already exists. Skipping." || \
  (openssl genrsa -out ca/ca.key && \
  openssl req -new -x509 -key ca/ca.key -out ca/ca.crt)

# Generate server cert if it does not already exist
[ -f ca/server.bundle.crt ] && echo "Server cert already exists. Skipping." || \
  (openssl genrsa -out ca/server.key 4096 && \
  openssl req -new -key ca/server.key -addext "subjectAltName = DNS:localhost" -out ca/server.csr && \
  openssl x509 -req -in ca/server.csr -CA ca/ca.crt -CAkey ca/ca.key -CAcreateserial -extfile <(printf "subjectAltName=DNS:localhost") -out ca/server.crt && \
  cat ca/server.crt ca/ca.crt > ca/server.bundle.crt)

[ -f ca/client.pem ] && "Client cert already exists. Skipping." || \
  (openssl genrsa -out ca/client.key 4096 && \
  openssl req -new -key ca/client.key -addext "subjectAltName = DNS:localhost" -out ca/client.csr && \ 
  openssl x509 -req -in ca/client.csr -CA ca/ca.crt -CAkey ca/ca.key -CAcreateserial -extfile <(printf "subjectAltName=DNS:localhost") -out ca/client.crt && \
  cat ca/client.crt ca/client.key > ca/client.pem)
