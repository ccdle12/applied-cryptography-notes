## One Way Functions

![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/one-way-function-1.png)

In the example above...

`X is a set X = {1, 2, 3... 16}`

The images are shown in the row `f(x)`

The function to transform each `x ∈ X` is `3^x % 17 = y`

Given the image of 3, `3^x % 17 = 3` it is easy to deduce the preimage is `x = 3`.

For other images it is not so simple, for example the image 7 `3^x % 17 = 7`, the answer straight is not above but the preimage is 11.

Given dealing with very large numbers, it becomes very difficult to deduce the the preimage `x`. This is the basis for trap-door functions used in `Public Key Cryptography`.

A trap door function is a one way function `f: X -> Y`.

Given the `trapdoor information` it becomes possible to find...

any given `y ∈ Im(f)` (any *y* that is a member of the images for the transformation according to function *f*), it is possible to deduce
the preimage `x ∈ X` such that `f(x) = y`.

Trapdoor functions are computationally infeasible to solve without the *trapdoor information*. If we have the *trapdoor information* it becomes possible to deduce the preimage given the image and function.
