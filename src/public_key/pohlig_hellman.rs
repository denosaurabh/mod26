/*===================================================================
 * Implementation of Pohlig-Hellman Exponention Cipher
 * Author: denosauabh
 *===================================================================*/


use primes::{PrimeSet, Sieve};
use crate::utils::{char_set::CharSet, mod_arithmetic::ModArithmetic};

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

    /// Calculate the modular multiplicative inverse of e modulo n.
    fn exponent_inverse(e: u64, n: u64) -> Result<u64, String> {
        let mut t = 0i128;
        let mut newt = 1i128;
        let mut r = n as i128;
        let mut newr = e as i128;

        while newr != 0 {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }

        if r > 1 {
            return Err("e is not invertible".to_string());
        }
        
        Ok(if t < 0 { t + n as i128 } else { t } as u64)
    }

    fn next_prime(n: u64) -> u64 {
        Sieve::new().find(n).1
    }

    /// Encrypt the given text using the provided key.
    pub fn encrypt(&self, text: &str, key: u64) -> Result<String, String> {
        let charset_len = self.char_set.len();
        let pad_length = charset_len.to_string().len();

        let mut encrypted = String::new();

        for chunk in text.chars().collect::<Vec<char>>().chunks(2) {
            let p = if chunk.len() == 2 {
                let a = self.char_set.index_of(chunk[0]);
                let b = self.char_set.index_of(chunk[1]);

                format!("{:0width$}{:0width$}", a, b, width = pad_length).parse::<u64>()
                    .map_err(|e| format!("Failed to parse p: {}", e))?
            } else {
                let a = self.char_set.index_of(chunk[0]);

                format!("{:0width$}", a, width = pad_length * 2).parse::<u64>()
                    .map_err(|e| format!("Failed to parse p: {}", e))?
            };

            let c = ModArithmetic::pow(p, key, self.n);

            encrypted.push_str(&format!("{:0width$}", c, width = pad_length * 2));
        }

        Ok(encrypted)
    }

    /// Decrypts the given text using the provided key.
    pub fn decrypt(&self, text: &str, key: u64) -> Result<String, String> {
        let charset_len = self.char_set.len();
        let pad_length = charset_len.to_string().len();

        let d = Self::exponent_inverse(key, self.n - 1)?;

        let mut decrypted = String::new();

        for chunk in text.chars().collect::<Vec<char>>().chunks(pad_length * 2) {
            let c = chunk.iter().collect::<String>().parse::<u64>()
                .map_err(|e| format!("Failed to parse c: {}", e))?;

            let p = ModArithmetic::pow(c, d, self.n);
            let p_str = format!("{:0width$}", p, width = pad_length * 2);

            let a = p_str[..pad_length].parse::<usize>()
                .map_err(|e| format!("Failed to parse a: {}", e))?;

            let b = p_str[pad_length..].parse::<usize>()
                .map_err(|e| format!("Failed to parse b: {}", e))?;

            decrypted.push(self.char_set.char_at(a));

            if b != 0 {
                decrypted.push(self.char_set.char_at(b));
            }
        }

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

        // Decrypt
        let decrypted = pohlig_hellman.decrypt(&encrypted, key)?;
        println!("Decrypted: {}", decrypted);

        assert_eq!(decrypted, text);

        Ok(())
    }
}
