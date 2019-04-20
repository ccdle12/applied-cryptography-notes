# Symmetic Keys vs Public Keys

## Advantages of Symmmetric Keys

* Symmetric Keys tend to be small (compared to Public Keys)
* Designed for high data throughput (encrypt rates)
* Used a base primitive for constructing cryptographic mechanisms

## Disadvantages of Symmetric Keys

* Both parties must keep the encryption key and decryption key secret at all times.
* The encryption key must be initially sent over a secure channel.
* In a large network, multiple, numerous or many keys must be kept for communication with multiple parties on a network.

## Advantages of Public Keys

* Only the Private Key needs to be kept, Public Keys can be openly shared.
* Public/Private Key Pairs may remain in use for a long time.
* Digital Signature Algorithms are efficient.

## Disadvantages of Public Keys

* Throughput is slower (compared to Symmetric Keys).
* Key sizes are much larger (compared to Symmetric Keys).

## Summary

* Public Key cryptography excels key management, signature generation and non-repudiation.
* Symmetric Keys excels in encryption for data integrity applications and communication.

### Note:
* Private Keys in a public-key system (RSA - 1024 bits)
* Secret Keys in a symmetric-key system (64 or 128 bits)
