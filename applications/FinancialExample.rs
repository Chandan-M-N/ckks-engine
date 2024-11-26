use ckks_engine::utils::{encode, decode, mod_reduce};
use ckks_engine::{CKKSEncryptor, CKKSDecryptor, CkksParameters, Polynomial};

fn main() {
    // Initialize CKKS parameters
    let params = CkksParameters::new(4096, 1000000000000007);
    let keygen = ckks_engine::KeyGenerator::new(); // No arguments required
    let (public_key, secret_key) = keygen.generate_keys();

    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    const SCALE: f64 = 1e7; // Scaling factor for encoding

    // Step 1: Define transactions (plaintext)
    let transactions = vec![500.69, -200.0, 300.0]; // Example transaction amounts
    let labels = vec!["Deposit: +500", "Withdrawal: -200", "Deposit: +300"]; // Corresponding labels

    // Step 2: Encrypt transaction amounts
    let encrypted_transactions: Vec<Polynomial> = transactions
        .iter()
        .map(|&value| encode(&[value], SCALE))
        .collect();

    // Step 3: Encrypt transaction labels
    let encrypted_labels: Vec<Polynomial> = labels
        .iter()
        .map(|label| {
            // Encode the label string as bytes and then encode as a polynomial
            let bytes = label.as_bytes();
            let float_vals: Vec<f64> = bytes.iter().map(|&b| b as f64).collect();
            encode(&float_vals, SCALE)
        })
        .collect();

    // Step 4: Compute the total balance using homomorphic addition
    let total_balance_encrypted = encrypted_transactions
        .iter()
        .cloned()
        .reduce(|acc, value| encryptor.homomorphic_add(&acc, &value))
        .unwrap();

    // Step 5: Round the total balance for reporting
    let rounded_balance_encrypted = encryptor.homomorphic_round(&total_balance_encrypted);

    // Step 6: Concatenate encrypted labels
    let concatenated_labels_encrypted = encrypted_labels
        .iter()
        .cloned()
        .reduce(|acc, label| encryptor.concatenate_encrypted_strings(&acc, &label))
        .unwrap();

    // Step 7: Decrypt and decode results
    let total_balance = decode(&total_balance_encrypted, SCALE);
    let rounded_balance = decode(&rounded_balance_encrypted, SCALE);

    // Decode concatenated labels back into plaintext
    let concatenated_labels_plaintext: String = concatenated_labels_encrypted
        .coeffs
        .iter()
        .map(|&c| (c as u8) as char)
        .collect();

    // Print the results
    println!("Total Balance: {:?}", total_balance);
    println!("Rounded Balance: {:?}", rounded_balance);
    println!("Transaction Labels: {:?}", concatenated_labels_plaintext);
}
