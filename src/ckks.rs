use crate::polynomial::Polynomial;
use crate::keygen::{PublicKey, SecretKey};
use crate::utils::{encode, decode, mod_reduce,encode_string,decode_string,mod_reduce_string};

// Struct to hold CKKS parameters
#[derive(Debug, Clone)]
pub struct CkksParameters {
    pub degree: usize,   // Polynomial degree N
    pub modulus: i64,    // Prime modulus q
}

impl CkksParameters {
    // Constructor to create new CKKS parameters
    pub fn new(degree: usize, modulus: i64) -> Self {
        CkksParameters { degree, modulus }
    }
}

// Struct for CKKS Encryptor containing public key and parameters
pub struct CKKSEncryptor {
    pub pub_key: PublicKey,   // Public key for encryption
    pub params: CkksParameters,  // Parameters for encryption
}

impl CKKSEncryptor {
    // Constructor to create a new CKKS Encryptor
    pub fn new(pub_key: PublicKey, params: CkksParameters) -> Self {
        Self { pub_key, params }
    }

    // Function to encrypt a collection of plaintext values
    pub fn encrypt_collection<T>(&self, plaintext: &[T]) -> Polynomial
    where
        T: Into<f64> + Copy,  // T can be converted into f64 and must implement Copy
    {
        let scaling_factor = 1e7; // Set scaling factor for encoding
        
        // Step 2: Encode the plaintext into a polynomial
        let plaintext_f64: Vec<f64> = plaintext.iter().map(|&x| x.into()).collect();  // Convert to f64
        let encoded = encode(&plaintext_f64, scaling_factor);

        // Step 3: Use public key for encryption
        let encrypted_poly: Vec<i64> = encoded.coeffs.iter()
            .zip(&self.pub_key.pk_0)
            .zip(&self.pub_key.pk_1)
            .map(|((&e, &pk0), &pk1)| e + pk0 * pk1)  // Encrypt: encoded + pk_0 * pk_1
            .collect();
        let encrypted_polynomial = Polynomial::new(encrypted_poly);

        // Step 4: Perform modular reduction using the prime modulus
        let ciphertext = mod_reduce(&encrypted_polynomial, self.params.modulus);

        ciphertext  // Return ciphertext
    }

    // Function to encrypt a single plaintext value
    pub fn encrypt_value<T>(&self, plaintext: T) -> Polynomial
    where
        T: Into<f64> + Copy, // Accepts a type that can be converted into f64
    {
        // Step 1: Convert the input value into a vector of f64
        let plaintext_vec: Vec<f64> = vec![plaintext.into()];

        // Step 2: Encode the plaintext into a polynomial
        let scaling_factor = 1e7; // Set a scaling factor for encoding
        let encoded = encode(&plaintext_vec, scaling_factor);

        // Step 3: Use public key for encryption
        let encrypted_poly: Vec<i64> = encoded.coeffs.iter()
            .zip(&self.pub_key.pk_0)
            .zip(&self.pub_key.pk_1)
            .map(|((&e, &pk0), &pk1)| e + pk0 * pk1) // Encrypt: encoded + pk_0 * pk_1
            .collect();
        let encrypted_polynomial = Polynomial::new(encrypted_poly);

        // Step 4: Perform modular reduction using the prime modulus
        let ciphertext = mod_reduce(&encrypted_polynomial, self.params.modulus);

        ciphertext  // Return ciphertext
    }

