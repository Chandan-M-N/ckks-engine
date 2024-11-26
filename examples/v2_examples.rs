use ckks_engine::*;
use log::info;


pub fn run_ckks_float_operations(){
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

    let float_array1 = [3.6, 5.26, 3.78];
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

     // Homomorphic ceil operation
   info!("\n=== Homomorphic Ceil Operations ===");

   // Perform ceil operation on the float array1
   let encrypted_ceil_float_array1 = encryptor.homomorphic_ceil(&encrypted_float_array1);
   let decrypted_ceil_float_array1 = decryptor.decrypt(&encrypted_ceil_float_array1);
   info!("Decrypted ceil values for float array 1: {:?}", decrypted_ceil_float_array1);

   // Perform ceil operation on the float array2
   let encrypted_ceil_float_array2 = encryptor.homomorphic_ceil(&encrypted_float_array2);
   let decrypted_ceil_float_array2 = decryptor.decrypt(&encrypted_ceil_float_array2);
   info!("Decrypted ceil values for float array 2: {:?}", decrypted_ceil_float_array2);

   // Perform ceil operation on the scalar float1
   let encrypted_ceil_scalar_float1 = encryptor.homomorphic_ceil(&encrypted_scalar_float1);
   let decrypted_ceil_scalar_float1 = decryptor.decrypt(&encrypted_ceil_scalar_float1);
   info!("Decrypted ceil value for scalar float 1: {:?}", decrypted_ceil_scalar_float1);

   // Perform ceil operation on the scalar float2
   let encrypted_ceil_scalar_float2 = encryptor.homomorphic_ceil(&encrypted_scalar_float2);
   let decrypted_ceil_scalar_float2 = decryptor.decrypt(&encrypted_ceil_scalar_float2);
   info!("Decrypted ceil value for scalar float 2: {:?}", decrypted_ceil_scalar_float2);

   // Homomorphic floor operation

   info!("\n=== Homomorphic Floor Operations ===");

   // Perform floor operation on the float array1
   let encrypted_floor_float_array1 = encryptor.homomorphic_floor(&encrypted_float_array1);
   let decrypted_floor_float_array1 = decryptor.decrypt(&encrypted_floor_float_array1);
   info!("Decrypted floor values for float array 1: {:?}", decrypted_floor_float_array1);

   // Perform floor operation on the float array2
   let encrypted_floor_float_array2 = encryptor.homomorphic_floor(&encrypted_float_array2);
   let decrypted_floor_float_array2 = decryptor.decrypt(&encrypted_floor_float_array2);
   info!("Decrypted floor values for float array 2: {:?}", decrypted_floor_float_array2);
   info!("\n=== Homomorphic Round and Truncate Operations ===");

   // Perform floor operation on the scalar float1
   let encrypted_floor_scalar_float1 = encryptor.homomorphic_floor(&encrypted_scalar_float1);
   let decrypted_floor_scalar_float1 = decryptor.decrypt(&encrypted_floor_scalar_float1);
   info!("Decrypted floor value for scalar float 1: {:?}", decrypted_floor_scalar_float1);

   // Perform floor operation on the scalar float2
   let encrypted_floor_scalar_float2 = encryptor.homomorphic_floor(&encrypted_scalar_float2);
   let decrypted_floor_scalar_float2 = decryptor.decrypt(&encrypted_floor_scalar_float2);
   info!("Decrypted floor value for scalar float 2: {:?}", decrypted_floor_scalar_float2);

   info!("\n=== Homomorphic Round and Truncate Operations ===");

   //Perform round operation on float arr1
   let encrypted_round_float_array1 = encryptor.homomorphic_round(&encrypted_float_array1);
   let decrypted_round_float_array1 = decryptor.decrypt(&encrypted_round_float_array1);
   info!("Decrypted round values for float array 1: {:?}", decrypted_round_float_array1);


   // Perform truncate operation on the float array1
   let encrypted_truncate_float_array1 = encryptor.homomorphic_truncate(&encrypted_float_array1);
   let decrypted_truncate_float_array1 = decryptor.decrypt_as_int(&encrypted_truncate_float_array1);
   info!("Decrypted truncate values for float array 1: {:?}", decrypted_truncate_float_array1);

   // Round on float array2
   let encrypted_round_float_array2 = encryptor.homomorphic_round(&encrypted_float_array2);
   let decrypted_round_float_array2 = decryptor.decrypt_as_int(&encrypted_round_float_array2);
   info!("Decrypted round values for float array 2: {:?}", decrypted_round_float_array2);

   // Truncate on float array2
   let encrypted_truncate_float_array2 = encryptor.homomorphic_truncate(&encrypted_float_array2);
   let decrypted_truncate_float_array2 = decryptor.decrypt_as_int(&encrypted_truncate_float_array2);
   info!("Decrypted truncate values for float array 2: {:?}", decrypted_truncate_float_array2);

   // Round on scalar floats
   let encrypted_round_scalar_float1 = encryptor.homomorphic_round(&encrypted_scalar_float1);
   let decrypted_round_scalar_float1 = decryptor.decrypt_as_int(&encrypted_round_scalar_float1);
   info!("Decrypted round value for scalar float 1: {:?}", decrypted_round_scalar_float1);

   // Truncate on scalar floats
   let encrypted_truncate_scalar_float1 = encryptor.homomorphic_truncate(&encrypted_scalar_float1);
   let decrypted_truncate_scalar_float1 = decryptor.decrypt_as_int(&encrypted_truncate_scalar_float1);
   info!("Decrypted truncate value for scalar float 1: {:?}", decrypted_truncate_scalar_float1);


   let encrypted_round_scalar_float2 = encryptor.homomorphic_round(&encrypted_scalar_float2);
   let decrypted_round_scalar_float2 = decryptor.decrypt_as_int(&encrypted_round_scalar_float2);
   info!("Decrypted round value for scalar float 2: {:?}", decrypted_round_scalar_float2);


   let encrypted_truncate_scalar_float2 = encryptor.homomorphic_truncate(&encrypted_scalar_float2);
   let decrypted_truncate_scalar_float2 = decryptor.decrypt_as_int(&encrypted_truncate_scalar_float2);
   info!("Decrypted truncate value for scalar float 2: {:?}", decrypted_truncate_scalar_float2);


   info!("\n=== All operations completed ===");

}

