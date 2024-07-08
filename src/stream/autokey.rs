/*===================================================================
 * Implementation of basic Plaintext & Ciphertext autokey cipher 
 * Author: denosaurabh
 *===================================================================*/


use std::usize;

use crate::utils::{char_set::CharSet, mod_arithmetic::ModArithmetic};


pub enum AutoKeyType {
    Plaintext,
    Ciphertext,
    // Key
}

pub struct Autokey {
    mode: AutoKeyType,
    char_set: CharSet,
}

impl Autokey {
    pub fn new(mode: AutoKeyType, char_set: CharSet) -> Self {
        Self { mode, char_set }
    }

    /// can be any other cipher or excryption method
    pub fn transformation(&self, c: char, inverse: bool) -> char {
        let c_index = self.char_set.index_of(c) as u32;
        let m = self.char_set.len() as u32;

        let new_char_index: u32;

        if inverse {
            let mult_inv = ModArithmetic::mod_inverse(25, m as i32).expect("inverse doesn't exist") as u32;
            new_char_index = ModArithmetic::modm_u32(mult_inv*ModArithmetic::modm_u32((if c_index == 0 { 26 } else { c_index }) - 1, m), m);
        } else {
            new_char_index = ModArithmetic::modm_u32(25*c_index + 1, m);
        }

        self.char_set.char_at(new_char_index as usize)
    }

    /// iv - initialization vector
    pub fn encrypt(&self, text: &str, iv: &str) -> String {
        let chars = text.chars().collect::<Vec<char>>();
        let iv_chars = iv.chars().collect::<Vec<char>>();

        let mut encrypted: Vec<char> = vec![];

        for (i, c) in text.chars().enumerate() {
            let shifted_c: char = if i < iv.len() { iv_chars[i] } else { 
                match self.mode {
                    AutoKeyType::Plaintext => chars[i - iv.len()],
                    AutoKeyType::Ciphertext => encrypted[i - iv.len()],
                }
            };
            let keystream_char = self.transformation(shifted_c, false);

            let encrypted_char = self.char_set.char_at(
                ModArithmetic::add_usize(
                    self.char_set.index_of(c),
                    self.char_set.index_of(keystream_char) as i32,
                    self.char_set.len() 
                )
            );

            // println!("{} -> {} -> {} -> {}", shifted_c, keystream_char, c, encrypted_char);

            encrypted.push(encrypted_char);
        }

        encrypted.iter().collect()
    }

    pub fn decrypt(&self, text: &str, iv: &str) -> String {
        let iv_chars = iv.chars().collect::<Vec<char>>();

        let mut decrypted: Vec<char> = vec![];

        for (i, c) in text.chars().enumerate() {
            let shifted_c = if i < iv.len() { iv_chars[i] } else { 
                match self.mode {
                    AutoKeyType::Ciphertext => text.chars().nth(i - iv.len()).unwrap(),
                    AutoKeyType::Plaintext => decrypted[i - iv.len()],
                }
            };
            let inverted_transformation = self.transformation(shifted_c, true);

            let plaintext = self.char_set.char_at(
                ModArithmetic::add_usize(
                    self.char_set.index_of(c),
                    -(self.char_set.index_of(inverted_transformation) as i32),

                    self.char_set.len()
                )
            );

            // println!("{} -> {} -> {} -> {}", shifted_c, inverted_transformation, c, plaintext);

            decrypted.push(plaintext);
        }

        decrypted.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autokey_encrypt() {
        // plaintext autokey
        let charset = CharSet::from_alphabet_lowercase();
        let autokey = Autokey::new(AutoKeyType::Plaintext, charset);

        let text = "aworthlesscracking";
        let expected = "gpnsyuvmmizajbujmx";
        assert_eq!(autokey.encrypt(text, "vic"), expected);

        // ciphertext autokey
        let charset = CharSet::from_alphabet_lowercase();
        let autokey = Autokey::new(AutoKeyType::Ciphertext, charset);
        let text = "wasteallyouroil";
        let expected = "ctrsmkuapvvduoj";
        assert_eq!(autokey.encrypt(text, "vic"), expected);
    }


    #[test]
    fn test_autokey_decrypt() {

        // plaintext autokey
        let charset = CharSet::from_alphabet_lowercase();
        let autokey = Autokey::new(AutoKeyType::Plaintext, charset);

        let text = "gxtedpfupblqkdjzgu";
        let expected = "aworthlesscracking";
        assert_eq!(autokey.decrypt(text, "v"), expected);

        // ciphertext autokey
        let charset = CharSet::from_alphabet_lowercase();
        let autokey = Autokey::new(AutoKeyType::Ciphertext, charset);

        let text = "ctrsmkuapvvduoj";
        let expected = "wasteallyouroil";
        assert_eq!(autokey.decrypt(text, "vic"), expected);

   }
}
