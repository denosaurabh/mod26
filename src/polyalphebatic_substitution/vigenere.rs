/*===================================================================
 * Author: denosauabh
 * Description: Implementation of Vigenère Cipher
 * Reference: https://en.wikipedia.org/wiki/Vigenère_cipher
*===================================================================*/

use crate::utils::{char_set::CharSet, mod_arithmetic::ModArithmetic};

pub struct VigenèreCipher {
    char_set: CharSet,
}

impl VigenèreCipher {
    pub fn new(char_set: CharSet) -> Self {
        Self { char_set }
    }

    pub fn encrypt(&self, text: &str, key: &str) -> String {
        text.chars()
            .enumerate()
            .map(|(i, c)| {
                let i: i32 = ModArithmetic::modm(i as i32, key.len() as i32);

                let p = self.char_set.index_of(c);
                let k = self.char_set.index_of(key.chars().nth(i as usize).unwrap());

                println!("p: {}, k: {}", p, k);

                let ci = ModArithmetic::modm((p + k) as i32, self.char_set.len() as i32) as usize;
                self.char_set.char_at(ci)
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str, key: &str) -> String {
        text.chars()
            .enumerate()
            .map(|(i, c)| {
                let ki = self.char_set.index_of(key.chars().nth(i % key.len()).unwrap());
                let ci = self.char_set.index_of(c);

                let pi = (ci + self.char_set.len() - ki) % self.char_set.len();
                self.char_set.char_at(pi)
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::char_set::CharSet;

    #[test]
    fn test_encrypt() {
        let char_set = CharSet::from_alphabet_smallcase();
        let vigenere = VigenèreCipher::new(char_set);

        let encrypted = vigenere.encrypt("attackingtonight", &"OCULORHINOLARINGOLOGY".to_lowercase());
        assert_eq!(encrypted, "ovnlqbpvthznzouz");


        let char_set = CharSet::from_numbers();
        let vigenere = VigenèreCipher::new(char_set);

        let encrypted = vigenere.encrypt("923467234", "82394343");
        assert_eq!(encrypted, "746300662");
    }

    #[test]
    fn test_decrypt() {
        let char_set = CharSet::from_alphabet_smallcase();
        let vigenere = VigenèreCipher::new(char_set);

        let decrypted = vigenere.decrypt("ovnlqbpvthznzouz", &"OCULORHINOLARINGOLOGY".to_lowercase());
        assert_eq!(decrypted, "attackingtonight");


        let char_set = CharSet::from_numbers();
        let vigenere = VigenèreCipher::new(char_set);

        let decrypted = vigenere.decrypt("746300662", "82394343");
        assert_eq!(decrypted, "923467234");
    }
 
}