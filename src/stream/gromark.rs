use std::usize;

use crate::utils::{char_set::CharSet, mod_arithmetic::ModArithmetic};

pub struct Gromark {
    char_set: CharSet,
    last_five_keys: Vec<usize>,
}

impl Gromark {
    pub fn new(char_set: CharSet) -> Self {
        Self { 
            char_set,
            last_five_keys: Vec::new(),
        }
    }

    pub fn lagged_fibonacci_generator(&mut self, default_iv_vec: Vec<usize>) -> usize {
        if self.last_five_keys.len() < 5 {
            let key = default_iv_vec[self.last_five_keys.len()];
            self.last_five_keys.push(key);
            key
        } else {
            let new_key = (self.last_five_keys[0] + self.last_five_keys[1]) % self.char_set.len();
            self.last_five_keys.remove(0);
            self.last_five_keys.push(new_key);
            new_key
        }
    }

    pub fn encrypt(&mut self, text: &str, default_iv_vec: Vec<usize>) -> String {
        self.char_set.panic_if_invalid(text);
        self.last_five_keys.clear();

        let mut encrypted = String::new();

        for (i, c) in text.chars().enumerate() {
            let key = self.lagged_fibonacci_generator(default_iv_vec.clone());

            println!("key: {}", key);

            let char_index = self.char_set.index_of(c);
            let encrypted_index = ModArithmetic::add_usize(char_index, key as i32, self.char_set.len());
            let encrypted_char = self.char_set.char_at(encrypted_index);

            encrypted.push(encrypted_char);
        }

        encrypted
    }

    pub fn decrypt(&mut self, text: &str, default_iv_vec: Vec<usize>) -> String {
        self.char_set.panic_if_invalid(text);
        self.last_five_keys.clear();

        let mut decrypted = String::new();

        for (i, c) in text.chars().enumerate() {
            let key = self.lagged_fibonacci_generator(default_iv_vec.clone());

            let char_index = self.char_set.index_of(c);
            let decrypted_index = ModArithmetic::add_usize(char_index, -(key as i32), self.char_set.len());
            let decrypted_char = self.char_set.char_at(decrypted_index);

            decrypted.push(decrypted_char);
        }

        decrypted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gromark() {
        let charset = CharSet::from_alphabet_lowercase();
        let mut gromark = Gromark::new(charset);

        let text = "multiplylikerabbits";
        let expected = "nwlvrsnawupiexskzdg";
        assert_eq!(gromark.encrypt(text, vec![1,2,0,2,9]), expected);
    }
}
