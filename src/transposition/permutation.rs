/*===================================================================
 * Permutation Cipher
 * Author: denosauabh
 * Description: Implementation of simple Permutation cipher
*===================================================================*/

use crate::utils::consts::NULL;

pub struct Permutation {}

impl Permutation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encrypt(&self, text: &str, key: &str) -> String {
        self.permute(text, self.inverse_key(self.parse_key(key)))
    }

    pub fn decrypt(&self, text: &str, key: &str) -> String {
        self.permute(text, self.parse_key(key))
    }

    pub fn permute(&self, text: &str, key: Vec<usize>) -> String {
        let text: Vec<char> = text.chars().collect();

        if key.len() > text.len() {
            panic!("Key length must be less than or equal to text length");
        };

        let mut encrypted = Vec::new();

        for i in 0..text.len() {
            let ki: usize = (i / key.len()) * key.len();
            let index = i % key.len();
            let pos = key[index] - 1;

            if let Some(c) = text.get(ki + pos) {
                encrypted.push(*c);
            } else {
                encrypted.push(NULL);
            }
        };

        encrypted.iter().collect()
    }

    pub fn parse_key(&self, s: &str) -> Vec<usize> {
        let chars: Vec<char> = s.chars().collect();
        let mut order: Vec<usize> = (0..chars.len()).collect();
        
        order.sort_by_key(|&i| chars[i]);
        
        let mut result = vec![0; chars.len()];
        for (i, &pos) in order.iter().enumerate() {
            result[pos] = i + 1;
        }
        
        result
    }

    pub fn inverse_key(&self, key: Vec<usize>) -> Vec<usize> {
        let mut result = vec![0; key.len()];
        for (i, &pos) in key.iter().enumerate() {
            result[pos - 1] = i + 1;
        }
        
        result 
    }

    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsekey() {
        let permutation = Permutation::new();
        assert_eq!(permutation.parse_key("HELOU"), vec![2, 1, 3, 4, 5]);
        assert_eq!(permutation.parse_key("TALE"), vec![4, 1, 3, 2]);
    }

    #[test]
    fn test_inversekey() {
        let permutation = Permutation::new();
        assert_eq!(permutation.inverse_key(vec![2, 4, 3, 1]), vec![4, 1, 3, 2]);
    }

    #[test]
    fn test_encrypt() {
        let permutation = Permutation::new();
        assert_eq!(permutation.encrypt("theb", "TALE"), "hbet");
        assert_eq!(permutation.encrypt("abcdefgh", "MEOW"), "bacdfegh");
        assert_eq!(permutation.encrypt("abcde", "MEOW"), format!("bacd{}", NULL));
    }

    #[test]
    fn test_decrypt() {
        let permutation = Permutation::new();
        assert_eq!(permutation.decrypt("hbet", "TALE"), "theb");
        assert_eq!(permutation.decrypt("bacdfegh", "MEOW"), "abcdefgh");
        assert_eq!(permutation.decrypt(&format!("bacd{}", NULL), "MEOW"), format!("abcd{}", NULL)); // a character gets lost
    }
}