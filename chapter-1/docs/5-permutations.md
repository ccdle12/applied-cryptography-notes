## Permutations

![alt text](https://raw.githubusercontent.com/ccdle12/applied-cryptography-notes/master/images/permuation-1.png)

A permutation is a bijection to itself. Let S be a set `S = {1, 2, 3, 4, 5}`. 

A permuation of S to itself can expressed as `p: S -> S`

Permutations are bijections so can also have inverses.

Similar to trapdoor functions, reversing the permuation is computationally infeasible without certain information for example 

```
Let X be the set of integers {0,1,2,3... pq−1} where p and q are distinct 
large primes(for example,p and q are each about 100 decimal digits long), and 
suppose that neither p−1 nor q−1 is divisible by 3.

Without pq, this permuation becomes infeasible to reverse
```
