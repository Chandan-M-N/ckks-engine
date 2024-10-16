use crate::polynomial::Polynomial;
use rand::Rng;
use log::{info};

// Define a constant scaling factor for encoding and decoding
pub const SCALE: f64 = 1e7;  // Use a fixed scaling factor

// Rounds a given value to a specified number of decimal places
fn round_to(value: f64, decimal_places: usize) -> f64 {
    let factor = 10f64.powi(decimal_places as i32); // Calculate the rounding factor
    (value * factor).round() / factor  // Round the value and return
}

// Encode real numbers into polynomial form with scaling
pub fn encode(plaintext: &[f64], scaling_factor: f64) -> Polynomial {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");  // Ensure the scaling factor is positive
    }
    // Print the input plaintext and scaling factor
    info!("Encoding real numbers {:?} with scaling factor {}", plaintext, scaling_factor);
    
    // Scale the real numbers and convert them to integer coefficients
    let coeffs: Vec<i64> = plaintext.iter()
        .map(|&x| (x * scaling_factor).round() as i64)  // Scale the real numbers
        .collect();
    
    // Print the resulting polynomial coefficients
    info!("Encoded polynomial coefficients: {:?}", coeffs);
    
    Polynomial::new(coeffs)  // Return a new polynomial with the coefficients
}

// Decode polynomial back to real numbers
pub fn decode(ciphertext: &Polynomial, scaling_factor: f64) -> Vec<f64> {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");  // Ensure the scaling factor is positive
    }
    let threshold = 1e-10; // Define a small threshold for considering values as zero
    let decimal_places = 2; // Number of decimal places for rounding

    // Print the input ciphertext and scaling factor
    info!("Decoding polynomial coefficients {:?} with scaling factor {}", ciphertext.coeffs, scaling_factor);

    // Perform decoding (reverse scaling) and apply thresholding and rounding
    let decoded_values: Vec<f64> = ciphertext.coeffs.iter()
        .map(|&c| {
            let value = (c as f64) / scaling_factor;  // Reverse scaling
            let rounded_value = round_to(value, decimal_places); // Round the value to 2 decimal places
            // Apply thresholding to treat small values as zero
            if rounded_value.abs() < threshold {
                0.0  // Treat small values as zero
            } else {
                rounded_value  // Return the rounded value
            }
        })
        .collect();
    
    // Print the decoded real numbers
    info!("Decoded real numbers (with thresholding and rounding): {:?}", decoded_values);

    decoded_values  // Return the decoded values
}

// Add noise to a polynomial
pub fn add_noise(poly: &Polynomial, _pub_key: &impl std::fmt::Debug) -> Polynomial {
    let mut rng = rand::thread_rng();  // Create a random number generator
    // Generate noise for each coefficient of the polynomial
    let noise: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff + rng.gen_range(-10..10)).collect();
    info!("Adding noise to polynomial {:?}. Result after noise addition: {:?}", poly.coeffs, noise);
    Polynomial::new(noise)  // Return a new polynomial with added noise
}

// Modular reduction using the prime modulus q
pub fn mod_reduce(poly: &Polynomial, modulus: i64) -> Polynomial {
    // Reduce each coefficient of the polynomial modulo the given modulus
    let reduced: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff % modulus).collect();
    info!("Performing modular reduction on polynomial {:?}. Result after mod reduction: {:?}", poly.coeffs, reduced);
    Polynomial::new(reduced)  // Return a new polynomial with reduced coefficients
}

// Add this function to your utils.rs
pub fn polynomial_approximation(cipher: &Polynomial, operation: &str) -> Polynomial {
    // For simplicity, let's assume we can apply the operation directly
    // In practice, you'd need to use appropriate polynomial approximations
    let approximated_coeffs: Vec<i64> = cipher.coeffs.iter().map(|&coeff| {
        let value = (coeff as f64) / SCALE; // Decode the coefficient
        let approximated_value = match operation {
            "ceil" => value.ceil(),
            "floor" => value.floor(),
            "round" => value.round(),
            "truncate" => value.trunc(),
            _ => value,
        };
        // Re-encode the value
        (approximated_value * SCALE).round() as i64
    }).collect();

    Polynomial::new(approximated_coeffs)
}


// Polynomial approximation of the sigmoid function
pub fn sigmoid_approx(x: &Polynomial, modulus: i64) -> Polynomial {
    // Approximate sigmoid(x) ≈ 0.5 * SCALE + (x * SCALE) / 4 - (x^3 * SCALE) / 48
    let constant_term = Polynomial::new(vec![ (0.5 * SCALE) as i64; x.coeffs.len() ]);
    let linear_term = x.scalar_multiply(1.0 / 4.0);
    let x_square = x.multiply(&x);
    let x_cube = x_square.multiply(&x);
    let cubic_term = x_cube.scalar_multiply(-1.0 / 48.0);
    let result = constant_term.add(&linear_term).add(&cubic_term);
    mod_reduce(&result, modulus)
}


pub fn round_approx(x: &Polynomial, modulus: i64) -> Polynomial {
    // Compute x - (sigmoid_approx(x - 0.5 * SCALE) - 0.5 * SCALE)
    let half = Polynomial::new(vec![ (0.5 * SCALE) as i64; x.coeffs.len() ]);
    let x_minus_half = x.subtract(&half);
    let sigmoid = sigmoid_approx(&x_minus_half, modulus);
    let sigmoid_minus_half = sigmoid.subtract(&half);
    let result = x.subtract(&sigmoid_minus_half);
    mod_reduce(&result, modulus)
}

