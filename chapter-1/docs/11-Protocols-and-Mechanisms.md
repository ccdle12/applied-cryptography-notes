# Protocols and Mechanisms

## Key Establishment, management and certification

* *key establishment* is a process where a shared secret becomes available to two parties.

* *key management* is the process of support key establishment and the maintanence of the
  key relationship. It also deals with replacing older keys with new ones when necessary.

## Key Management through symmemtric-key techniques

A key management solution that employs symemtric-key techqniues uses a Trusted Third Party
(an entity in the network trusted by all parties).

Each enity *Ai* shares a unique symmetric key *ki* with the TTP (trusted third party).
The keys, *ki* are assumed to be shared over a secure channel.

If two parties want to communicate *A1* and *A5*, the TTP will generate a key *k* (session key)
and sends the encrypted session key according to each entities key held at the TTP. Therefore
each entity does not need to hold n = n(n-1) / 2 number of keys.

### Advantages

* Adding and removing entities from the network is easy.
* Each entity only stores one long term sercret key (the key pair shared with the TTP).

### Disadvantages

* Single Point of failure the TTP, if compromised all messages are insecure.
* All communication requires an initial communication with the TTP.
* The TTP must store n = n(n-1) / 2 number of keys.
* The TTP can read all messages.

TODO:
<INSERT IMAGE>
