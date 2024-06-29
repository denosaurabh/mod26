/*===================================================================
 * Rail Fence Cipher 
 * Author: denosauabh
 * Description: Implementation of types Rail Fence Cipher (zigzag cipher)
 * Reference: https://en.wikipedia.org/wiki/Rail_fence_cipher
*===================================================================*/

use crate::utils::{consts::NULL};

pub struct RailFence {
    row_len: u32,
}

impl RailFence {
    pub fn new(row_len: u32) -> Self {
        Self { row_len }
    }

    pub fn encrypt(&self, s: &str) -> String {
        /*
         
         - . . . . . - 
         . - . . . - .
         . . - . - . . 
         . . . - . . . 

         take third line :-

         . . - . - . .

         two hypens are 'charCount`

         first two dots are `beginPad`
         middle dot is `middlePad`
         end two dots are `endPad`

         */

        let text = s.chars().collect::<Vec<char>>();
        
        let period = 2 * (self.row_len - 1);
        let _p_usize = period as usize;

        // let grid = vec![self.row_len, (text.len() as u32) / self.row_len];
        let grid = vec![self.row_len, text.len() as u32];

        let totalPeriods = text.len() / period as usize;

        println!("totalPeriods: {}", totalPeriods);


        let mut encrypted: Vec<char> = Vec::new();
        
        for r in 0..grid[0] {
            let charCount = if r == 0 || r == self.row_len - 1 { 1 } else { 2 };

            let beginPad = r;
            let _beginPad_usize = beginPad as usize;

            let endPad = if r == 0 { 0 } else { beginPad - 1 };

            let middlePad = period - beginPad - endPad - charCount;
            let _middlePad_usize = middlePad as usize;

            println!("beginPad: {}, middlePad: {}, endPad: {}, charCount: {}, period: {}", beginPad, middlePad, endPad, charCount, period);

            for c in 0..grid[1] {
                // period index
                let p = c  / period;

                println!("p: {}, c: {}, p+c: {}, (c % period): {}", p, c, p+c, c % period);

                match charCount {
                    1 => {
                        if (r == 0 && (c % period) != 0) || (c % period) != beginPad {
                            println!("skipping 1");
                            continue;
                        }
                    },
                    2 => {
                        if (c % period) != beginPad && (c % period) != beginPad + middlePad + 1 {
                            println!("skipping 2");
                            continue;
                        }
                    },
                    _ => {}
                }

                let l = p * period;

                match charCount {
                    1 => {
                        println!("LLL: {}", l);
                        encrypted.push(text[(l + beginPad) as usize]);
                    },
                    2 => {

                        // so, it only gets added once
                        if (c % period) == beginPad {
                            encrypted.push(text[(l + beginPad) as usize]);
                            encrypted.push(text[(l + beginPad + middlePad + 1) as usize]);
                        }
                    },
                    _ => {}
                }
            };

        };

        encrypted.iter().collect()
    }


    pub fn decrypt(&self, _text: &str) -> String {
        todo!()
    }

    fn parse_plain_text(&self, s: &str) -> String {
        let rem = s.len() % self.row_len as usize;
        let mut s = s.to_string();

        // Add NULL characters to make the text length a multiple of row_len
        if rem != 0 {
            let nulls = self.row_len as usize - rem;
            s.push_str(&NULL.to_string().repeat(nulls));
        }

        s
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let rail_fence = RailFence::new(3);
        let encrypted = rail_fence.encrypt(&"WE ARE DISCOVERED RUN AT ONCE".replace(" ", ""));
        assert_eq!(encrypted, format!("WECRUOERDSOEERNTNEAIVDAC"));

        let rail_fence = RailFence::new(3);
        assert_eq!(rail_fence.encrypt("HELLO"), format!("HOELL"));

    }

    // #[test]
    // fn test_decrypt() {
    //     let rail_fence = RailFence::new(3);
    //     let decrypted = rail_fence.decrypt("WECRUOERDSOEERNTNEAIVDAC");
    //     assert_eq!(decrypted, "WE ARE DISCOVERED RUN AT ONCE".replace(" ", ""));
    // }
}