#![no_main]
use libfuzzer_sys::fuzz_target;
use ckks_engine::*;

// Helper to generate random parameters and keys
fn setup_ckks() -> (CKKSEncryptor, CKKSDecryptor) {
    let params = CkksParameters::new(2048, 1000000000000007);
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    let encryptor = CKKSEncryptor::new(public_key, params.clone());
    let decryptor = CKKSDecryptor::new(secret_key, params);

    (encryptor, decryptor)
}

// Helper function to generate a random string from bytes
fn generate_random_string(data: &[u8], max_length: usize) -> String {
    let length = std::cmp::min(data.len(), max_length);
    data.iter()
        .take(length)
        .map(|&b| (b % 94 + 32) as char) // Maps to printable ASCII characters
        .collect()
}

// Fuzz logic for individual functions
fn fuzz_encrypt_collection(encryptor: &CKKSEncryptor, data: &[f64]) {
    let encrypted_data = encryptor.encrypt_collection(data);
    assert!(!encrypted_data.coeffs.is_empty());
}

fn fuzz_encrypt_value(encryptor: &CKKSEncryptor, value: f64) {
    let encrypted_value = encryptor.encrypt_value(value);
    assert!(encrypted_value.coeffs.len() > 0);
}

fn fuzz_homomorphic_operations(
    encryptor: &CKKSEncryptor,
    decryptor: &CKKSDecryptor,
    data1: &[f64],
    value: f64,
) {
    let enc_collection1 = encryptor.encrypt_collection(data1);

    let enc_value = encryptor.encrypt_value(value);

    // Test homomorphic_add with all combinations
    let add_results = [
        encryptor.homomorphic_add(&enc_collection1, &enc_value), // Collection + Value
        encryptor.homomorphic_add(&enc_value, &enc_collection1), // Value + Collection
        encryptor.homomorphic_add(&enc_collection1, &enc_collection1), // Collection + Collection
    ];
    for (i, result) in add_results.iter().enumerate() {
        let decrypted_add = decryptor.decrypt(result);
        assert_eq!(decrypted_add.len(), data1.len());
    }

    // Test homomorphic_subtract with all combinations
    let sub_results = [
        encryptor.homomorphic_subtract(&enc_collection1, &enc_value),
        encryptor.homomorphic_subtract(&enc_value, &enc_collection1),
        encryptor.homomorphic_subtract(&enc_collection1, &enc_collection1),
    ];
    for (i, result) in sub_results.iter().enumerate() {
        let decrypted_sub = decryptor.decrypt(result);
        assert_eq!(decrypted_sub.len(), data1.len());
    }

    // Test homomorphic_multiply with all combinations
    let mul_results = [
        encryptor.homomorphic_multiply(&enc_collection1, &enc_value),
        encryptor.homomorphic_multiply(&enc_value, &enc_collection1),
        encryptor.homomorphic_multiply(&enc_collection1, &enc_collection1),
    ];
    for (i, result) in mul_results.iter().enumerate() {
        let decrypted_mul = decryptor.decrypt(result);
        assert_eq!(decrypted_mul.len(), data1.len());
    }

    // Test homomorphic_divide with all combinations
    let div_results = [
    encryptor.homomorphic_divide(&enc_collection1, &enc_value),
    encryptor.homomorphic_divide(&enc_value, &enc_collection1),
    encryptor.homomorphic_divide(&enc_collection1, &enc_collection1),
    ];

    for (i, result) in div_results.iter().enumerate() {
        let decrypted_div = decryptor.decrypt(result);

        // Skip assertion if the decrypted result is empty (indicating a division by zero)
        if decrypted_div.is_empty() {
            println!("Skipping assertion for index {} due to division by zero.", i);
            continue;
        }

        assert_eq!(decrypted_div.len(), data1.len());
    }
}

fn fuzz_string_operations(
    encryptor: &CKKSEncryptor,
    decryptor: &CKKSDecryptor,
    string1: &str,
    string2: &str,
) {
    let encrypted_string1 = encryptor.encrypt_string(string1);
    let encrypted_string2 = encryptor.encrypt_string(string2);

    let concat_result = encryptor.concatenate_encrypted_strings(&encrypted_string1, &encrypted_string2);
    let decrypted_concat = decryptor.decrypt_string(&concat_result);
    assert!(decrypted_concat.contains(string1) && decrypted_concat.contains(string2));

    let substring_result = encryptor.extract_encrypted_substring(&encrypted_string1, 0..3);
    let decrypted_substring = decryptor.decrypt_string(&substring_result);
    assert!(string1.starts_with(&decrypted_substring));
}

fn fuzz_advanced_operations(
    encryptor: &CKKSEncryptor,
    decryptor: &CKKSDecryptor,
    data: &[f64],
    value: f64,
) {
    let encrypted_data = encryptor.encrypt_collection(data);
    let encrypted_value = encryptor.encrypt_value(value);

    let ceil_result = encryptor.homomorphic_ceil(&encrypted_data);
    let decrypted_ceil = decryptor.decrypt(&ceil_result);
    assert_eq!(decrypted_ceil.len(), data.len());

    let floor_result = encryptor.homomorphic_floor(&encrypted_data);
    let decrypted_floor = decryptor.decrypt(&floor_result);
    assert_eq!(decrypted_floor.len(), data.len());

    let round_result = encryptor.homomorphic_round(&encrypted_data);
    let decrypted_round = decryptor.decrypt(&round_result);
    assert_eq!(decrypted_round.len(), data.len());

    let truncate_result = encryptor.homomorphic_truncate(&encrypted_data);
    let decrypted_truncate = decryptor.decrypt_as_int(&truncate_result);
    assert_eq!(decrypted_truncate.len(), data.len());

    let exp_result = encryptor.homomorphic_exponentiation(&encrypted_data, 3);
    let decrypted_exp = decryptor.decrypt(&exp_result);
    assert_eq!(decrypted_exp.len(), data.len());


}

fuzz_target!(|data: &[u8]| {
    if data.len() < 20 {
        return; // Ensure sufficient data for fuzzing
    }

    let (encryptor, decryptor) = setup_ckks();

    // Generate random inputs for fuzzing
    let float_array1: Vec<f64> = data.iter().take(10).map(|&x| x as f64 / 10.0).collect();
    let scalar = data[0] as f64 / 10.0;

    // Generate random strings for string-related functions
    let string1 = generate_random_string(data, 20);
    let string2 = generate_random_string(&data[10..], 20);

    // Execute fuzzing for various functions
    fuzz_encrypt_collection(&encryptor, &float_array1);
    fuzz_encrypt_value(&encryptor, scalar);
    fuzz_homomorphic_operations(&encryptor, &decryptor, &float_array1, scalar);
    fuzz_string_operations(&encryptor, &decryptor, &string1, &string2);
    fuzz_advanced_operations(&encryptor, &decryptor, &float_array1, scalar);
});
