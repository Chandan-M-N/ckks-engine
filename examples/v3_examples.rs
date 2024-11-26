use ckks_engine::*;
use log::info;

// Helper function to log homomorphic operation results
fn log_homomorphic_operation_results(
    operation: &str,
    decrypted_values: &Vec<f64>,
    label: &str,
) {
    info!("\n========================================");
    info!("Decrypted {} values for {}: {:?}", operation, label, decrypted_values);
}

fn test_homomorphic_exponentiation() {
    // Initialize parameters with a higher polynomial degree
    let degree = 2048;
    let params = CkksParameters::new(degree, 1000000000000007);

    // Initialize key generator
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize encryptor and decryptor
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Declare a float array and a scalar float value
    let float_array = [2.3, 3.14, 4.5];
    let scalar_float = 3.14;

    // Encrypt float array and scalar float
    info!("\n=== Encrypting Float Array and Scalar for Exponentiation ===");
    let encrypted_float_array = encryptor.encrypt_collection(&float_array);
    let encrypted_scalar_float = encryptor.encrypt_value(scalar_float);

    // Define the exponent
    let exponent = 3;

    // Perform exponentiation on encrypted float array
    let encrypted_exp_float_array = encryptor.homomorphic_exponentiation(&encrypted_float_array, exponent);
    let decrypted_exp_float_array = decryptor.decrypt(&encrypted_exp_float_array);
    log_homomorphic_operation_results(
        "exponentiation",
        &decrypted_exp_float_array,
        &format!("float array ^ {}", exponent)
    );

    // Perform exponentiation on encrypted scalar float
    let encrypted_exp_scalar_float = encryptor.homomorphic_exponentiation(&encrypted_scalar_float, exponent);
    let decrypted_exp_scalar_float = decryptor.decrypt(&encrypted_exp_scalar_float);
    log_homomorphic_operation_results(
        "exponentiation",
        &decrypted_exp_scalar_float,
        &format!("scalar float ^ {}", exponent)
    );

    info!("\n=== Exponentiation Operations Completed ===");
}

fn test_homomorphic_divide() {
    // Initialize parameters with a higher polynomial degree
    let degree = 2048;
    let params = CkksParameters::new(degree, 1000000000000007);

    // Initialize key generator
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize encryptor and decryptor
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Encrypt numerator and denominator
    let numerator = 500;
    let denominator = 2;
    let encrypted_numerator = encryptor.encrypt_value(numerator);
    let encrypted_denominator = encryptor.encrypt_value(denominator);

    // Perform division with 5 iterations for reciprocal approximation
    let encrypted_division = encryptor.homomorphic_divide(&encrypted_numerator, &encrypted_denominator);

    // Decrypt the result as integers
    let decrypted_division = decryptor.decrypt(&encrypted_division);
    info!("Decrypted division result: {:?}", decrypted_division);
}

pub fn adv_string_operations(){
    // Set CKKS parameters: degree of polynomial (N = 2048) and prime modulus (q)
   let params = CkksParameters::new(2048, 1000000000000007);

   // Key generation
   let keygen = KeyGenerator::new();
   let (public_key, secret_key) = keygen.generate_keys();

   // Initialize CKKS encryptor and decryptor
   let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
   let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

   // Define some strings to encrypt and decrypt
   let string1 = "Hello, CKKS. This is ";
   let string2 = "Homomorphic Encryption of Strings";

   // Encrypt the strings
   info!("\n=== Encrypting Strings ===");
   let encrypted_string1 = encryptor.encrypt_string(string1);
   let encrypted_string2 = encryptor.encrypt_string(string2);

   let concatenate = encryptor.concatenate_encrypted_strings(&encrypted_string1,&encrypted_string2);

   let decrypted_string1 = decryptor.decrypt_string(&concatenate);

   info!("Decrypted Concatenated String : {:?}", decrypted_string1);

   let encrypted_substring = encryptor.extract_encrypted_substring(&encrypted_string1, ..3);

   let decrypted_string2 = decryptor.decrypt_string(&encrypted_substring);
   info!("Decrypted Sub String : {:?}", decrypted_string2);



}

fn main(){
     // Initialize the logger
     env_logger::Builder::from_default_env()
     .filter_level(log::LevelFilter::Info)
     .init();
 
 // Run the homomorphic exponentiation test
 test_homomorphic_exponentiation();
 test_homomorphic_divide();
 adv_string_operations();
}
