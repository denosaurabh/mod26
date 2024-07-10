/*===================================================================
 * Implementation of Pohlig-Hellman Exponention Cipher
 * Author: denosauabh
 *===================================================================*/


use primes::{PrimeSet, Sieve};
use crate::utils::{char_set::{CharSet, Converter}, mod_arithmetic::ModArithmetic};

pub struct PohligHellman {
    char_set: CharSet,
    n: u64,
}

impl PohligHellman {
    pub fn new(char_set: CharSet) -> Result<Self, String> {
        let n = Self::calculate_n(&char_set)?;
        Ok(Self { char_set, n })
    }

    fn calculate_n(char_set: &CharSet) -> Result<u64, String> {
        let n = char_set.len().to_string().repeat(2).parse::<u64>()
            .map_err(|e| format!("Failed to parse n: {}", e))?;
        Ok(Self::next_prime(n))
    }

    fn next_prime(n: u64) -> u64 {
        Sieve::new().find(n).1
    }

    /// Encrypt the given text using the provided key.
    pub fn encrypt(&self, text: &str, key: u64) -> Result<String, String> {
        let mut encrypted = String::new();

        let convertor = Converter::new(self.char_set.clone());
        let num_vec = convertor.convert_to_numvec(text)?;

        for p in num_vec.iter() {
            let c = ModArithmetic::pow(*p as u64, key, self.n);
            encrypted.push_str(&format!("{:0width$}", c, width = convertor.pad_length * 2));
        }

        Ok(encrypted)
    }

    // Decrypts the given text using the provided key.
    pub fn decrypt(&self, text: &str, key: u64) -> Result<String, String> {
        let charset_len = self.char_set.len();
        let pad_length = charset_len.to_string().len();

        let d = ModArithmetic::pow_inverse(key as u128, (self.n - 1) as u128)? as u64;

        let num_vec = text.chars()
            .collect::<Vec<char>>()
            .chunks(pad_length * 2)
            .map(|chunk| {
               let c = chunk.iter().collect::<String>().parse::<u64>().unwrap();
               ModArithmetic::pow(c, d, self.n) as u32
            })
            .collect::<Vec<u32>>();

        let decrypted = Converter::new(self.char_set.clone()).numvec_to_string(num_vec);

        Ok(decrypted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pohlig_hellman() -> Result<(), String> {
        let charset = CharSet::from_alphabet_lowercase();
        let pohlig_hellman = PohligHellman::new(charset)?;

        let text = "powertothepeople";
        let key = 769;

        // Encrypt
        let encrypted = pohlig_hellman.encrypt(text, key)?;
        println!("Encrypted: {}", encrypted);

        assert_eq!(encrypted, "10872142021919680818197307942378");

        // Decrypt
        let decrypted = pohlig_hellman.decrypt(&encrypted, key)?;
        println!("Decrypted: {}", decrypted);

        assert_eq!(decrypted, "powertothepeople");

        Ok(())
    }
}
