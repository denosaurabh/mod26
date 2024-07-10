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
        let text: Vec<char> = text.chars().collect();

        let mut encryped_vec: Vec<Vec<char>> = vec![vec![]; key.len()];

        let rows = (text.len() / key.len()) + if text.len() % key.len() == 0 { 0 } else { 1 };

        for r in 0..rows {
            for c in 0..key.len() {
                let pos = (r * key.len()) + c;
                let ki = key[c];

                if pos < text.len() {
                    encryped_vec[ki - 1].push(text[pos]);
                }
            }
        }

        let encrypted = encryped_vec.iter().flat_map(|v| v.iter()).collect::<String>();

        encrypted.to_string()
    }

    /// TODO: not working for some cases
    pub fn permute_decrypt(&self, text: &str, key: Vec<usize>) -> String {
        let text: Vec<char> = text.chars().collect();

        let cols = key.len();
        let rows = text.len() / cols;

        let mut decryped_vec: Vec<Vec<char>> = vec![vec![]; key.len()];
        let mut decrypted: Vec<char> = vec![NULL; text.len()];
        
        let mut inverse_key = self.inverse_key(key);
        inverse_key = inverse_key.iter().map(|&x| x - 1).collect();

        for i in 0..text.len() {
            let ri = i % rows;
            let ci = i / rows;

            if ci < inverse_key.len() { 
                let k = inverse_key[ci];
                decryped_vec[k].push(text[i]);
            } else { 
                decryped_vec[i - (rows * cols)].push(text[i]);
            };
        }

        for i in 0..decryped_vec.len() {
            for j in 0..decryped_vec[i].len() {
                let pos = (j * cols) + i;
                if pos < decrypted.len() {
                    decrypted[pos] = decryped_vec[i][j];
                }
            }
        }

        println!("decryped_vec: {:?}", decryped_vec);
    
        decrypted.into_iter().filter(|&c| c != NULL).collect()
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
        let columnar = Columnar::new();
        assert_eq!(columnar.parse_key("ZEBRAS"), vec![6, 3, 2, 4, 1, 5]);
    }

    #[test]
    fn test_encrypt() {
        let columnar = Columnar::new();
        assert_eq!(columnar.encrypt(
            &"WE ARE DISCOVERED FLEE AT ONCE QKJEU".replace(' ', ""), "ZEBRAS"), 
            "EVLNE ACDTK ESEAQ ROFOJ DEECU WIREE".replace(' ', "")
        );
        assert_eq!(columnar.encrypt(
            &"WE ARE DISCOVERED FLEE AT ONCE".replace(' ', ""), "ZEBRAS"), 
            "EVLNA CDTES EAROF ODEEC WIREE".replace(' ', "")
        );
    }

    #[test]
    fn test_decrypt() {
        let columnar = Columnar::new();
        assert_eq!(columnar.decrypt(
            &"EVLNA CDTES EAROF ODEEC WIREE".replace(' ', ""), "ZEBRAS"),
            "WE ARE DISCOVERED FLEE AT ONCE".replace(' ', "")
        );

        assert_eq!(
            columnar.decrypt("DGDDDAGDDGAFADDFDADVDVFAADVX", "PRIVACY"),
            "ADDDDDADAGVGADDDAFDGVFVFADDX"
        );

    }
}
