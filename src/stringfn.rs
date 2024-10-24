use crate::polynomial::Polynomial;
use crate::ckks::CKKSEncryptor;

impl CKKSEncryptor {
    /// Function to calculate the homomorphic length of an encrypted string.
    /// This is the number of non-zero coefficients in the encrypted polynomial.
    pub fn homomorphic_length(&self, encrypted_poly: &Polynomial) -> usize {
        // Filter out zero coefficients and count the remaining non-zero ones
        let non_zero_coeffs = encrypted_poly.coeffs.iter().filter(|&&coeff| coeff != 0).count();
        
        // Return the number of non-zero coefficients
        non_zero_coeffs
    }
}
