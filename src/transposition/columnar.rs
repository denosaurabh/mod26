/*===================================================================
 * Columnar Transposition Cipher
 * Author: denosauabh
 * Description: Implementation of Columnar Transposition Cipher
 * Reference: https://en.wikipedia.org/wiki/Transposition_cipher#Columnar_transposition
*===================================================================*/

use crate::utils::consts::NULL;

pub struct Columnar {}

impl Columnar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encrypt(&self, text: &str, key: &str) -> String {
        self.permute_encrypt(text, self.parse_key(key))
    }

    pub fn decrypt(&self, text: &str, key: &str) -> String {
        self.permute_decrypt(text, self.parse_key(key))
    }

    pub fn permute_encrypt(&self, text: &str, key: Vec<usize>) -> String {
        println!("key: {:?}", key);

        let text: Vec<char> = text.chars().collect();

        let mut encryped_vec: Vec<Vec<char>> = vec![vec![]; key.len()];

        let rows = (text.len() / key.len()) + if text.len() % key.len() == 0 { 0 } else { 1 };

        for r in 0..rows {
            for c in 0..key.len() {
                let pos = (r * key.len()) + c;
                let ki = key[c];

                println!("r: {}, c: {}, pos: {}, ki: {}, p: {:?}", r, c, pos, ki, text.get(pos));

                if pos < text.len() {
                    encryped_vec[ki - 1].push(text[pos]);
                }
            }
        }

        let encrypted = encryped_vec.iter().flat_map(|v| v.iter()).collect::<String>();

        encrypted.to_string()
    }

    pub fn permute_decrypt(&self, text: &str, key: Vec<usize>) -> String {
        let text: Vec<char> = text.chars().collect();
        let cols = key.len();
        let rows = (text.len() / cols) + if text.len() % cols == 0 { 0 } else { 1 };
        let mut decrypted: Vec<char> = vec![NULL; rows * cols];
        
        let inverse_key = self.inverse_key(key);
        let mut pos = 0;
    
        for &col in inverse_key.iter() {
            let col_length = if col <= text.len() % cols { rows } else { rows - 1 };
            for row in 0..col_length {
                if pos < text.len() {
                    decrypted[row * cols + col - 1] = text[pos];
                    pos += 1;
                }
            }
        }
    
        decrypted.into_iter().filter(|&c| c != NULL).collect()
    }

    pub fn parse_key(&self, s: &str) -> Vec<usize> {
        let mut chars: Vec<char> = s.chars().collect();
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
        let columnar = Columnar::new();
        assert_eq!(columnar.parse_key("ZEBRAS"), vec![6, 3, 2, 4, 1, 5]);
    }

    #[test]
    fn test_encrypt() {
        let columnar = Columnar::new();
        assert_eq!(columnar.encrypt(
            &"WE ARE DISCOVERED FLEE AT ONCE QKJEU".replace(" ", ""), "ZEBRAS"), 
            "EVLNE ACDTK ESEAQ ROFOJ DEECU WIREE".replace(" ", "")
        );
        assert_eq!(columnar.encrypt(
            &"WE ARE DISCOVERED FLEE AT ONCE".replace(" ", ""), "ZEBRAS"), 
            "EVLNA CDTES EAROF ODEEC WIREE".replace(" ", "")
        );
    }

    #[test]
    fn test_decrypt() {
        let columnar = Columnar::new();
        assert_eq!(columnar.decrypt(
            &"EVLNA CDTES EAROF ODEEC WIREE".replace(" ", ""), "ZEBRAS"),
            "WE ARE DISCOVERED FLEE AT ONCE".replace(" ", "")
        );

    }
}