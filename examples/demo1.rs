use ckks_engine::*;

fn example1_operations() {
    println!("=== Example 1 Operations ===");

    // Set CKKS parameters
    let params = CkksParameters::new(2048, 1000000000000007);

    // Key generation
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize CKKS encryptor and decryptor
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Integer arrays
    let int_array1 = vec![121, 262, 384];
    let int_array2 = vec![98.3, 48.9, 36.6];

    println!("Original Integer Arrays: {:?}, {:?}", int_array1, int_array2);

    let encrypted_int_array1 = encryptor.encrypt_collection(&int_array1);
    let encrypted_int_array2 = encryptor.encrypt_collection(&int_array2);

    println!("\n=== Homomorphic Addition ===");
    let encrypted_add = encryptor.homomorphic_add(&encrypted_int_array1, &encrypted_int_array2);
    println!("Encrypted Addition Result: {:?}", encrypted_add);
    let decrypted_add = decryptor.decrypt(&encrypted_add);
    println!("Addition Result (Decrypted): {:?}", decrypted_add);

    println!("\n=== Homomorphic Subtraction ===");
    let encrypted_subtract = encryptor.homomorphic_subtract(&encrypted_int_array1, &encrypted_int_array2);
    println!("Encrypted Subtraction Result: {:?}", encrypted_subtract);
    let decrypted_subtract = decryptor.decrypt(&encrypted_subtract);
    println!("Subtraction Result (Decrypted): {:?}", decrypted_subtract);

    println!("\n=== Homomorphic Multiplication ===");
    let encrypted_multiply = encryptor.homomorphic_multiply(&encrypted_int_array1, &encrypted_int_array2);
    println!("Encrypted Multiplication Result: {:?}", encrypted_multiply);
    let decrypted_multiply = decryptor.decrypt(&encrypted_multiply);
    println!("Multiplication Result (Decrypted): {:?}", decrypted_multiply);

    println!("\n=== Homomorphic Negation ===");
    let int_array3 = vec![-98.3, 48.9, -36.6];
    println!("Original Integer Array 3: {:?}", int_array3);
    let encrypted_int_array3 = encryptor.encrypt_collection(&int_array3);
    let encrypted_negation = encryptor.homomorphic_negation(&encrypted_int_array3);
    println!("Encrypted Negation Result: {:?}", encrypted_negation);
    let decrypted_negation = decryptor.decrypt(&encrypted_negation);
    println!("Negation Result (Decrypted): {:?}", decrypted_negation);

    println!("\n=== Scalar Encryption and Operations ===");
    let scalar_value1 = 39;
    let scalar_value2 = 44;

    println!("Original Scalar 1: {:?}", scalar_value1);
    println!("Original Scalar 2: {:?}", scalar_value2);

    let encrypted_scalar1 = encryptor.encrypt_value(scalar_value1);
    let encrypted_scalar2 = encryptor.encrypt_value(scalar_value2);

    println!("Encrypted Scalar 1: {:?}", encrypted_scalar1);
    println!("Encrypted Scalar 2: {:?}", encrypted_scalar2);

    println!("\n=== Scalar Multiplication ===");
    let encrypted_scalar_multiply = encryptor.homomorphic_multiply(&encrypted_scalar1, &encrypted_scalar2);
    println!("Encrypted Scalar Multiplication Result: {:?}", encrypted_scalar_multiply);
    let decrypted_scalar_multiply = decryptor.decrypt(&encrypted_scalar_multiply);
    println!("Scalar Multiplication Result (Decrypted): {:?}", decrypted_scalar_multiply);
}

