# Digital Signatures

It is a means for an individual to bind their identity to a piece of information.

## Nomenclature and set-up

* *M* is the set of messages that will be signed.
* *S* is a set of signatures.
* *Sa* is the transformation from the message set *M* to the signature set *S*, and it is called the *signing transformation* for entity *A*. The transformation is kept a secret by A and is used to create signatures for messages *M*.
* *Va* is the verification transformation from the set M x S to the set *{true, false}*. *Va* is called a verification transformation for A's signature. It is publicly  known and used by other entites to verfiy signatures created by A.

