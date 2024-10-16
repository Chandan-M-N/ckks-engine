use crate::utils::SCALE;

#[derive(Debug)]
pub struct Polynomial {
    pub coeffs: Vec<i64>,  // Coefficients for the polynomial
}

impl Polynomial {
    // Constructor to create a new Polynomial with given coefficients
    pub fn new(coeffs: Vec<i64>) -> Self {
        Polynomial { coeffs }
    }

    // Polynomial addition
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        // Determine the maximum length of the two polynomials
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0; len];  // Initialize result vector with zeros

        // Add coefficients of both polynomials
        for i in 0..len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { 0 };  // Get coefficient from self or 0 if out of bounds
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { 0 }; // Get coefficient from other or 0 if out of bounds
            result[i] = a + b;  // Add coefficients
        }

        Polynomial::new(result)  // Return new polynomial as the result
    }

    // Polynomial subtraction
    pub fn subtract(&self, other: &Polynomial) -> Polynomial {
        // Determine the maximum length of the two polynomials
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0; len];  // Initialize result vector with zeros

        // Subtract coefficients of the second polynomial from the first
        for i in 0..len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { 0 };  // Get coefficient from self or 0 if out of bounds
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { 0 }; // Get coefficient from other or 0 if out of bounds
            result[i] = a - b;  // Subtract coefficients
        }

        Polynomial::new(result)  // Return new polynomial as the result
    }

    // Polynomial multiplication
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let len = self.coeffs.len().min(other.coeffs.len());
        let mut result_coeffs = Vec::with_capacity(len);
        for i in 0..len {
            // Use i128 to prevent overflow
            let prod = (self.coeffs[i] as i128) * (other.coeffs[i] as i128);
            // Rescale by dividing by SCALE
            let scaled_prod = (prod / (SCALE as i128)) as i64;
            result_coeffs.push(scaled_prod);
        }
        Polynomial::new(result_coeffs)
    }

    // Polynomial negation
    pub fn negation(&self) -> Polynomial {
        // Negate each coefficient of the polynomial
        let negated_coeffs: Vec<f64> = self.coeffs.iter().map(|&c| -(c as f64)).collect();
        // Create a new polynomial with rounded negated coefficients
        Polynomial::new(negated_coeffs.iter().map(|&x| x.round() as i64).collect())
    }

    pub fn scalar_divide(&self, scalar: i64) -> Polynomial {
        let divided_coeffs: Vec<i64> = self.coeffs.iter().map(|&c| c / scalar).collect();
        Polynomial::new(divided_coeffs)
    }
    // pub fn scalar_multiply(&self, scalar: i64) -> Polynomial {
    //     let new_coeffs: Vec<i64> = self.coeffs.iter().map(|&c| (c * scalar) / (SCALE as i64)).collect();
    //     Polynomial::new(new_coeffs)
    // }

    pub fn scalar_multiply(&self, scalar: f64) -> Polynomial {
        let new_coeffs: Vec<i64> = self.coeffs.iter().map(|&c| {
            let scaled_value = (scalar * c as f64).round();
            scaled_value as i64
        }).collect();
        Polynomial::new(new_coeffs)
    }
}
