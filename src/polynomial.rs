use std::panic;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coeffs: Vec<i64>,  // Coefficients for the polynomial
}

impl Polynomial {
    // Constructor to create a new Polynomial with given coefficients
    pub fn new(coeffs: Vec<i64>) -> Self {
        Polynomial { coeffs }
    }

    //wrote this only for homomorphic truncate - we haven't used this anywhere apart from it
    pub fn decode(&self) -> Vec<i64> {
        self.coeffs.iter().map(|&c| {
            let real = (c as f64) / 10_000_000.0;
            real.round() as i64 // Round to the nearest integer
        }).collect()
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
        // Determine size for the resulting polynomial
        let result_size = self.coeffs.len().max(other.coeffs.len());
        let mut result_coeffs = vec![0.0; result_size]; // Initialize result coefficients with f64 for scaling

        // Multiply matching coefficients of both polynomials
        let min_len = self.coeffs.len().min(other.coeffs.len());
        for i in 0..min_len {
            result_coeffs[i] = (self.coeffs[i] as f64 * other.coeffs[i] as f64) / 1e7; // Scale and store the product
        }
        // Create a new polynomial with rounded coefficients
        Polynomial::new(result_coeffs.iter().map(|&x| x.round() as i64).collect())
    }

    // Polynomial negation
    pub fn negation(&self) -> Polynomial {
        // Negate each coefficient of the polynomial
        let negated_coeffs: Vec<f64> = self.coeffs.iter().map(|&c| -(c as f64)).collect();
        // Create a new polynomial with rounded negated coefficients
        Polynomial::new(negated_coeffs.iter().map(|&x| x.round() as i64).collect())
    }

    pub fn divide(&self, divisor: &Polynomial, scaling_factor: f64) -> Polynomial {
        let mut result_coeffs = Vec::new();

        // Handle scalar division if divisor has only one coefficient
        if divisor.coeffs.len() == 1 {
            let scalar = divisor.coeffs[0];
            if scalar == 0 {
                eprintln!("Division by zero encountered in scalar divisor. Skipping computation.");
                return Polynomial::new(vec![]); // Return an empty polynomial
            }

            for a in &self.coeffs {
                let result = panic::catch_unwind(|| {
                    let scaled_result = (*a as f64 / scalar as f64) * scaling_factor;
                    scaled_result.round() as i64
                });

                match result {
                    Ok(value) => result_coeffs.push(value),
                    Err(_) => {
                        eprintln!("Panic occurred during scalar division. Skipping coefficient.");
                        result_coeffs.push(0); // Default value on panic
                    }
                }
            }
        } else {
            // Handle polynomial division
            for i in 0..std::cmp::max(self.coeffs.len(), divisor.coeffs.len()) {
                let a = self.coeffs.get(i).copied().unwrap_or(0); // Default to 0 if self.coeffs is shorter
                let b = divisor.coeffs.get(i).copied().unwrap_or(0); // Default to 0 for missing divisor terms

                if b == 0 {
                    eprintln!(
                        "Division by zero encountered at index {i}. Defaulting to 0."
                    );
                    result_coeffs.push(0);
                    continue;
                }

                let result = panic::catch_unwind(|| {
                    let scaled_result = (a as f64 / b as f64) * scaling_factor;
                    scaled_result.round() as i64
                });

                match result {
                    Ok(value) => result_coeffs.push(value),
                    Err(_) => {
                        eprintln!("Panic occurred during polynomial division at index {i}. Defaulting to 0.");
                        result_coeffs.push(0); // Default value on panic
                    }
                }
            }
        }

        // Return a new polynomial with the resulting coefficients after division
        Polynomial::new(result_coeffs)
    }
}
