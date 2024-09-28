use rand::Rng;

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub pk_0: Vec<i64>,
    pub pk_1: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    pub poly: Vec<i64>,
}

pub struct KeyGenerator;

impl KeyGenerator {
    pub fn new() -> Self {
        KeyGenerator
    }

    pub fn generate_keys(&self) -> (PublicKey, SecretKey) {
        let mut rng = rand::thread_rng();

        // Generate secret key (random polynomial)
        let sec_key_poly: Vec<i64> = (0..10).map(|_| rng.gen_range(1..100)).collect();
        let sec_key = SecretKey { poly: sec_key_poly.clone() };

        // Generate public key based on the secret key
        let random_poly: Vec<i64> = (0..10).map(|_| rng.gen_range(1..100)).collect();
        let noise_poly: Vec<i64> = (0..10).map(|_| rng.gen_range(-10..10)).collect();

        // Public key polynomials
        let pk_0: Vec<i64> = sec_key_poly.iter().zip(&random_poly)
            .map(|(&sk, &r)| -sk * r + rng.gen_range(-10..10))  // pk_0 = -sk * random + noise
            .collect();
        let pk_1: Vec<i64> = random_poly;  // pk_1 = random

        let pub_key = PublicKey { pk_0, pk_1 };

        (pub_key, sec_key)
    }
}
