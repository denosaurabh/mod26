/*===================================================================
 * Simple Implementation of RSA system
 * Author: denosauabh
 * Reference: https://en.wikipedia.org/wiki/RSA_(cryptosystem)
 *===================================================================*/


use std::convert::TryFrom;

use crate::utils::mod_arithmetic::ModArithmetic;

pub struct RSA {
    pub n: u128,
    pub e: u128,
    d: u128,
}

impl RSA {
    pub fn new(p: u128, q: u128) -> Self {
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        let e = Self::generate_e(phi);
        let d = ModArithmetic::mod_inverse_i128(e as i128, phi as i128).expect("Failed to calculate d") as u128;

        RSA { n, e, d }
    }

    pub fn encrypt(&self, message: u128) -> u128 {
        ModArithmetic::pow_u128(message, self.e, self.n)
    }

    pub fn decrypt(&self, ciphertext: u128) -> u128 {
        ModArithmetic::pow_u128(ciphertext, self.d, self.n)
    }

    fn generate_e(phi: u128) -> u128 {
        // For simplicity, we'll use 65537 as our public exponent
        // This is a common choice in many RSA implementations
        let e = 65537;
        assert!(ModArithmetic::euclidean_gcd_u128(e, phi) == 1, "65537 is not coprime with phi");
        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa() {
        // In practice, use much larger primes
        let p: u128 = 61;
        let q: u128 = 53;

        let rsa = RSA::new(p, q);

        let message: u128 = 2022;
        let encrypted = rsa.encrypt(message);
        let decrypted = rsa.decrypt(encrypted);

        assert_eq!(message, decrypted);
    }
}
