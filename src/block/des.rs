/*===================================================================
 * Implementation of DES (Data Encryption Standard)
 * Author: denosauabh
 * Reference: https://en.wikipedia.org/wiki/Data_Encryption_Standard 
 * Description: Implementation of DES using paper - https://csrc.nist.gov/files/pubs/fips/46-3/final/docs/fips46-3.pdf

 *===================================================================*/

use std::usize;


pub struct DES {
    key: u64,
    round_keys: [u64; 16],
}

impl DES {
    /* KEY GENERATION */
    // PC-1
    const KEY_INITIAL_PERMUTATION: [u8; 56] = [
        57, 49, 41, 33, 25, 17, 9,
        1, 58, 50, 42, 34, 26, 18,
        10, 2, 59, 51, 43, 35, 27,
        19, 11, 3, 60, 52, 44, 36,
        63, 55, 47, 39, 31, 23, 15,
        7, 62, 54, 46, 38, 30, 22,
        14, 6, 61, 53, 45, 37, 29,
        21, 13, 5, 28, 20, 12, 4
    ];

    const KEY_LEFT_SHIFT_TABLE : [u8; 16] = [
        1, 1, 2, 2, 2, 2, 2, 2,
        1, 2, 2, 2, 2, 2, 2, 1
    ];

    const KEY_COMPRESSION_PERMUTATION: [u8; 48] = [
        14, 17, 11, 24, 1, 5,
        3, 28, 15, 6, 21, 10,
        23, 19, 12, 4, 26, 8,
        16, 7, 27, 20, 13, 2,
        41, 52, 31, 37, 47, 55,
        30, 40, 51, 45, 33, 48,
        44, 49, 39, 56, 34, 53,
        46, 42, 50, 36, 29, 32
    ];


    /* DATA ENCRYPTION */

    // Initial Permutation- IP
    const INITIAL_PERMUTATION: [u8; 64] = [
        58, 50, 42, 34, 26, 18, 10, 2,
        60, 52, 44, 36, 28, 20, 12, 4,
        62, 54, 46, 38, 30, 22, 14, 6,
        64, 56, 48, 40, 32, 24, 16, 8,
        57, 49, 41, 33, 25, 17, 9, 1,
        59, 51, 43, 35, 27, 19, 11, 3,
        61, 53, 45, 37, 29, 21, 13, 5,
        63, 55, 47, 39, 31, 23, 15, 7
    ];

    // Final Permutation/Inverse Intial-Permutation - FP
    const FINAL_PERMUTATION: [u8; 64] = [
        40, 8, 48, 16, 56, 24, 64, 32,
        39, 7, 47, 15, 55, 23, 63, 31,
        38, 6, 46, 14, 54, 22, 62, 30,
        37, 5, 45, 13, 53, 21, 61, 29,
        36, 4, 44, 12, 52, 20, 60, 28,
        35, 3, 43, 11, 51, 19, 59, 27,
        34, 2, 42, 10, 50, 18, 58, 26,
        33, 1, 41, 9, 49, 17, 57, 25
    ];

    const ROUND_HALF_BLOCK_EXPANSION: [u8; 48] = [
        32, 1, 2, 3, 4, 5,
        4, 5, 6, 7, 8, 9,
        8, 9, 10, 11, 12, 13,
        12, 13, 14, 15, 16, 17,
        16, 17, 18, 19, 20, 21,
        20, 21, 22, 23, 24, 25,
        24, 25, 26, 27, 28, 29,
        28, 29, 30, 31, 32, 1
    ];

    const ROUND_FINAL_PERMUTATION: [u8; 32] = [
        16, 7, 20, 21, 29, 12, 28, 17,
        1, 15, 23, 26, 5, 18, 31, 10,
        2, 8, 24, 14, 32, 27, 3, 9,
        19, 13, 30, 6, 22, 11, 4, 25
    ];

