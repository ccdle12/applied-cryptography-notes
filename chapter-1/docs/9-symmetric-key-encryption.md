# Symmetric Key Encryption

## Overview of block ciphers and stream ciphers

For transformations of {Ee: e ∈ K} and {Dd: d ∈ K}. The encryption scheme is a symmetric-key for each pair (e,d) it is "easy" to computationally determine d from e.

A Block Cipher is an encryption scheme that breaks up plaintext messages of a fixed length **t** over an Alphabet of Definition **A** and encrypts each block at a time.

### Subsitution Ciphers

Are Block Ciphers that replace symbols by other symbols.

A is an alphabet of q symbols and M be the set of all strings of length t over A. K is the set of all permutations on the set A. For each e ∈ K an encryption scheme Ee:

```
Ee(m) = (e(m1)e(m2)···e(mt)) = (c1c2 ···ct) = c
```

M is the message space m = (m1m2...mt)

For each symbol in tuple t replace it by another symbol in A according to a fixed permutation. To decrypt the cipher text space c = (c1c2...ct) we need to compute the inverse permutation d = e^-1 

```
Dd(c) = (d(c1)d(c2)···d(ct)) = (m1m2 ···mt) = m.
```

Using the key space of the English Alphabet:

```
26! ≈ 4 × 10^26
```

Although even though the key space is large. The distribution of letter frequency is uniform. The letter 'E' is the most common letter, by viewing the letter frequency we can determine the secret d (the key).

### Polyalphabetic Subtition Ciphers

A substitution cipher that consists of...

* A key space K that consists of ordered sets of permutations `(p1, p2... pt)`, each permuation is defined on a set A.
* Encryption of a message `m = (m1m2 ...mt)` is performed under the key `e = (p1,p2,... ,pt)`. The encryption scheme is `Ee(m) = (p1(m1)p2(m2) · · · pt(mt))` a permutation of `pi` on each message space `mi`.
* Decryption is associated with the permutations `(p1, p2... pt)`, therefore `d = (p1^-1, p2^-1... pt^-1)`

Example
```
Let A = {A,B,C,... ,X,Y,Z}
t = 3
e = (p1, p2, p3)

* t is the length of each block
* p1 will be a shift by 3 places to the right
* p2 will be a shift by 7 places to the right
* p3 will be a shift by 10 places to the right

m =         THI SCI PHE RIS CER TAI NLY NOT SEC URE
c = Ee(m) = WOS VJS SOO UPC FLB WHS QSI QVD VLM XYO.
```

Polyalphabetic substitution ciphers have the advantage of hiding uniform distribution due to the different permutations. For example O has been shifted to E and H. Although if the length t is know, it becomes easy to view the distribution by looking at each t, block of symbols.
