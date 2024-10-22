# ckks-engine

`ckks-engine` is a Rust crate that provides an implementation of the CKKS (Cheon-Kim-Kim-Song) homomorphic encryption scheme. This enables encrypted computations on real numbers and strings while preserving the privacy of the underlying data. With `ckks-engine`, you can perform a wide range of mathematical operations on encrypted data, including addition, subtraction, multiplication, and more.

## CKKS Scheme Overview

The CKKS scheme is a homomorphic encryption method designed to allow for computations on encrypted data with a certain level of error tolerance. It is particularly well-suited for applications that require the processing of real numbers while maintaining confidentiality. The CKKS scheme utilizes polynomial encoding to represent real numbers and supports operations directly on these encoded values.

### Version 1.0 Release

We are excited to announce the initial release of version 1.0 of `ckks-engine`. This version includes a complete implementation of the CKKS homomorphic encryption algorithm, along with essential features designed for secure and efficient encrypted computations. Key highlights of this release include:

- **CKKS Algorithm Implementation**: The complete implementation of the CKKS homomorphic encryption algorithm, allowing for secure encryption and computation of real numbers.

- **Encryption and Decryption Functions**: Secure functions to encrypt and decrypt data using the CKKS scheme, ensuring data privacy throughout the process.

- **Basic Arithmetic Operations**:
  - **Addition**: Perform encrypted addition, allowing for the summation of encrypted values without revealing the underlying data.
  - **Subtraction**: Execute encrypted subtraction to determine the difference between encrypted values.
  - **Multiplication**: Multiply encrypted values securely, enabling more complex computations while preserving privacy.
  - **Negation**: Compute the negation of encrypted values, providing additional flexibility in mathematical operations.

### Version 2.0 Release

We are excited to announce the initial release of version 2.0 of `ckks-engine`. This version includes additional features for enhanced encrypted computations. Key highlights of this release include:

- **Floating-Point Number Operations**:
  - **Ceil**: Compute the ceiling of encrypted floating-point numbers.
  - **Round**: Round encrypted floating-point numbers to the nearest integer.
  - **Floor**: Compute the floor of encrypted floating-point numbers.
  - **Truncate**: Remove the decimal part of encrypted floating-point numbers.

- **String Operations**:
  - **Character Encoding**: Convert characters into ASCII/Unicode values for encryption.
  - **Length Calculation**: Calculate the length of encrypted strings.

## Project Architecture

The architecture is modular, ensuring scalability and separation of concerns. Below is an overview of the project's architecture:

### Module-Level Structure

- **`lib.rs`**:
    The `lib.rs` file serves as the entry point for the `ckks-engine` crate. It declares and re-exports key modules, making it easier to access the core components of the crate.

- **`ckks.rs`**:
  - Contains the core CKKS encryption and decryption functions.
  - Manages the encryption and decryption of data, operating on polynomial representations.

- **`keygen.rs`**:
  - Handles the generation of public and secret keys.
  - Uses secure random polynomials for key creation.

- **`polynomial.rs`**:
  - Defines polynomial structures and operations used in encryption.
  - Supports polynomial arithmetic, including:
    - Addition
    - Subtraction
    - Multiplication
    - Negation
  - Polynomials form the backbone of the CKKS encryption method.

- **`utils.rs`**:
  - Provides utility functions for:
    - Encoding real numbers (both integers and floating-point values) into polynomials.
    - Decoding polynomials back into real numbers.
  - Manages rounding and scaling factors to ensure numerical precision in encrypted computations.

- **`arithmetic.rs`**:
  - Implements the main arithmetic functions for encrypted data:
    - Homomorphic addition, subtraction, multiplication, and negation.
  - Allows operations on both individual values and collections of encrypted values (arrays).

- **`stringfn.rs`**:
  - Contains functions for string operations, including length calculation for encrypted strings.


### Flow of Operations

The CKKS engine follows this flow for encryption and operations:

1. **Key Generation**:
   - Public and secret keys are generated using random polynomials in `keygen.rs`.

2. **Encoding**:
   - Real numbers are encoded into polynomials with scaling using functions in `utils.rs`.

3. **Encryption**:
   - Encoded polynomials are encrypted using the public key with the CKKS scheme in `ckks.rs`.

4. **Homomorphic Operations**:
   - Homomorphic operations are applied directly to ciphertexts.

5. **Decryption**:
   - The encrypted polynomials (ciphertexts) are decrypted using the secret key.

6. **Decoding**:
   - Decoded polynomials are converted back into real numbers, with precision control using rounding and thresholding.

### Inter-Module Interaction

The modules interact to form a complete system for secure, encrypted computation. The interaction flow is as follows:

- **Key generation** occurs in `keygen.rs`.
- **Data encoding** and **decoding** occur in `utils.rs`.
- **Encryption and decryption** processes are handled in `ckks.rs`.
- **Polynomial operations** are defined in `polynomial.rs`.
- **Homomorphic arithmetic and float** is performed in `arithmetic.rs`.
- **String operations**, including length calculation, are handled in `stringfn.rs`.

## Crate Dependencies

The `ckks-engine` crate relies on several external libraries to provide the necessary functionality. Below is a brief overview of each dependency:

- **ff (version 0.9)**: 
  - A library that provides finite field arithmetic, which is essential for performing operations in the CKKS scheme. This library allows the handling of mathematical operations over finite fields efficiently.

- **rand (version 0.8)**: 
  - A random number generation library used to create secure random values. In homomorphic encryption, randomness is crucial for key generation and ensuring the security of the encrypted data.

- **num-bigint (version 0.4)**: 
  - This library offers support for large integers, which can be particularly useful for modular arithmetic operations in the encryption scheme. It enables operations on integers larger than those supported by standard Rust integer types.

- **log (version 0.4)**: 
  - A logging framework that allows you to add logging capabilities to your application. It helps track the execution of the program and debug issues by logging relevant information.

- **env_logger (version 0.10)**: 
  - A logging implementation that reads log configuration from the environment. It allows you to control the logging level of your application via environment variables, making it easier to manage logging output in different environments.

### Installation

To include these dependencies in your own Rust project, add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
ff = "0.9"
rand = "0.8"
num-bigint = "0.4"
log = "0.4"
env_logger = "0.10"
```


## Usage

To test and utilize the features of `ckks-engine`, a sample code is provided in examples directory. This code demonstrates the functionality of the encryption scheme and basic homomorphic operations.

### Instructions to Run the Code

1. Clone the repository:
   ```bash
   git clone https://github.com/Chandan-M-N/ckks-engine.git
    ```
2. Navigate to the project directory:
    ```bash
    cd ckks-engine
    ```
3. Build the project using cargo:
    ```bash
    cargo build
    ```
4. Run the example code:

   v1_examples --> For arithmectic operations
   v2_examples --> For float and string operations

    ```bash
    cargo run --example v1_examples
    ```

This will execute the sample code, illustrating how to use the ckks-engine crate for encrypted computations.
