/*===================================================================
 * Implementation of ADFGVX cipher
 * Author: denosauabh
 * Reference: https://en.wikipedia.org/wiki/ADFGVX_cipher

 * Description:
 
        A	D	F	G	V	X
    A	N	A	1	C	3	H
    D	8	T	B	2	O	M
    F	E	5	W	R	P	D
    G	4	F	6	G	7	I
    V	9	J	0	K	L	Q
    X	S	U	V	X	Y	Z


    there's also a version of this cipher called ADFGX, which is a simplified version of ADFGVX
    where the alphabet is reduced to 5 characters, and the grid is 5x5 rather than 6x6


 *===================================================================*/


 use crate::{transposition::columnar::Columnar, utils::char_set::CharSet};

 pub struct ADFGVX {
    char_set: CharSet,

    row_char_set: CharSet,
    col_char_set: CharSet,
 }
 
 impl ADFGVX {
     /// as this is based purely on Polybius Square, we will use the default alphabet charset
     pub fn new(
        char_set: Option<CharSet>,

        row_char_set: Option<CharSet>,
        col_char_set: Option<CharSet>,
     ) -> Self {
        let char_set = char_set.unwrap_or(CharSet::from_string("NA1C3H8TB2OME5WRPD4F6G7I9J0KLQSUVXYZ"));

        let row_char_set = row_char_set.unwrap_or(CharSet::from_string("ADFGVX"));
        let col_char_set = col_char_set.unwrap_or(CharSet::from_string("ADFGVX"));

        if char_set.len() != row_char_set.len() * col_char_set.len() {
            panic!("char_set length must be equal to row_char_set length * col_char_set length");
        }

        Self {
            char_set,
            row_char_set,
            col_char_set
        }
     }
 
     pub fn encrypt(&self, text: &str, key: &str) -> String {
        let mut encrypted = String::new();

        // polyliterial substitution using a table
        for c in text.chars() {
            let index = self.char_set.index_of(c);
            let row = index / self.col_char_set.len();
            let col = index % self.col_char_set.len();

            encrypted.push(self.row_char_set.char_at(row));
            encrypted.push(self.col_char_set.char_at(col));
        }

        // columnar transposition using key
        let columnar = Columnar::new();
        encrypted = columnar.encrypt(&encrypted, key);

        encrypted
     }
 
     pub fn decrypt(&self, text: &str, key: &str) -> String {
        // reverse columnar transposition using key
        let columnar = Columnar::new();
        let transposed_text = columnar.decrypt(text, key);

        println!("transposed_text: {}", transposed_text);

        let mut decrypted = String::new();

        // reverse polyliterial substitution using a table
        for i in 0..transposed_text.len() / 2 {
            let row = self.row_char_set.index_of(transposed_text.chars().nth(i * 2).unwrap());
            let col = self.col_char_set.index_of(transposed_text.chars().nth(i * 2 + 1).unwrap());

            let index = row * self.col_char_set.len() + col;
            decrypted.push(self.char_set.char_at(index));
        }

        decrypted
    }
 }
 
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_encrypt() {
         let adfgvx = ADFGVX::new(Option::None, Option::None, Option::None);
 
         let encrypted = adfgvx.encrypt(&"ATTACK AT 1200 AM".replace(" ", ""), "PRIVACY");
         assert_eq!(encrypted, "DGDD DAGD DGAF ADDF DADV DVFA ADVX".replace(" ", ""));
     }
 
     #[test]
     fn test_decrypt() {
        let adfgvx = ADFGVX::new(Option::None, Option::None, Option::None);
 
        let decrypted: String = adfgvx.decrypt(&"DGDD DAGD DGAF ADDF DADV DVFA ADVX".replace(" ", ""), "PRIVACY");
        assert_eq!(decrypted, "ATTACK AT 1200 AM".replace(" ", ""));
     }
 }