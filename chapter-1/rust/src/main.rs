mod datastructs;
mod maths;
use self::datastructs::Bijection;

fn main() {}

// #[cfg(test)]
mod tests {

    #[test]
    fn test_transformation() {
        // Initialize set X.
        let mut x: Vec<u32> = Vec::new();
        for i in 1..17 {
            x.push(i);
        }
        assert_eq!(x.len(), 16);

        // Initialize set Y.
        let y = vec![3, 9, 10, 13, 5, 15, 11, 16, 14, 8, 7, 4, 12, 2, 6, 1];

        // Initialize a Bijection.
        let bijection = super::Bijection::new(&x, &y);

        // Assert that x's image is y.
        let res = bijection.transform();
        assert!(res);
    }

    #[test]
    fn larger_trapdoor_function() {
        // pq is a large prime.
        let p: u64 = 48611;
        let q: u64 = 53993;
        let pq: u64 = &p * &q;

        // x is a member of set X.
        let x: u64 = 2489991;
        let image: u64 = &x.pow(3) % &pq;

        // Only given the image, it is difficult to deduce the preimage.
        // f(x) = y | x^3 % pq = y
        // With only knowing - pq, y, 3, it is difficult to deduce x.
        assert_eq!(image, 1981394214);
    }

    #[test]
    fn basic_euclidean_test() {
        // Should find the greatest common divisor.
        let a = super::maths::basic_euclidean(15, 10);
        assert_eq!(a, 5);

        let b = super::maths::basic_euclidean(35, 10);
        assert_eq!(b, 5);

        let c = super::maths::basic_euclidean(31, 2);
        assert_eq!(c, 1);

        let d = super::maths::basic_euclidean(81, 57);
        assert_eq!(d, 3);
    }
}
