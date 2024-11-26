use ckks_engine::*;
use log::info;


pub fn run_ckks_arithmetic_operations() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // Set CKKS parameters: degree of polynomial (N = 2048) and prime modulus (q)
    let params = CkksParameters::new(2048, 1000000000000007);

    // Key generation
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize CKKS encryptor and decryptor
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Declare two integer arrays and two float arrays
    let int_array1 = [100, 202, 304];
    let int_array2 = [400, 150, 210];

    let float_array1 = [4.32, 5.26, 3.78];
    let float_array2 = [2.11, 6.55, 1.99];

    // Declare scalar values
    let scalar_int1 = 10;
    let scalar_int2 = 20;

    let scalar_float1 = 4.56;
    let scalar_float2 = 7.89;

    // Encrypt integer arrays and float arrays
    info!("\n=== Encrypting Integer and Float Arrays ===");
    let encrypted_int_array1 = encryptor.encrypt_collection(&int_array1);
    let encrypted_int_array2 = encryptor.encrypt_collection(&int_array2);

    let encrypted_float_array1 = encryptor.encrypt_collection(&float_array1);
    let encrypted_float_array2 = encryptor.encrypt_collection(&float_array2);

    // Encrypt scalar integers and scalar floats
    info!("\n=== Encrypting Scalar Integers and Floats ===");
    let encrypted_scalar_int1 = encryptor.encrypt_value(scalar_int1);
    let encrypted_scalar_int2 = encryptor.encrypt_value(scalar_int2);

    let encrypted_scalar_float1 = encryptor.encrypt_value(scalar_float1);
    let encrypted_scalar_float2 = encryptor.encrypt_value(scalar_float2);

    // Homomorphic Operations on Arrays
    info!("\n=== Homomorphic Operations on Arrays ===");

    // Integer Array vs Integer Array
    let int_array_add = encryptor.homomorphic_add(&encrypted_int_array1, &encrypted_int_array2);
    let int_array_subtract = encryptor.homomorphic_subtract(&encrypted_int_array1, &encrypted_int_array2);
    let int_array_multiply = encryptor.homomorphic_multiply(&encrypted_int_array1, &encrypted_int_array2);
    let int_array_negate = encryptor.homomorphic_negation(&encrypted_int_array1);

    let decrypted_int_array_add = decryptor.decrypt(&int_array_add);
    let decrypted_int_array_subtract = decryptor.decrypt(&int_array_subtract);
    let decrypted_int_array_multiply = decryptor.decrypt(&int_array_multiply);
    let decrypted_int_array_negate = decryptor.decrypt(&int_array_negate);

    info!("Addition (int array + int array): {:?}", decrypted_int_array_add);
    info!("Subtraction (int array - int array): {:?}", decrypted_int_array_subtract);
    info!("Multiplication (int array * int array): {:?}", decrypted_int_array_multiply);
    info!("Negation (int array): {:?}", decrypted_int_array_negate);

    // Float Array vs Float Array
    let float_array_add = encryptor.homomorphic_add(&encrypted_float_array1, &encrypted_float_array2);
    let float_array_subtract = encryptor.homomorphic_subtract(&encrypted_float_array1, &encrypted_float_array2);
    let float_array_multiply = encryptor.homomorphic_multiply(&encrypted_float_array1, &encrypted_float_array2);
    let float_array_negate = encryptor.homomorphic_negation(&encrypted_float_array1);

    let decrypted_float_array_add = decryptor.decrypt(&float_array_add);
    let decrypted_float_array_subtract = decryptor.decrypt(&float_array_subtract);
    let decrypted_float_array_multiply = decryptor.decrypt(&float_array_multiply);
    let decrypted_float_array_negate = decryptor.decrypt(&float_array_negate);

    info!("Addition (float array + float array): {:?}", decrypted_float_array_add);
    info!("Subtraction (float array - float array): {:?}", decrypted_float_array_subtract);
    info!("Multiplication (float array * float array): {:?}", decrypted_float_array_multiply);
    info!("Negation (float array): {:?}", decrypted_float_array_negate);

    // Integer Array vs Float Array
    let int_float_array_add = encryptor.homomorphic_add(&encrypted_int_array1, &encrypted_float_array1);
    let int_float_array_subtract = encryptor.homomorphic_subtract(&encrypted_int_array1, &encrypted_float_array1);
    let int_float_array_multiply = encryptor.homomorphic_multiply(&encrypted_int_array1, &encrypted_float_array1);

    let decrypted_int_float_array_add = decryptor.decrypt(&int_float_array_add);
    let decrypted_int_float_array_subtract = decryptor.decrypt(&int_float_array_subtract);
    let decrypted_int_float_array_multiply = decryptor.decrypt(&int_float_array_multiply);

    info!("Addition (int array + float array): {:?}", decrypted_int_float_array_add);
    info!("Subtraction (int array - float array): {:?}", decrypted_int_float_array_subtract);
    info!("Multiplication (int array * float array): {:?}", decrypted_int_float_array_multiply);

    // Homomorphic Operations on Scalars
    info!("\n=== Homomorphic Operations on Scalars ===");

    // Integer vs Integer
    let scalar_int_add = encryptor.homomorphic_add(&encrypted_scalar_int1, &encrypted_scalar_int2);
    let scalar_int_subtract = encryptor.homomorphic_subtract(&encrypted_scalar_int1, &encrypted_scalar_int2);
    let scalar_int_multiply = encryptor.homomorphic_multiply(&encrypted_scalar_int1, &encrypted_scalar_int2);
    let scalar_int_negate = encryptor.homomorphic_negation(&encrypted_scalar_int1);

    let decrypted_scalar_int_add = decryptor.decrypt(&scalar_int_add);
    let decrypted_scalar_int_subtract = decryptor.decrypt(&scalar_int_subtract);
    let decrypted_scalar_int_multiply = decryptor.decrypt(&scalar_int_multiply);
    let decrypted_scalar_int_negate = decryptor.decrypt(&scalar_int_negate);

    info!("Addition (int + int): {:?}", decrypted_scalar_int_add);
    info!("Subtraction (int - int): {:?}", decrypted_scalar_int_subtract);
    info!("Multiplication (int * int): {:?}", decrypted_scalar_int_multiply);
    info!("Negation (int): {:?}", decrypted_scalar_int_negate);

    // Float vs Float
    let scalar_float_add = encryptor.homomorphic_add(&encrypted_scalar_float1, &encrypted_scalar_float2);
    let scalar_float_subtract = encryptor.homomorphic_subtract(&encrypted_scalar_float1, &encrypted_scalar_float2);
    let scalar_float_multiply = encryptor.homomorphic_multiply(&encrypted_scalar_float1, &encrypted_scalar_float2);
    let scalar_float_negate = encryptor.homomorphic_negation(&encrypted_scalar_float1);

    let decrypted_scalar_float_add = decryptor.decrypt(&scalar_float_add);
    let decrypted_scalar_float_subtract = decryptor.decrypt(&scalar_float_subtract);
    let decrypted_scalar_float_multiply = decryptor.decrypt(&scalar_float_multiply);
    let decrypted_scalar_float_negate = decryptor.decrypt(&scalar_float_negate);

    info!("Addition (float + float): {:?}", decrypted_scalar_float_add);
    info!("Subtraction (float - float): {:?}", decrypted_scalar_float_subtract);
    info!("Multiplication (float * float): {:?}", decrypted_scalar_float_multiply);
    info!("Negation (float): {:?}", decrypted_scalar_float_negate);

    // Integer vs Float
    let scalar_int_float_add = encryptor.homomorphic_add(&encrypted_scalar_int1, &encrypted_scalar_float1);
    let scalar_int_float_subtract = encryptor.homomorphic_subtract(&encrypted_scalar_int1, &encrypted_scalar_float1);
    let scalar_int_float_multiply = encryptor.homomorphic_multiply(&encrypted_scalar_int1, &encrypted_scalar_float1);

    let decrypted_scalar_int_float_add = decryptor.decrypt(&scalar_int_float_add);
    let decrypted_scalar_int_float_subtract = decryptor.decrypt(&scalar_int_float_subtract);
    let decrypted_scalar_int_float_multiply = decryptor.decrypt(&scalar_int_float_multiply);

    info!("Addition (int + float): {:?}", decrypted_scalar_int_float_add);
    info!("Subtraction (int - float): {:?}", decrypted_scalar_int_float_subtract);
    info!("Multiplication (int * float): {:?}", decrypted_scalar_int_float_multiply);

   
}

fn main(){
    run_ckks_arithmetic_operations();
}