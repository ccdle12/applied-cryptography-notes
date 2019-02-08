pub struct Bijection {
    order: u32,
    base: u32,
    x: Vec<u32>,
    y: Vec<u32>,
}

impl Bijection {
    // Constructor.
    pub fn new(x: &Vec<u32>, y: &Vec<u32>) -> Bijection {
        Bijection {
            order: 17,
            base: 3,
            // Converts the reference.
            x: x.to_vec(),
            y: y.to_vec(),
        }
    }

    // Map f: X -> Y
    pub fn transform(&self) -> bool {
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
