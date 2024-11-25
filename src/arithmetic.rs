use crate::ckks::CKKSEncryptor;
use crate::polynomial::Polynomial;
use crate::utils::{mod_reduce};

impl CKKSEncryptor {

    // Function to perform homomorphic addition on two encrypted polynomials (ciphertexts)
    pub fn homomorphic_add(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {

        // Add the two polynomials (ciphertexts). Assuming the ciphertexts have the same scaling factor
        let result = cipher1.add(cipher2);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);

        // Return the reduced result as the final homomorphic addition result
        reduced_result
    }

    // Function to perform homomorphic subtraction on two encrypted polynomials (ciphertexts)
    pub fn homomorphic_subtract(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {

        // Subtract the second polynomial (cipher2) from the first (cipher1)
        let result = cipher1.subtract(cipher2);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);

        // Return the reduced result as the final homomorphic subtraction result
        reduced_result
    }

    // Function to perform homomorphic multiplication on two encrypted polynomials (ciphertexts)
    pub fn homomorphic_multiply(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
    
        // Multiply the two polynomials (ciphertexts). The result size is determined by the degree of the polynomials
        let result = cipher1.multiply(cipher2);
    
        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
    
        // Return the reduced result as the final homomorphic multiplication result
        reduced_result
    }    

    // Function to perform homomorphic negation on an encrypted polynomial (ciphertext)
    pub fn homomorphic_negation(&self, cipher1: &Polynomial) -> Polynomial {
        
        // Negate the coefficients of the polynomial (ciphertext)
        let negated_poly = cipher1.negation();
        
        // Perform modular reduction to ensure the negated result fits within the modulus
        let reduced_result = mod_reduce(&negated_poly, self.params.modulus);
        
        // Return the reduced result as the final homomorphic negation result
        reduced_result
    }

    // Function to perform homomorphic ceil on encrypted polynomials (ciphertexts)
    pub fn homomorphic_ceil(&self, cipher: &Polynomial) -> Polynomial {
        // This function will operate on encrypted coefficients
        let ceil_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| {
            let scaled_value = (c as f64) / 1e7; // scale down
            let ceil_value = scaled_value.ceil() as i64; // apply ceil
            (ceil_value as i64) * (1e7 as i64) // scale up back after ceil
        }).collect();

        // Return the new polynomial with ceil applied on encrypted data
        let ceil_polynomial = Polynomial::new(ceil_poly);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&ceil_polynomial, self.params.modulus);

        // Return the reduced result
        reduced_result
    }

    // Function to perform homomorphic floor on encrypted polynomials (ciphertexts)
    pub fn homomorphic_floor(&self, cipher: &Polynomial) -> Polynomial {
        // This function will operate on encrypted coefficients
        let floor_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| {
            let scaled_value = (c as f64) / 1e7; // scale down
            let floor_value = scaled_value.floor() as i64; // apply floor
            (floor_value as i64) * (1e7 as i64) // scale up back after floor
        }).collect();

        // Return the new polynomial with floor applied on encrypted data
        let floor_polynomial = Polynomial::new(floor_poly);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&floor_polynomial, self.params.modulus);

        // Return the reduced result
        reduced_result
    }


    // Function to perform homomorphic round on encrypted polynomials (ciphertexts)
    pub fn homomorphic_round(&self, cipher: &Polynomial) -> Polynomial {
        // Operate on encrypted coefficients
        let round_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| {
            let scaled_value = (c as f64) / 1e7; // Scale down
            let rounded_value = scaled_value.round() as i64; // Apply round
            (rounded_value as i64) * (1e7 as i64) // Scale up back after rounding
        }).collect();

        // Create a new polynomial with rounded coefficients
        let rounded_polynomial = Polynomial::new(round_poly);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&rounded_polynomial, self.params.modulus);

        // Return the reduced result
        reduced_result
    }

    pub fn homomorphic_truncate(&self, cipher: &Polynomial) -> Polynomial {
        let scale: i64 = 10_000_000; // Defining scaling factor as integer (1e7)

        // Truncate each coefficient by performing integer division and scaling back up
        let truncate_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| (c / scale) * scale).collect();

        let truncated_polynomial = Polynomial::new(truncate_poly);

        let reduced_result = mod_reduce(&truncated_polynomial, self.params.modulus);
        reduced_result
    }

    pub fn homomorphic_reciprocal(&self, cipher: &Polynomial, iterations: u32) -> Polynomial {
        let scale: i64 = 10_000_000; // Define scaling factor as integer (1e7)

        // Initialize the reciprocal with a closer initial guess
        let mut reciprocal = Polynomial::new(vec![scale / 2]); // Represents 0.5

        for _i in 0..iterations {
            // Step 1: Compute c * x_n / scale
            let temp = self.homomorphic_multiply(cipher, &reciprocal);
            let temp_coeff = temp.coeffs[0];

            // Step 2: Compute 2 * scale - temp_coeff
            let two_scale = scale * 2;
            let updated_coeff = two_scale - temp_coeff;

            // Step 3: Multiply the updated_coeff with the current reciprocal
            let updated_poly = Polynomial::new(vec![updated_coeff]);
            let multiplied = self.homomorphic_multiply(&updated_poly, &reciprocal);

            // Step 4: Update the reciprocal
            reciprocal = multiplied;
        }

        reciprocal
    }

    pub fn homomorphic_divide(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        let scaling_factor = 1e7; // Use a scaling factor for precision
 
        // Use the divide function from the Polynomial struct
        let result_poly = cipher1.divide(cipher2, scaling_factor);
 
        // Apply modular reduction to keep coefficients within the bounds of the modulus
        let reduced_result = mod_reduce(&result_poly, self.params.modulus);
 
        reduced_result // Return the final homomorphic division result
    }



    // Function to perform homomorphic exponentiation on an encrypted polynomial (ciphertext)
    pub fn homomorphic_exponentiation(&self, cipher: &Polynomial, exponent: u32) -> Polynomial {
        if exponent == 0 {
            // Return polynomial representing 1 (scaled by 1e7)
            return Polynomial::new(vec![10000000]); 
        }
        
        if exponent == 1 {
            return cipher.clone();
        }
    
        // Initialize the result with the original ciphertext
        let mut result = cipher.clone();

        // Perform repeated multiplication
        for _ in 1..exponent {
            // Multiply the result by cipher polynomial
            let temp = self.homomorphic_multiply(&result, cipher);
            result = temp;
        }
        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);

        reduced_result
    }

    pub fn homomorphic_divide_with_constant(&self, cipher: &Polynomial, constant: i64) -> Polynomial {
        // Gracefully handle the case when the constant is zero
        if constant == 0 {
            return cipher.clone(); // Return the original polynomial as is
        }

        // Scale the constant to match the ciphertext's scale
        let scaling_factor = 10_000_000; // Assuming 1e7 scaling factor
        let scaled_constant = constant * scaling_factor;

        // Compute the reciprocal of the scaled constant
        let reciprocal = scaling_factor / scaled_constant; // This is effectively 1/constant in scaled form

        // Multiply the ciphertext by the reciprocal
        let scaled_reciprocal_poly = Polynomial::new(vec![reciprocal]);
        let result = self.homomorphic_multiply(cipher, &scaled_reciprocal_poly);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);

        reduced_result
    }


}
