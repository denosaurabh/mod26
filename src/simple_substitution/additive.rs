/*===================================================================
 * Author: denosauabh
 * Description: Implementation of simple Additive cipher
*===================================================================*/

use crate::utils::char_set::CharSet;
use crate::utils::mod_arithmetic::ModArithmetic;


pub struct AdditiveCipher {
    char_set: CharSet,
}

impl AdditiveCipher {
    pub fn new(char_set: CharSet) -> Self {
        Self { char_set }
    }

    pub fn encrypt(&self, text: &str, key: i32) -> String {
        text.chars()
            .map(|c| self.shift_char(c, key))
            .collect()
    }

    pub fn decrypt(&self, text: &str, key: i32) -> String {
        self.encrypt(text, -key)
    }

    fn shift_char(&self, c: char, key: i32) -> char {
        if let Some(index) = self.char_set.index_of(c) {
            let new_index = ModArithmetic::add_usize(index, key, self.char_set.len());
            self.char_set.char_at(new_index).unwrap_or(c)
        } else {
            c
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_lowercase() {
        let char_set = CharSet::from_reduced_ascii();
        let cipher = AdditiveCipher::new(char_set);
        assert_eq!(cipher.encrypt("hello", 3), "khoor");
        assert_eq!(cipher.decrypt("khoor", 3), "hello");
    }

    #[test]
    fn test_ascii_uppercase() {
        let char_set = CharSet::from_range('A', 'Z');
        let cipher = AdditiveCipher::new(char_set);
        assert_eq!(cipher.encrypt("HELLO", 3), "KHOOR");
        assert_eq!(cipher.decrypt("KHOOR", 3), "HELLO");
    }

    #[test]
    fn test_mixed_case_and_punctuation() {
        let char_set = CharSet::from_ascii(); 
        let cipher = AdditiveCipher::new(char_set);
        assert_eq!(cipher.encrypt("Hello, World!", 3), "Khoor/#Zruog$");
        assert_eq!(cipher.decrypt("Khoor/#Zruog$", 3), "Hello, World!");
    }

    #[test]
    fn test_unicode() {
        let char_set = CharSet::from_range('α', 'ω'); 
        let cipher = AdditiveCipher::new(char_set);
        assert_eq!(cipher.encrypt("αβγδε", 2), "γδεζη");
        assert_eq!(cipher.decrypt("γδεζη", 2), "αβγδε");
    }

    #[test]
    fn test_custom_char_set() {
        let char_set = CharSet::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        let cipher = AdditiveCipher::new(char_set);
        assert_eq!(cipher.encrypt("Hello123", 5), "Mjqqt678");
        assert_eq!(cipher.decrypt("Mjqqt678", 5), "Hello123");
    }
}