pub fn run_ckks_string_operations() {
   // Set CKKS parameters: degree of polynomial (N = 2048) and prime modulus (q)
   let params = CkksParameters::new(2048, 1000000000000007);

   // Key generation
   let keygen = KeyGenerator::new();
   let (public_key, secret_key) = keygen.generate_keys();

   // Initialize CKKS encryptor and decryptor
   let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
   let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

   // Define some strings to encrypt and decrypt
   let string1 = "Hello, CKKS. This is the string encryption buddy";
   let string2 = "Homomorphic Encryption";

   // Encrypt the strings
   info!("\n=== Encrypting Strings ===");
   let encrypted_string1 = encryptor.encrypt_string(string1);
   let encrypted_string2 = encryptor.encrypt_string(string2);

   info!("Encrypted String 1: {:?}", encrypted_string1);
   info!("Encrypted String 2: {:?}", encrypted_string2);

   // Test and log the homomorphic lengths of the encrypted strings
   let length1 = encryptor.homomorphic_length(&encrypted_string1);
   let length2 = encryptor.homomorphic_length(&encrypted_string2);

   info!("Homomorphic length of Encrypted String 1: {}", length1);
   info!("Homomorphic length of Encrypted String 2: {}", length2);

   // Decrypt the strings
   info!("\n=== Decrypting Strings ===");
   let decrypted_string1 = decryptor.decrypt_string(&encrypted_string1);
   let decrypted_string2 = decryptor.decrypt_string(&encrypted_string2);

   info!("Decrypted String 1: {:?}", decrypted_string1);
   info!("Decrypted String 2: {:?}", decrypted_string2);

   // Verify that the decrypted strings match the originals
   assert_eq!(string1, decrypted_string1);
   assert_eq!(string2, decrypted_string2);

   info!("\n=== String encryption, decryption, and length operations completed ===");
}

fn main(){
    run_ckks_float_operations();
    run_ckks_string_operations();
}