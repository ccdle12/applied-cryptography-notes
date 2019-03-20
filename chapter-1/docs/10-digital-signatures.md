# Digital Signatures

It is a means for an individual to bind their identity to a piece of information.

## Nomenclature and set-up

* *M* is the set of messages that will be signed.
* *S* is a set of signatures.
* *Sa* is the transformation from the message set *M* to the signature set *S*, and it is called the *signing transformation* for entity *A*. The transformation is kept a secret by A and is used to create signatures for messages *M*.
* *Va* is the verification transformation from the set M x S to the set *{true, false}*. *Va* is called a verification transformation for A's signature. It is publicly  known and used by other entites to verfiy signatures created by A.


![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/signature-transformation-1.png)

* M = {m1, m2, m3} and S = {s1, s2, s3}
M is the set of messages and S is the set of signatures.

* The product of M x S is known as the Cartesian Product of M and S.

## Signing Procedure

A (alice) creates a signature for a message *m* ∈ M:

* Computes s = Sa(m)
* Sends the pair (m, s). *s* is the signature and *m* is the message.

## Verification Procedure

The verifier B (bob) verifys the signature *s* on a message *m* was created by A.

* B uses the verification function Va of A.
* Computes u = Va(m, s)
* If u == true: signature is created by A. If u == false: signature was not created by A.
