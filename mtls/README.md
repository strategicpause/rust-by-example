# Introduction
TLS, or Transport Layer Security, is an encryption protocol used to authenticate client-server connections. After a connection has been authenticated, then data sent over the connection is encrypted using the server's public key. TLS works by having the server present a certificate to the client when it attempts to establish a connection. The certificate consists of the server's public key and a digital signature, which is generated using the server's private key. Using the public key, the client can validate the digital signature which indicates the server possesses its private key. Once the client validates the authenticity of the certificate, then the client and server can exchange information over the encrypted TLS connection.

With TLS only the server is authenticated by the client, but not the other way around. mTLS, or mutual TLS, provides a method to authenticate both the client and the server when establishing a connection. This kind of mutual authentication is seen in zero trust security architectures.  

# Generating Certificates
## Certificate Authority (CA)
1. Generate a 4096-bit RSA private key, for the CA, using the PKCS#8 format.
~~~~
openssl genrsa -out ca.key 4096
~~~~
2. Using the private key generated in the previous step, this will generate a self-signed x509 certificate for the CA using the `req` command. `req` is used to create and process certificate requests in the PKCS#10 format.
~~~~
openssl req -new -x509 -key ca.key -out ca.crt
~~~~

## Server
1. Generate 4096-bit RSA private key, for the server.
~~~~
openssl genrsa -out server.key 4096
~~~~
2. Generate a Certificate Signing Request (CSR) using the server's private key. A CSR contains information about the server which the CA will use when generating a certificate.
~~~~
openssl req -new -key server.key -addext "subjectAltName = DNS:localhost" -out server.csr
~~~~
3. Using the certificate of the CA, and the CSR, this will create an x509 certificate for the server.
~~~~
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -extfile <(printf "subjectAltName=DNS:localhost") -out server.crt
~~~~
4. Create a file which is a bundle of the server certificate and the CA certificate.
~~~~
cat server.crt ca.crt > server.bundle.crt
~~~~

## Client
1. Generate a 4096-bit RSA private key for the client.
~~~~
openssl genrsa -out client.key 4096
~~~~

2. Create a CSR for the client.
~~~~
openssl req -new -key client.key -addext "subjectAltName = DNS:localhost" -out client.csr
~~~~

3. Use the CA to sign it.
~~~~
openssl x509 -req -in client.csr -CA ca.crt -CAkey ca.key -CAcreateserial -extfile <(printf "subjectAltName=DNS:localhost") -out client.crt
~~~~
4. Generate PEM file.
~~~~
cat client.crt ca.crt > client.pem
~~~~
