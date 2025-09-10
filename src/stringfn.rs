use crate::polynomial::Polynomial;
use crate::ckks::CKKSEncryptor;
use std::ops::RangeBounds;

impl CKKSEncryptor {
    /// Function to calculate the homomorphic length of an encrypted string.
    /// This is the number of non-zero coefficients in the encrypted polynomial.
    pub fn homomorphic_length(&self, encrypted_poly: &Polynomial) -> usize {
        // Filter out zero coefficients and count the remaining non-zero ones
        let non_zero_coeffs = encrypted_poly.coeffs.iter().filter(|&&coeff| coeff != 0).count();

        // Return the number of non-zero coefficients
        non_zero_coeffs
    }


    /// Function to concatenate two encrypted strings by combining their encrypted polynomials.
    /// This will create a new polynomial containing the coefficients of both encrypted strings.
        pub fn concatenate_encrypted_strings(&self, encrypted_poly1: &Polynomial, encrypted_poly2: &Polynomial) -> Polynomial {
        // Combine the coefficients of both encrypted polynomials
        let mut combined_coeffs = encrypted_poly1.coeffs.clone();
        combined_coeffs.extend_from_slice(&encrypted_poly2.coeffs);

        // Create and return the new concatenated polynomial
        Polynomial::new(combined_coeffs)
    }

    /// Extracts a substring from an encrypted string, using Rust range syntax.
    /// - `encrypted_poly`: The encrypted string as a polynomial.
    /// - `range`: A range representing the indices to extract (e.g., `0..3`, `3..`, `..5`).
    pub fn extract_encrypted_substring<R>(&self, encrypted_poly: &Polynomial, range: R) -> Polynomial
where
    R: RangeBounds<usize>,
{
    // Convert RangeBounds into a concrete Range<usize>
    let start = match range.start_bound() {
        std::ops::Bound::Included(&s) => s,
        std::ops::Bound::Excluded(&s) => s + 1,
        std::ops::Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        std::ops::Bound::Included(&e) => e + 1,
        std::ops::Bound::Excluded(&e) => e,
        std::ops::Bound::Unbounded => encrypted_poly.coeffs.len(),
    };

    // Ensure start and end are within bounds
    let start = start.clamp(0, encrypted_poly.coeffs.len());
    let end = end.clamp(0, encrypted_poly.coeffs.len());

    if start >= end {
        // If the range is invalid (start >= end), return an empty polynomial
        return Polynomial::new(vec![]);
    }

    // Apply the range to get the substring coefficients
    let substring_coeffs = encrypted_poly.coeffs[start..end].to_vec();

    // Return a new polynomial with the substring coefficients
    Polynomial::new(substring_coeffs)
}

}
