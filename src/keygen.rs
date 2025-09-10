use rand::Rng;  // Importing random number generator traits

// Struct to represent the public key containing two polynomials
#[derive(Debug, Clone)]
pub struct PublicKey {
    pub pk_0: Vec<i64>,  // First polynomial of the public key
    pub pk_1: Vec<i64>,  // Second polynomial of the public key
}

// Struct to represent the secret key containing a polynomial
#[derive(Debug, Clone)]
pub struct SecretKey {
    pub poly: Vec<i64>,  // Polynomial of the secret key
}

// Struct for key generation
pub struct KeyGenerator;

impl Default for KeyGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyGenerator {
    // Constructor to create a new KeyGenerator
    pub fn new() -> Self {
        KeyGenerator
    }

    // Function to generate a pair of public and secret keys
    pub fn generate_keys(&self) -> (PublicKey, SecretKey) {
        let mut rng = rand::rng();  // Create a random number generator

        // Generate secret key (random polynomial of size 2048)
        let sec_key_poly: Vec<i64> = (0..2048).map(|_| rng.random_range(1..100)).collect();
        let sec_key = SecretKey { poly: sec_key_poly.clone() };  // Create secret key using the generated polynomial

        // Generate a random polynomial for public key generation
        let random_poly: Vec<i64> = (0..2048).map(|_| rng.random_range(1..100)).collect();

        // Create public key polynomials
        let pk_0: Vec<i64> = sec_key_poly.iter().zip(&random_poly)
            .map(|(&sk, &r)| -sk * r + rng.random_range(-10..10))  // Compute pk_0 as -sk * random + noise
            .collect();
        let pk_1: Vec<i64> = random_poly;  // Set pk_1 to the random polynomial

        // Create public key with the generated polynomials
        let pub_key = PublicKey { pk_0, pk_1 };

        (pub_key, sec_key)  // Return the generated public and secret keys
    }


}