    const S_BOXES: [[[u8; 16]; 4]; 8] = [
        [
            // S1
            [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
            [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
            [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
            [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13]
        ],
        [
            // S2
            [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
            [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
            [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
            [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9]
        ],
        [
            // S3
            [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
            [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
            [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
            [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12]
        ],
        [
            // S4
            [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
            [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
            [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
            [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14]
        ],
        [
            // S5
            [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
            [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
            [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
            [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3]
        ],
        [
            // S6
            [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
            [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
            [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
            [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13]
        ],
        [
            // S7
            [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
            [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
            [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
            [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12]
        ],
        [
            // S8
            [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
            [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
            [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
            [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11]
        ]
            ];


    pub fn new(
        key: u64
    ) -> Self {
        Self {key, round_keys: DES::generate_roundkeys(key)}
    }

    pub fn generate_roundkeys(
        key: u64
    ) -> [u64; 16] {
        // key after intial permutation
        let mut pc1: u64 = 0;

        for &i in Self::KEY_INITIAL_PERMUTATION.iter() {
            pc1 <<= 1;
            pc1 |= (key >> (64 - i)) & 1;
        }

        // 16 rounds
        let mut round_keys: [u64; 16] = [0; 16];

        let mut c: u32 = (pc1 >> 28) as u32; 
        let mut d: u32 = (pc1 & 0x0FFFFFFF) as u32;


        for i in 0..16 {
            // rotate left
            let shift: usize = Self::KEY_LEFT_SHIFT_TABLE[i] as usize;

            c = (c << shift | c >> (28 - shift)) & 0x0FFFFFFF;
            d = (d << shift | d >> (28 - shift)) & 0x0FFFFFFF;

            // combine & compress c & d
            round_keys[i] = 0;

            for &j in Self::KEY_COMPRESSION_PERMUTATION.iter() {
                round_keys[i] <<= 1;
                round_keys[i] |= ((((c as u64) << 28) | d as u64) >> (56  - j)) & 1;
            }
        }

        round_keys
    }

    pub fn round_function(half_block: u32, subkey: u64) -> u32 {
        // Expansion
        let mut expanded: u64 = 0;
        for &i in Self::ROUND_HALF_BLOCK_EXPANSION.iter() {
            expanded <<= 1;
            expanded |= ( (half_block >> (32 - i)) & 1 ) as u64;
        }
        // println!("Expanded: 0x{:016X}", expanded);

        // XOR with subkey
        expanded ^= subkey;
        // println!("XOR: 0x{:016X}", expanded);

        // S-Boxes
        let mut sboxed: u32 = 0;
        for i in 0..8 {
            let chunk: u8 = ((expanded >> (42 - 6 * i)) & 0x3F) as u8;
            let row = (((chunk & 0x20) >> 4) | (chunk & 0x01)) as usize;
            let col = ((chunk & 0x1E) >> 1) as usize;

            let s = Self::S_BOXES[i][row][col];
            sboxed = (sboxed << 4) | s as u32;
        }
        // println!("S-Box: 0x{:016X}", sboxed);

        // Permutation
        let mut permuted: u32 = 0;
        for &i in Self::ROUND_FINAL_PERMUTATION.iter() {
            permuted <<= 1;
            permuted |= (sboxed >> (32 - i)) & 1;
        }
        // println!("Permuted: 0x{:016X}", permuted);

        permuted
    }


    pub fn execute(&self, input: u64, decrypt: bool) -> u64 {
        // Step 1: Initial Permutation
        let mut ip: u64 = 0;
        for &i in Self::INITIAL_PERMUTATION.iter() {
            ip <<= 1;
            ip |= (input >> (64 - i)) & 1;
        }

        // println!("IP: 0x{:016X}", ip);

        // Step 2: 16 Rounds
        let mut l: u32 = (ip >> 32) as u32;
        let mut r: u32 = (ip & 0xFFFFFFFF) as u32;

        // println!("L: 0x{:016X}", l);
        // println!("R: 0x{:016X}", r);

        for _i in 0..16 {

            // println!("\n");
            // println!("Round {}", _i);


            let i = if decrypt { 15 - _i } else { _i };

            let temp: u32 = r;

            r = l ^ Self::round_function(r, self.round_keys[i]);
            // println!("L: 0x{:016X}, R: 0x{:016X}", l, r);


            l = temp;
        }

        // Swap
        std::mem::swap(&mut l, &mut r);
        
        // Step 3: Final Permutation
        let combined: u64 = ((l as u64) << 32) | r as u64;

        // println!("FP: 0x{:016X}", combined);

        let mut result: u64 = 0;
        for &i in Self::FINAL_PERMUTATION.iter() {
            result <<= 1;
            result |= (combined >> (64 - i)) & 1;
        }

        // println!("Result: 0x{:016X}", result);

        result
    }

    pub fn encrypt(&self, input: u64) -> u64 {
        self.execute(input, false)
    }

    pub fn decrypt(&self, input: u64) -> u64 {
        self.execute(input, true)
    }
}


fn print_u64_array_hex(arr: &[u64; 16]) {
    for (i, &value) in arr.iter().enumerate() { 
        println!("arr[{:2}] = 0x{:016X}", i, value);
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_roundkeys() {
        let des = DES::new(
            u64::from_str_radix("AABB09182736CCDD", 16).expect("Invalid key")
        );

        print_u64_array_hex(&des.round_keys);

        let res: [u64; 16] = [
            27817705397900,
            76314457062606,
            7617739814325,
            239886861561571,
            116162390051091,
            212843790550878,
            123741545542592,
            58240342738541,
            145939842194636,
            2707289585087,
            120213461761189,
            214138100534259,
            169063126386975,
            40800239425488,
            56284570821485,
            26510106150509
        ];

        assert_eq!(des.round_keys, res);
    }


    #[test]
    fn test_encrypt() {
        let key: u64 = u64::from_str_radix("789", 16).expect("Invalid key");
        let des = DES::new(key);

        // print_u64_array_hex(&des.round_keys);

        let text: u64 = u64::from_str_radix("1234", 16).expect("Invalid plaintext");
        let ciphertext: u64 =  0xD7BB47F844A55D09;

        let encrypted = des.encrypt(text);

        assert_eq!(encrypted, ciphertext);
    }

    #[test]
    fn test_decrypt() {
        let key: u64 = u64::from_str_radix("789", 16).expect("Invalid key");
        let des = DES::new(key);

        let text: u64 = u64::from_str_radix("1234", 16).expect("Invalid plaintext");
        let ciphertext: u64 =  0xD7BB47F844A55D09;

        let decrypted = des.decrypt(ciphertext);

        assert_eq!(decrypted, text);
    }

}

