/*===================================================================
 * Author: denosauabh
 * Description: Implementation of simple `C = kP + m` Affine cipher
*===================================================================*/

use crate::utils::char_set::CharSet;
use crate::utils::mod_arithmetic::ModArithmetic;

pub struct AffineCipher {
    char_set: CharSet,
}

impl AffineCipher {
    pub fn new(char_set: CharSet) -> Self {
        Self { char_set }
    }

    pub fn encrypt(&self, text: &str, k: i32, m: i32) -> String {
        text.chars()
            .map(|c| {
                let index = self.char_set.index_of(c);
                let p = ModArithmetic::mult_usize(index, k, self.char_set.len());

                let new_index = ModArithmetic::add_usize(
                    p,
                    m,
                    self.char_set.len()
                );
                self.char_set.char_at(new_index)
                
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str, k: i32, m: i32) -> String {
        let k_inv = ModArithmetic::mod_inverse(k, self.char_set.len() as i32)
            .expect("Invalid key: k is not coprime with m");

        text.chars()
            .map(|c| {
                let index = self.char_set.index_of(c);
                let new_index = ModArithmetic::mult_usize(
                    ModArithmetic::add_usize(index, -m, self.char_set.len()),
                    k_inv,
                    self.char_set.len()
                );
                self.char_set.char_at(new_index)
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let char_set = CharSet::from_numbers();
        println!("char_set: {:?}", char_set.chars);

        let cipher = AffineCipher::new(char_set);


        assert_eq!(cipher.encrypt("123", 7, 3), "074");
    }

    #[test]
    fn test_decrypt() {
        let char_set = CharSet::from_numbers();
        let cipher = AffineCipher::new(char_set);
        assert_eq!(cipher.decrypt("074", 7, 3), "123");
    }
}