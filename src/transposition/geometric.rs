/*===================================================================
 * Author: denosauabh
 * Description: Implementation of types of Geometric Transposition Ciphers,
 
 below - Horizontal,
        AlternateHorizontal,
        Vertical,
        AlternateVertical,
        Diagonal,
        AlternateDiagonal,
        SpiralClockwise,
        SpiralCounterClockwise

        were first listed by Cononel Parker Hitt in his book "US Army Mannual of Cryptography" in WW1

 *===================================================================*/

use cursive::Vec2;

use crate::utils::{consts::NULL};

enum GeometricTranspositionMethod {
    Horizontal,
    AlternateHorizontal,

    Vertical,
    AlternateVertical,

    Diagonal,
    AlternateDiagonal,

    SpiralClockwise,
    SpiralCounterClockwise,
}

pub struct Geometric {
    method: GeometricTranspositionMethod,
    row_len: u32,
}

impl Geometric {
    pub fn new (method: GeometricTranspositionMethod, row_len: u32) -> Self {
        Self { method, row_len }
    }

    pub fn encrypt(&self, s: &str) -> String {
        let text = self.parse_plain_text(s).chars().collect::<Vec<char>>();

        let grid = Vec2::new(self.row_len as usize, text.len() / self.row_len as usize);

        let mut encrypted: Vec<char> = Vec::new();

        for r in 0..grid.y {
            for c in 0..grid.x {

                let index = match self.method {
                    GeometricTranspositionMethod::Horizontal => r * grid.x + c,
                    GeometricTranspositionMethod::AlternateHorizontal => r * grid.x + if r % 2 == 0 { c } else { grid.x - c - 1 },

                    GeometricTranspositionMethod::Vertical => c * grid.y + r,
                    GeometricTranspositionMethod::AlternateVertical => c * grid.y + if c % 2 == 0 { r } else { grid.y - r - 1 },

                    GeometricTranspositionMethod::Diagonal => {
                        todo!()
                    },
                    GeometricTranspositionMethod::AlternateDiagonal => {
                        todo!()
                    },

                    GeometricTranspositionMethod::SpiralClockwise => {
                        todo!()
                    },
                    GeometricTranspositionMethod::SpiralCounterClockwise => {
                        todo!()
                    }
                };

                encrypted.push(if index < text.len() { text[index] } else { NULL });
            }
        }

        encrypted.iter().collect()
    }

    fn calculate_value(&self, row: usize, col: usize) -> usize {
        let sum = row + col;
        (sum * (sum + 1)) / (2 + row)
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

    const TEXT: &str = "ABCDEFGHIJKLMNOPQRSTUVWX";   

    #[test]
    fn test_horizontal() {
        let geometric = Geometric::new(GeometricTranspositionMethod::Horizontal, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "ABCDEFGHIJKLMNOPQRSTUVWX");
    }

    #[test]
    fn test_alternate_horizontal() {
        let geometric = Geometric::new(GeometricTranspositionMethod::AlternateHorizontal, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "ABCDEFLKJIHGMNOPQRXWVUTS");
    }

    #[test]
    fn test_vertical() {
        let geometric = Geometric::new(GeometricTranspositionMethod::Vertical, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "AEIMQUBFJNRVCGKOSWDHLPTX");
    }

    #[test]
    fn test_alternate_vertical() {
        let geometric = Geometric::new(GeometricTranspositionMethod::AlternateVertical, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "AHIPQXBGJORWCFKNSVDELMTU");
    }

    /*

    #[test]
    fn test_diagonal() {
        let geometric = Geometric::new(GeometricTranspositionMethod::Diagonal, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "ABDGKOCEHLPSFIMQTVJNRUWX");
    }

    #[test]
    fn test_alternate_diagonal() {
        let geometric = Geometric::new(GeometricTranspositionMethod::AlternateDiagonal, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "ABFGNOCEHMPUDILQTVJKRSWX");
    }

    #[test]
    fn test_spiral_clockwise() {
        let geometric = Geometric::new(GeometricTranspositionMethod::SpiralClockwise, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "ABCDEFPQRSTGOXWVUHNMLKJI");
    }

    #[test]
    fn test_spiral_counter_clockwise() {
        let geometric = Geometric::new(GeometricTranspositionMethod::SpiralCounterClockwise, 6);
        let encrypted = geometric.encrypt(TEXT);
        assert_eq!(encrypted, "APONMLBQXWVKCRTUJDEFGHI");
    }

     */
}