/*===================================================================
 * Simple Biliteral Cipher based on Polybius Square
 * Author: denosauabh
 * Reference: https://en.wikipedia.org/wiki/Polybius_square

 * Description:
 
    POLIBIUS SQUARE:

        1	2	3	4	5
    1	A	B	C	D	E
    2	F	G	H	I/J	K
    3	L	M	N	O	P
    4	Q	R	S	T	U
    5	V	W	X	Y	Z

    putting `IJ` is a common practice as the square only allows 25 characters, so one letter needs to fit with someone else. 


 *===================================================================*/


use crate::utils::char_set::CharSet;

pub struct Biliteral {
    char_set: CharSet,
}

impl Biliteral {
    /// as this is based purely on Polybius Square, we will use the default alphabet charset
    pub fn new() -> Self {
        Self { char_set: CharSet::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ") }
    }

    pub fn encrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                let mut index = self.char_set.index_of(c);

                /// to accomodate IJ together
                if index > 9 { index -= 1 };
                
                let row = index / 5 + 1;
                let col = index % 5 + 1;

                format!("{}{}", row, col)
            })
            .collect()
    }

    pub fn decrypt(&self, text: &str) -> String {
        text.chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if i % 2 == 0 {
                    return None;
                };

                let mut row = text.chars().nth(i - 1).unwrap().to_digit(10).unwrap() - 1;
                let mut col = c.to_digit(10).unwrap() - 1;

                if row * 5 + col > 9 {
                    let cc: u32 = col + 1;

                    if cc > 4 {
                        row += 1;
                        col = 0;
                    } else {
                        col += 1;
                    }
                }
                
                let index = row * 5 + col;
                Some(self.char_set.char_at(index as usize))
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_encrypt() {
        let biliteral = Biliteral::new();

        let encrypted = biliteral.encrypt("HELLO");
        assert_eq!(encrypted, "23 15 31 31 34".replace(' ', ""));
    }

    #[test]
    fn test_decrypt() {
        let biliteral = Biliteral::new();

        let decrypted = biliteral.decrypt(&"23 15 31 31 34".replace(' ', ""));
        assert_eq!(decrypted, "HELLO");
    }
}