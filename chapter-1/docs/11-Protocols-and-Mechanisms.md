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

TODO:
<INSERT IMAGE>

### Advantages

* Adding and removing entities from the network is easy.
* Each entity only stores one long term sercret key.

### Disadvantages

* Single Point of failure the TTP, if compromised all messages are insecure.
* All communication requires an initial communication with the TTP.
* The TTP must store n = n(n-1) / 2 number of keys.
* The TTP can read all messages.

## Key Management through public-key techniques

The following is a very simple model.

Each entity in the network *Ai* has a public/private key pair. The public key and the identity
of the owner is stored in a central repository known as the *public file*.

When entity *A1* wants to send a message to entity *A6*, *A1* retrieves the public key of *A6*
from the *public file*, encrypts the message using the public key of of *A6* and sends the 
ciphter text over the wire to *A6*.


TODO:
<INSERT IMAGE>

```
Message encryption using A6 Public Key

c = Ee6(m)

Message decryption at A6

m = Dd6(c)
```

### Advantages

* No TTP required
* The public file can live on each entity
* only n number of public keys need to be stored

### Disadvantages

#### MIM (man in the middle)

An active adversary who can alter the public file creates a problem in key management.


TODO:
<INSERT IMAGE>

In the image above, the adversary replaces the public key *e6* of entity *A6* in the public file
with * e* * which is the public key of the adversary.

Any messages meant for *A6* can only be decrypted by the adversary. The adversary can decrypt
and read the message, and then encrypt the message with the public key of *A6*, and send it on to
*A6*.





