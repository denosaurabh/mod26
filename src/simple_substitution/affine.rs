use crate::utils::char_set::CharSet;
use crate::utils::mod_arithmetic::ModArithmetic;

pub struct AffineCipher {
    char_set: CharSet,
}

impl AffineCipher {
    pub fn encrypt(&self, text: &str, k: i32, m: i32) -> String {
        text.chars()
            .map(|c| {
                if let Some(index) = self.char_set.index_of(c) {
                    let new_index = ModArithmetic::add_usize(
                        ModArithmetic::mult_usize(index, k, self.char_set.len()),
                        m,
                        self.char_set.len()
                    );
                    self.char_set.char_at(new_index).unwrap_or(c)
                } else {
                    c
                }
                
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str, k: i32, m: i32) -> String {
        let k_inv = ModArithmetic::mod_inverse(k, self.char_set.len() as i32)
            .expect("Invalid key: k is not coprime with m");

        text.chars()
            .map(|c| {
                if let Some(index) = self.char_set.index_of(c) {
                    let new_index = ModArithmetic::mult_usize(
                        ModArithmetic::add_usize(index, -m, self.char_set.len()),
                        k_inv,
                        self.char_set.len()
                    );
                    self.char_set.char_at(new_index).unwrap_or(c)
                } else {
                    c
                }
                
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let char_set = CharSet::from_string("1234567890");
        let cipher = AffineCipher { char_set };
        assert_eq!(cipher.encrypt("123", 7, 3), "074");
    }

    #[test]
    fn test_decrypt() {
        let char_set = CharSet::from_string("1234567890");
        let cipher = AffineCipher { char_set };
        assert_eq!(cipher.decrypt("074", 7, 3), "123");
    }
}