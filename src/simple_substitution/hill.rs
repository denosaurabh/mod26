/*===================================================================
 * Author: denosauabh
 * Description: Implementation of Hill Cipher by Lester S. Hill
 * Reference: https://en.wikipedia.org/wiki/Hill_cipher     
                
 * Formula:
 
 2-letter block example:
 `C1 = k1*P1 + k2*P2 (mod 26)`
 `C2 = k3*P1 + k4*P2 (mod 26)`

*===================================================================*/

use crate::utils::char_set::CharSet;
use crate::utils::mod_arithmetic::ModArithmetic;

pub struct HillCipher {
    char_set: CharSet,
    block_size: u8,
}

impl HillCipher {
    pub fn new(char_set: CharSet) -> Self {
        Self {
            char_set, 
            block_size: 2
         }
    }

    pub fn encrypt(&self, text: &str, key: [i32; 4]) -> String {
        let blocks = self.create_blocks(text);
        let m = self.char_set.len() as i32;

        blocks
            .iter()
            .map(|block| {
                let bi: Vec<i32> = block.iter().map(|c| self.char_set.index_of(*c) as i32).collect::<Vec<i32>>();

                let c1 = ModArithmetic::add(
                    bi[0] * key[0],
                    bi[1] * key[1],
                    m
                );
                let c2 = ModArithmetic::add(
                    bi[0] * key[2],
                    bi[1] * key[3],
                    m
                );

                format!("{}{}", self.char_set.char_at(c1 as usize), self.char_set.char_at(c2 as usize))
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str, key: [i32; 4]) -> String {
        let blocks = self.create_blocks(text);
        let m = self.char_set.len() as i32;

        // inverse keys
        let k1 = key[0];
        let k2 = key[1];
        let k3 = key[2];
        let k4 = key[3];

        let key_inv = ModArithmetic::mod_inverse(ModArithmetic::modm((k1*k4) - (k2*k3), m), m).expect("inverse doesn't exist");

        let m1: i32 = key_inv * k4; 
        let m2: i32 = key_inv * -k2; 
        let m3: i32 = key_inv * -k3;  
        let m4: i32 = key_inv * k1; 

        blocks
            .iter()
            .map(|block| {
                let bi: Vec<i32> = block.iter().map(|c| self.char_set.index_of(*c) as i32).collect::<Vec<i32>>();

                let p1 = ModArithmetic::add(
                    bi[0] * m1,
                    bi[1] * m2,
                    m
                );
                let p2 = ModArithmetic::add(
                    bi[0] * m3,
                    bi[1] * m4,
                    m
                );

                format!("{}{}", self.char_set.char_at(p1 as usize), self.char_set.char_at(p2 as usize))
            })
            .collect()
    }


    pub fn create_blocks(&self, text: &str) -> Vec<Vec<char>> {
        let binding = text.chars().collect::<Vec<char>>();
        let blocks = binding.chunks(self.block_size as usize);

        // if the last block is not of size `block_size`, pad it with `PAD_CHAR`
        blocks.map(|block| {
            let mut block = block.to_vec();
            while block.len() < self.block_size as usize {
                block.push(self.PAD_CHAR());
            }
            block
        }).collect()
    }

    pub fn PAD_CHAR(&self) -> char {
        // self.char_set.char_at(0).unwrap_or(' ')
        '\u{FFFD}'
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let char_set = CharSet::from_numbers();
        let hill_cipher = HillCipher::new(char_set);

        assert_eq!(hill_cipher.encrypt("1234", [1, 2, 3, 4]), "5115");

        
        let char_set_sm_alph = CharSet::from_alphabet_smallcase();
        let hill_cipher_sm_alph = HillCipher::new(char_set_sm_alph);

        assert_eq!(hill_cipher_sm_alph.encrypt("jack", [3, 5, 6, 1]), "bcew");

    }

    #[test]
    fn test_decrypt() {
        let char_set_sm_alph = CharSet::from_alphabet_smallcase();
        let hill_cipher_sm_alph = HillCipher::new(char_set_sm_alph);


        // assert_eq!(hill_cipher_sm_alph.encrypt("bcew", [25, 5, 6, 23]), "jack");
        // assert_eq!(hill_cipher_sm_alph.encrypt("bcew", [1, 21, 20, 3]), "jack");
        assert_eq!(hill_cipher_sm_alph.decrypt("bcew", [3, 5, 6, 1]), "jack");
    }
}