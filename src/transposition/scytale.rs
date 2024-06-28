/*===================================================================
 * Author: denosauabh
 * Description: Implementation of Scytale
 * Reference: https://en.wikipedia.org/wiki/Scytale
*===================================================================*/

use cursive::Vec2;

use crate::utils::{consts::NULL, mod_arithmetic::ModArithmetic};

pub struct Scytale {
    row_len: u32,
}

impl Scytale {
    pub fn new(row_len: u32) -> Self {
        Self { row_len }
    }

    pub fn encrypt(&self, s: &str) -> String {
        let text = self.parse_plain_text(s).chars().collect::<Vec<char>>();

        let grid = Vec2::new(self.row_len as usize, text.len() / self.row_len as usize);

        let mut encrypted = Vec::new();

        for r in 0..grid.x {
            for c in 0..grid.y {
                let index = c * grid.x + r;
                encrypted.push(if index < text.len() { text[index] } else { NULL });
            }
        }

        encrypted.iter().collect()
    }

    pub fn decrypt(&self, text: &str) -> String {
        let text = text.chars().collect::<Vec<char>>();

        let grid = Vec2::new(self.row_len as usize, text.len() / self.row_len as usize);

        let mut decrypted = Vec::new();

        for r in 0..grid.y {
            for c in 0..grid.x {
                let index = c * grid.y + r;

                if text[index] != NULL {
                    decrypted.push(text[index]);
                }
            }
        }

        decrypted.iter().collect()
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
    fn test_parse_plain_text() {
        let scytale = Scytale::new(5);
        let parsed = scytale.parse_plain_text("HELLO WORLD!");
        assert_eq!(parsed, format!("HELLO WORLD!{}{}{}", NULL, NULL, NULL));
    }

    #[test]
    fn test_encrypt() {
        let scytale = Scytale::new(3);
        let encrypted = scytale.encrypt("HELLO WORLD");
        assert_eq!(encrypted, format!("HLWLEOODL R{}", NULL));

        let scytale = Scytale::new(4);
        assert_eq!(scytale.encrypt("I am hurt very badly help"), format!("I tra p h ydh{}auv le�mrebyl{}", NULL, NULL));
    }

    #[test]
    fn test_decrypt() {
        let scytale = Scytale::new(3);
        let decrypted = scytale.decrypt(format!("HLWLEOODL R{}", NULL).as_str());
        assert_eq!(decrypted, "HELLO WORLD");
    }
}