    // Function to encrypt a string
    pub fn encrypt_string(&self, plaintext: &str) -> Polynomial {
        let scaling_factor = 1e9; // Set scaling factor for encoding
        
        // Step 1: Encode the plaintext string into a polynomial
        let encoded = encode_string(plaintext, scaling_factor);  // Use encode_string for string
    
        // Step 2: Ensure the public keys and encoded polynomial match in length
        if self.pub_key.pk_0.len() < encoded.coeffs.len() || self.pub_key.pk_1.len() < encoded.coeffs.len() {
            panic!("Public key length is insufficient for encryption.");
        }
    
        // Step 3: Use public key for encryption
        let encrypted_poly: Vec<i64> = encoded.coeffs.iter()
            .zip(&self.pub_key.pk_0)
            .zip(&self.pub_key.pk_1)
            .map(|((e, pk0), pk1)| e + pk0 * pk1)  // Encrypt: encoded + pk_0 * pk_1
            .collect();
        
        let encrypted_polynomial = Polynomial::new(encrypted_poly);
    
        // Step 4: Perform modular reduction using the prime modulus
        let ciphertext = mod_reduce_string(&encrypted_polynomial, self.params.modulus);
        ciphertext  // Return ciphertext
    }
    
}

// Struct for CKKS Decryptor containing secret key and parameters
pub struct CKKSDecryptor {
    sec_key: SecretKey,   // Secret key for decryption
    params: CkksParameters,  // Parameters for decryption
}

impl CKKSDecryptor {
    // Constructor to create a new CKKS Decryptor
    pub fn new(sec_key: SecretKey, params: CkksParameters) -> Self {
        Self { sec_key, params }
    }
    //decrypt for int
    pub fn decrypt_as_int(&self, ciphertext: &Polynomial) -> Vec<i64> {
        // Log the ciphertext before decryption

        // Step 1: Perform modular reduction
        let reduced_poly = mod_reduce(ciphertext, self.params.modulus);

        // Step 2: Apply the secret key to reverse the encryption
        let decrypted_poly: Vec<i64> = reduced_poly.coeffs.iter()
            .zip(&self.sec_key.poly)
            .map(|(&c, &sk)| c - sk) // Subtract secret key influence
            .collect();
        let decrypted_polynomial = Polynomial::new(decrypted_poly);

        // Step 3: Decode the decrypted polynomial to get integer values
        let decoded = decrypted_polynomial.decode(); // Vec<i64>

        decoded
    }

    // Function to decrypt a ciphertext polynomial
    pub fn decrypt(&self, ciphertext: &Polynomial) -> Vec<f64> {
        // Print the ciphertext before decryption for debugging

        // Step 1: Perform modular reduction to keep coefficients manageable
        let reduced_poly = mod_reduce(ciphertext, self.params.modulus);

        // Step 2: Apply the secret key to reverse the public key's effect
        let decrypted_poly: Vec<i64> = reduced_poly.coeffs.iter()
            .zip(&self.sec_key.poly)
            .map(|(&c, &sk)| c - sk)  // Subtract secret key influence
            .collect();
        let decrypted_polynomial = Polynomial::new(decrypted_poly);

        let scaling_factor = 1e7; // Set scaling factor for decoding
        // Step 3: Decode the result and scale it back to retrieve the original value
        let decoded = decode(&decrypted_polynomial, scaling_factor);

        decoded // Return the decoded plaintext values
    }

    // Function to decrypt a ciphertext polynomial back into a string
    pub fn decrypt_string(&self, ciphertext: &Polynomial) -> String {
        let scaling_factor = 1e9; // Set scaling factor for decoding
        
        // Step 1: Perform modular reduction
        let reduced_poly = mod_reduce_string(ciphertext, self.params.modulus);
        
        // Step 2: Apply the secret key to reverse the encryption process
        let decrypted_poly: Vec<i64> = reduced_poly.coeffs.iter()
            .zip(&self.sec_key.poly)
            .map(|(&c, &sk)| c - sk)  // Subtract the secret key
            .collect();
        
        let decrypted_polynomial = Polynomial::new(decrypted_poly);
    
        // Step 3: Decode the polynomial back into a string
        let decoded_string = decode_string(&decrypted_polynomial, scaling_factor);  // Use decode_string for string
        decoded_string  // Return the decrypted string
    }
    
}
