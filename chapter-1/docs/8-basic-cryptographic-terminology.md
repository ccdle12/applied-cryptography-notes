## Basic Cryptographic Terminology

### Encryption domains and codomains

#### *A* denotes a set called the *alphabet of definition*.

```
A = {0, 1} is the binary alphabet
```

The binary alphabet is frequently used, we can encode strings and messages.

#### M denotes a set called the *message space*.

The message consists of strings and symbols made up of the *alphabet of definition*.
An element of *M* is referred to as a *plain text message* or simply *plain text*.

#### C denotes a set called the *cipher text space*.

The cipher text space consists of the *alphabet of definition* that differs from the *message space*.
An element of the *cipher text space* is known as a *cipher text message*.

### Encryption and decryption transformations

#### K denotes a set called the *Key space*.

Each `e ∈ K` uniquely determines a bijection from *M* to *C*, denoted by *Ee*.

#### *Ee* is the *encryption function* or *encryption transformation*. 

*Ee* must be a bijection for the process to be reversed (*decrypted*) and reveal a unique plain text message for each
cipher text message.

* Each d ∈ K, Dd denotes a bijection from C to M (i.e., C -> M).

#### Dd is known as the *decryption function* or *decryption transformation*.

* The process of applying *Ee* to a message `m ∈ M` is referred to as *encryption of m* or encrypting *m*.

* The process of applying *Dd* to a *cipher text message* is referred to as *decrypting c* or the *decryption of c*.

* An *encryption scheme consists* of a set `{Ee: e ∈ K}` of encryption transformations and a corresponding set `{Dd: d ∈ K}`
of decryption transformations. It has the property that for each `e ∈ K` there is a unique key `d ∈ K` such that `Dd = E^-1e` (encryption is reversed); that is `Dd(Ee(m)) = m` (decrypt the encryption of the message m to receive m).

* The keys *e* and *d* the previous definition is referred to as a *key pair* and is sometimes denoted as (e, d). *e* and *d* could be the same.

* To *construct* an encryption scheme, it requires one to select a message space *M*, a ciphertext space *C*, a key space *K*, a set of encryption transformations `{Ee: e ∈ K}` and a corresponding set of decryption transformations `{Dd: d ∈ K}`.

## Achieving Confidentiality

Two parties Alice and Bob, first choose and exchange a key pair secretly `(e, d)`.
If Alice wants to send a message `m ∈ M` to Bob, she computes `c = Ee(m)` and sends `c` to Bob.
When Bob receives `c`, decrypts it using `Dd(c) = m` and recovers the original message `m`.

Why use keys as oppossed to one encryption function and corresponding decryption function?

Using keys, we can continously change the keys. This is similar to having a pad lock and continously changing the combination.

![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/transformation-example-1.png)

The above example, Alice encrypts message m1, Alice encrypts `E1(m1) = c3` and sends `c3` to Bob. 
Bob decrypts `c3` by reversing the arrows, to reveal m1.

## Communication and Participants
![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/transformation-example-2.png)
 
* *An entity* is a person, thing or computer that sends, receives or manipulates information. In the above image, Alice and Bob are *entities*.

* *A sender* is the legitimate participants in a two way communication, that is the original and legitimate sender of an encrypted message. In the above image, *Alice* is the sender.

* *A receiver* is the intended receiver of the encrypted message. In the above image, *Bob* is the receiver.

* *An adversary* is a third party that is not part of a two way communication. The adversary is trying to defeat the security system.

## Channels
* It is a means of communicating from one entity to another.

* *A physically secure channel* or *secure channel* is a channel that is not physically accessible to the *adversary*

* *An unsecure* channel, is one that allows an *adversary* to read, delete or reorder the information/message.

* *A secured* channel, is one where the *adversary* cannot affect the channel - reorder, delete, insert or read. 
