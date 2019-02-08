// Basic Euclidean.
// Time Complexity O(Log min(a,b))
pub fn basic_euclidean(a: u32, b: u32) -> u32 {
    // When a reaches 0, b is the gcd.
    if a == 0 {
        return b;
    }

    return basic_euclidean(b % a, a);
}

// Bezouts Identity -
// Let a and b be integers with a gcd of d. There exists integers, x and y such that..
// ax + by = d.
// The integers ax + by are exactly multiples of d.
// x and y are known as "Bezout coefficients".
// The "Bezout coeffficients are calculated using the extended Euclidean Algorithm.
//
// Calculating the inverse of x:
// p0 = 0
// p1 = 1
// calculate pi = pi-2 - (pi-1 * qi-2) % mod n
// qi-2 is the quotient from the Eulclidean Algorithm.
// This is run one step beyond the Euclidean Algorithm.
//
//         mod n = quotient(x)+ remainder
//
// Step 0: 26 = 1(15) + 11   p0 = 0

// Step 1: 15 = 1(11) + 4    p1 = 1

// Step 2: 11 = 2(4) + 3     p2 = (pi-2) - (pi - 1) * (qi-2) mod 26 = 0 - 1(1) % 26 = 25

// Step 3: 4 = 1(3) + 1      p3 = (pi-2) - (pi - 1) * (qi-2) mod 26 = 1 - 25(1) % 26 = 2
//
// Step 4: 3 = 3(1) + 0      p4 = (pi-2) - (pi - 1) * (qi-2) mod 26 = 25 - 2(2) % 26 = 21
//
// Step 5:                   p5 = (pi-2) - (pi - 1) * (qi-2) mod 26 = 2 - 21(1) % 26 = 7
//
// Notice: 15 * 7 = 105 = 1 + 4(26) == 1 mod 26.
// 7 is the inverse.
// If the remainder is not 1, then x does not have an inverse.
//

// Example 2
// Find the GCD OF 81 and 57 and then a, b where...
// 81(a) + 57(b) = r
//
// GCD(81, 57) = 3
// 81 = 1(57) + 24
// 57 = 2(24) + 9
// 24 = 2(9) + 6
// 9 = 1(6) + 3
// 6 = 2(3) + 0
//
// Reverse:
// 3 = 9 -1(6)
// 3 = 9 - 1(24 -2 (9)) = 3(9) - 1(24)
// 3 = 3(57 - 2(24)) - 1(24) = 3(57) - 7(24)
