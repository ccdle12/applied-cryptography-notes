## Eulers Totient

Eulers Totient is represented using the greek alphebt **phi (*fiii*)** 

```
φ(n)
```

### Definition:
```
φ(n) = |{ x:1 ≤ x ≤ n, gcd(x, n) =1 }|
```

φ(n) = defined as, the amount of numbers between 1 and n, whose gcd is the number 1.

### Rules:

```
Rule 1: If p is prime, then φ(p) = p - 1

Rule 2: If a = p^n is a prime power then φ(p^n) = p^n - p^n-1

Rule 3: If the product (p) is from coprime numbers. If the gcd(m, n) = 1 then φ(mn) = φ(m)φ(n)
```

### Examples:

Basic:
```
φ(6) = 2
Number:      1 2 3 4 5 6
GCD  with 6: 1 2 3 2 1 6
             X       X
```

Rule 1:
```
φ(41) = 40

Every number between 1 and 40 has a gcd of 1, because 41 is prime.

gcd(1, 41) = 1, gcd(2, 41) = 1, gcd(3, 41) = 1, gcd(4, 41) = 1...
```

Rule 2:
```
φ(32) = 2^5
32 is a prime power of 2^5, 2 is the prime number to the power of a positive integer. 
We make a claim that the product is divisible by the base prime number.

φ(32) = 2^5 - 2^4 = 16
There are 16 numbers that has the gcd of 1.
```

Rule 3:
```
                                 we can use rule 1
φ(35) = φ(7 x 5) = φ(7) x φ(5) = (7 - 1) x (5 - 1) = 24
```
