/*===================================================================
 * Author: denosauabh
 * Description: Implementation of Alberti Cipher
 * Reference: https://en.wikipedia.org/wiki/Alberti_cipher
*===================================================================*/

use crate::utils::char_set::CharSet;


pub struct AlbertiCipher {
    char_set: CharSet,
    disk: CharSet,

    period_len: u32,
}


impl AlbertiCipher {
    pub fn new(char_set: CharSet, disk: CharSet, period_len: u32) -> Self {
        if disk.len() != char_set.len() {
            panic!("Disk charset must be equal to the main charset");
        }

        Self { char_set, disk, period_len }
    }

    pub fn encrypt(&self, text: &str) -> String {
        text.chars()
            .enumerate()
            .map(|(i, c)| {
                let pi = self.char_set.index_of(c);
                
                let period = i / (self.period_len as usize);
                

                self.disk.char_at(pi + period )
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str) -> String {
        text.chars()
            .enumerate()
            .map(|(i, c)| {
                let di = self.disk.index_of(c);
                
                let period = i / (self.period_len as usize);
                

                self.char_set.char_at(di - period)
            })
            .collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let char_set = CharSet::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let disk = CharSet::from_string(/**/"CDEFGHIJKLMNOPQRSTUVWXYZAB");
        let alberti = AlbertiCipher::new(char_set, disk, 4);

        let encrypted = alberti.encrypt("ZABCDEF");
        assert_eq!(encrypted, "BCDEGHI");
    }

    #[test]
    fn test_decrypt() {
        let char_set = CharSet::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let disk = CharSet::from_string(/**/"CDEFGHIJKLMNOPQRSTUVWXYZAB");
        let alberti = AlbertiCipher::new(char_set, disk, 4);

        let decrypted = alberti.decrypt("BCDEGHI");
        assert_eq!(decrypted, "ZABCDEF");
    }
}