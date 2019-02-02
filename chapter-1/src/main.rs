fn main() {}

pub struct Bijection {
    order: u32,
    base: u32,
    x: Vec<u32>,
    y: Vec<u32>,
}

impl Bijection {
    // Constructor.
    pub fn new(x: Vec<u32>, y: Vec<u32>) -> Bijection {
        Bijection {
            order: 17,
            base: 3,
            x,
            y,
        }
    }

    // Map f: X -> Y
    fn transform(&self) -> bool {
        // Transformed Image of X.
        let mut image: Vec<u32> = Vec::new();

        // NOTE: (ccdle12) using &self.x to borrow x NOT change ownership.
        for c in &self.x {
            // Implement f: X -> Y
            let output: u32 = &self.base.pow(*c) % &self.order;

            println!{"output of function f: {}", output};

            // Add the image to the image vector.
            image.push(output);
        }

        // Compare image output for equality with set Y.
        for i in 0..image.len() {
            if &image[i] != &self.y[i] {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
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
        let bijection = super::Bijection::new(x, y);

        // Assert that x's image is y.
        let res = bijection.transform();
        assert!(res);
    }
}
