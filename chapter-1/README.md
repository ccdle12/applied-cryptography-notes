## The Four Main Goals of Cryptography

1. Confidentiality

    Keep the content of the information secret to only those who have authorization

2. Data Integrity

    The data sent and arrives as it was intended, without the data being tampered

3. Authentication Service

   When 2 parties being communication they prove/verify who they are

4. Non-Repudiation ***TODO: update this more***

    A service that prevents a party from denying previous actions.

## Proofs / Mathematics / Glossary

### Sets
Denoted as `X = {a, b, c}`

In other words *X is a set containing a, b, c*.

### One way, 1-1, Trapdoor
A function is defined by *x2 sets X and Y*.

A rule *f* assigns *each element of X to Y*.

```
X(a) → Y(1)
X is known as the Domain 
Y is known as the Codomain
```

### Members
`x ∈ X`

The above means that *x is a member-of the set X*.

### Images

The image of X is the mapping in Y according to the rule *f*.

The image y of x is noted as:

`y = f(x)`

### Standard Notation of a function

The standard notation for a function between the sets X to Y is denoted as:

`f: X → Y`

### PreImage

If y is a member of Y:

`y ∈ Y`

Then the preimage of y is x as a member of X:

`x ∈ X`

The preimage can be denoted as follows:

`f(x) = y`

Denotes that all the sets in y have at least one preimage:

`Im(f)`

## Bijections

A bijection is when each element in the set of X is mapped/transformed to an element in the set of Y.

![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/bijection-1.png)

The above image is not technically a bijection because all members of the set X must be transformed to each set of Y.

![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/bijection-2.png)

A bijection is when each element in the set of X is mapped/transformed to an element in the set of Y.

The above image is a bijection that maps/transforms set of X to Y.

*f* is the bijection from *X to Y*.

`f(x) = y`

G is bijection from Y to X.

`g(y) = x`

The inverse can be defined as:

`g = f^-1`

In cryptography bijections are used to encrypt a message. When we reverse a bijection we are decrypting a message.

## One Way Functions

![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/one-way-function-1.png)
