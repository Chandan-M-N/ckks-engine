use crate::polynomial::Polynomial;
use crate::keygen::{PublicKey, SecretKey};
use crate::utils::{encode, decode, mod_reduce};

#[derive(Debug, Clone)]
pub struct CkksParameters {
    pub degree: usize,   // Polynomial degree N
    pub modulus: i64,    // Prime modulus q
}

impl CkksParameters {
    pub fn new(degree: usize, modulus: i64) -> Self {
        CkksParameters { degree, modulus }
    }
}

pub struct CKKSEncryptor {
    pub_key: PublicKey,
    params: CkksParameters,
}

impl CKKSEncryptor {
    pub fn new(pub_key: PublicKey, params: CkksParameters) -> Self {
        Self { pub_key, params }
    }

    pub fn encrypt(&self, plaintext: &[f64]) -> Polynomial {
        // Step 1: Encode the plaintext into a polynomial
        let scaling_factor = 1_000_000.0;  // Set a scaling factor for encoding
        let encoded = encode(plaintext, scaling_factor);
        println!("Encoded plaintext: {:?}", encoded);

        // Step 2: Use public key for encryption
        let encrypted_poly: Vec<i64> = encoded.coeffs.iter()
            .zip(&self.pub_key.pk_0)
            .zip(&self.pub_key.pk_1)
            .map(|((&e, &pk0), &pk1)| e + pk0 * pk1)  // Encrypt: encoded + pk_0 * pk_1
            .collect();
        let encrypted_polynomial = Polynomial::new(encrypted_poly);
        println!("Encrypted polynomial: {:?}", encrypted_polynomial);

        // Step 3: Perform modular reduction using the prime modulus
        let ciphertext = mod_reduce(&encrypted_polynomial, self.params.modulus);
        println!("Ciphertext (after mod reduction): {:?}", ciphertext);

        ciphertext
    }

    pub fn homomorphic_add(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before addition: {:?}", cipher1);
        println!("Ciphertext 2 before addition: {:?}", cipher2);

        // Add ciphertexts (assuming they have the same scaling factor)
        let result = cipher1.add(cipher2);
        println!("Result after homomorphic addition: {:?}", result);

        // Perform modular reduction using the prime modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);

        reduced_result
    }

    pub fn homomorphic_sub(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before subtraction: {:?}", cipher1);
        println!("Ciphertext 2 before subtraction: {:?}", cipher2);

        // Subtract ciphertexts
        let result = cipher1.sub(cipher2);
        println!("Result after homomorphic subtraction: {:?}", result);

        // Modular reduction using the prime modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);

        reduced_result
    }

    pub fn homomorphic_neg(&self, cipher: &Polynomial) -> Polynomial {
        println!("Ciphertext before negation: {:?}", cipher);

        // Negate each coefficient in the polynomial
        let negated_coeffs: Vec<i64> = cipher.coeffs.iter().map(|&c| -c).collect();
        let result = Polynomial::new(negated_coeffs);
        println!("Result after homomorphic negation: {:?}", result);

        // Perform modular reduction using the prime modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);

        reduced_result
    }

    pub fn homomorphic_mul(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before multiplication: {:?}", cipher1);
        println!("Ciphertext 2 before multiplication: {:?}", cipher2);

        let mut result_coeffs = vec![0; cipher1.coeffs.len() + cipher2.coeffs.len() - 1];

        // Perform polynomial multiplication (convolution of coefficients)
        for i in 0..cipher1.coeffs.len() {
            for j in 0..cipher2.coeffs.len() {
                result_coeffs[i + j] += cipher1.coeffs[i] * cipher2.coeffs[j];
            }
        }

        let result = Polynomial::new(result_coeffs);
        println!("Result after homomorphic multiplication (before scaling): {:?}", result);

        // Rescale the result by dividing by the scaling factor
        let scaling_factor = 1_000_000.0;
        let rescaled_coeffs: Vec<i64> = result.coeffs.iter().map(|&c| (c as f64 / scaling_factor) as i64).collect();
        let rescaled_poly = Polynomial::new(rescaled_coeffs);
        println!("Result after rescaling: {:?}", rescaled_poly);

        // Perform modular reduction using the prime modulus
        let reduced_result = mod_reduce(&rescaled_poly, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);

        reduced_result
    }

}

pub struct CKKSDecryptor {
    sec_key: SecretKey,
    params: CkksParameters,
}

impl CKKSDecryptor {
    pub fn new(sec_key: SecretKey, params: CkksParameters) -> Self {
        Self { sec_key, params }
    }

    pub fn decrypt(&self, ciphertext: &Polynomial, is_multiplication: bool) -> Vec<f64> {
        println!("Ciphertext before decryption: {:?}", ciphertext);

        // Step 1: Perform modular reduction to keep coefficients manageable
        let reduced_poly = mod_reduce(ciphertext, self.params.modulus);
        println!("Decrypted polynomial (after mod reduction): {:?}", reduced_poly);

        // Step 2: Apply the secret key to reverse the public key's effect
        let decrypted_poly: Vec<i64> = reduced_poly.coeffs.iter()
            .zip(&self.sec_key.poly)
            .map(|(&c, &sk)| c - sk)  // Subtract secret key influence
            .collect();
        let decrypted_polynomial = Polynomial::new(decrypted_poly);
        println!("Decrypted polynomial (after applying secret key): {:?}", decrypted_polynomial);

        // Step 3: Decode the result and scale it back to retrieve the original value
        let scaling_factor = 1_000_000.0;  // Use the same scaling factor as in encryption
        let decoded = decode(&decrypted_polynomial, scaling_factor, is_multiplication);
        println!("Decoded plaintext (after decryption): {:?}", decoded);

        decoded
    }
}