fn example2_operations() {
    println!("=== Example 2 Operations ===");

    // Set CKKS parameters
    let params = CkksParameters::new(2048, 1000000000000007);

    // Key generation
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize CKKS encryptor and decryptor
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Floating-point arrays
    let float_array1 = vec![3.6, 5.26, 3.78];
    let float_array2 = vec![2.11, 6.55, 1.99];

    println!("Original Float Arrays: {:?}, {:?}", float_array1, float_array2);

    let encrypted_float_array1 = encryptor.encrypt_collection(&float_array1);
    let encrypted_float_array2 = encryptor.encrypt_collection(&float_array2);

    println!("\n=== Homomorphic Ceil ===");
    let encrypted_ceil = encryptor.homomorphic_ceil(&encrypted_float_array1);
    println!("Encrypted Ceil Result: {:?}", encrypted_ceil);
    let decrypted_ceil = decryptor.decrypt(&encrypted_ceil);
    println!("Ceil Result (Decrypted): {:?}", decrypted_ceil);

    println!("\n=== Homomorphic Floor ===");
    let encrypted_floor = encryptor.homomorphic_floor(&encrypted_float_array2);
    println!("Encrypted Floor Result: {:?}", encrypted_floor);
    let decrypted_floor = decryptor.decrypt(&encrypted_floor);
    println!("Floor Result (Decrypted): {:?}", decrypted_floor);

    println!("\n=== Homomorphic Round ===");
    let encrypted_round = encryptor.homomorphic_round(&encrypted_float_array1);
    println!("Encrypted Round Result: {:?}", encrypted_round);
    let decrypted_round = decryptor.decrypt(&encrypted_round);
    println!("Round Result (Decrypted): {:?}", decrypted_round);

    println!("\n=== Homomorphic Truncate ===");
    let encrypted_truncate = encryptor.homomorphic_truncate(&encrypted_float_array1);
    println!("Encrypted Truncate Result: {:?}", encrypted_truncate);
    let decrypted_truncate = decryptor.decrypt_as_int(&encrypted_truncate);
    println!("Truncate Result (Decrypted): {:?}", decrypted_truncate);
}

fn example3_operations() {
    println!("=== Example 3 Operations ===");

    // Set CKKS parameters
    let params = CkksParameters::new(2048, 1000000000000007);

    // Key generation
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize CKKS encryptor and decryptor
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    println!("\n=== Exponentiation ===");
    let float_array = vec![2.3, 3.14, 4.5];
    let exponent = 2;

    println!("Original Float Array: {:?}", float_array);
    println!("Exponent value: {:?}", exponent);

    let encrypted_float_array = encryptor.encrypt_collection(&float_array);
    println!("Encrypted Float Array: {:?}", encrypted_float_array);

    let encrypted_exponentiation = encryptor.homomorphic_exponentiation(&encrypted_float_array, exponent);
    println!("Encrypted Exponentiation Result: {:?}", encrypted_exponentiation);

    let decrypted_exponentiation = decryptor.decrypt(&encrypted_exponentiation);
    println!("Exponentiation Result (Decrypted): {:?}", decrypted_exponentiation);

    println!("\n=== Division ===");
    let numerator = 378;
    let denominator = 23;

    let encrypted_numerator = encryptor.encrypt_value(numerator);
    let encrypted_denominator = encryptor.encrypt_value(denominator);

    println!("Encrypted Numerator: {:?}", encrypted_numerator);
    println!("Encrypted Denominator: {:?}", encrypted_denominator);

    let encrypted_division = encryptor.homomorphic_divide(&encrypted_numerator, &encrypted_denominator);
    println!("Encrypted Division Result: {:?}", encrypted_division);

    let decrypted_division = decryptor.decrypt(&encrypted_division);
    println!("Division Result (Decrypted): {:?}", decrypted_division);

    println!("\n=== String Operations ===");
    let string1 = "Hello, CKKS 👋";
    let string2 = "Encryption 🚀";

    let encrypted_string1 = encryptor.encrypt_string(string1);
    let encrypted_string2 = encryptor.encrypt_string(string2);

    println!("Encrypted String 1: {:?}", encrypted_string1);
    println!("Encrypted String 2: {:?}", encrypted_string2);

    let concatenated = encryptor.concatenate_encrypted_strings(&encrypted_string1, &encrypted_string2);
    println!("Encrypted Concatenated String: {:?}", concatenated);

    let decrypted_concatenated = decryptor.decrypt_string(&concatenated);
    println!("Concatenated String (Decrypted): {:?}", decrypted_concatenated);

    let substring = encryptor.extract_encrypted_substring(&encrypted_string1, 1..5);
    println!("Encrypted Substring: {:?}", substring);

    let decrypted_substring = decryptor.decrypt_string(&substring);
    println!("Substring (Decrypted): {:?}", decrypted_substring);

    let length1 = encryptor.homomorphic_length(&encrypted_string1);
    println!("Length of Encrypted String 1: {:?}", length1);
}

fn main() {
    example1_operations();
    example2_operations();
    example3_operations();
}
