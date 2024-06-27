/*===================================================================
 * Author: denosauabh
 * Description: Implementation of simple Multiplicative cipher
*===================================================================*/


use crate::utils::char_set::CharSet;
use crate::utils::mod_arithmetic::ModArithmetic;

struct MultiplicativeCipher {
    char_set: CharSet,
}

impl MultiplicativeCipher {
    pub fn new(char_set: CharSet) -> Self {
        Self { char_set }
    }

    pub fn encrypt(&self, text: &str, key: i32) -> String {
        text.chars()
            .map(|c| self.shift_char(c, key))
            .collect()
    }

    pub fn decrypt(&self, text: &str, key: i32) -> String {
        match ModArithmetic::mod_inverse(key, self.char_set.len() as i32) {
            Ok(inv_key) => self.encrypt(text, inv_key),
            Err(err) => {
               panic!("{}", err)
            },
        }
    }

    fn shift_char(&self, c: char, key: i32) -> char {
        let index = self.char_set.index_of(c);
        let new_index = ModArithmetic::mult_usize(index, key, self.char_set.len());

        if new_index == 0 {
            return '\u{FFFD}';
        }

        self.char_set.char_at(new_index)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let char_set = CharSet::from_numbers();
        let cipher = MultiplicativeCipher::new(char_set);
        assert_eq!(cipher.encrypt("123", 3), "369");

        let r_ascii_char_set = CharSet::from_reduced_ascii();
        let rascii_cipher = MultiplicativeCipher::new(r_ascii_char_set);

        assert_eq!(rascii_cipher.encrypt("HeLlO..", 0), "\u{FFFD}".repeat(7));
        assert_eq!(rascii_cipher.encrypt("HeLlO..", 1), "HeLlO..");
        assert_eq!(rascii_cipher.encrypt("!@HeLllO!..", 8), "(bCmcFF{(11");
    }

    #[test]
    fn test_decrypt() {
        let char_set = CharSet::from_numbers();
        let cipher = MultiplicativeCipher::new(char_set);
        assert_eq!(cipher.decrypt("369", 3), "123");
    }

}