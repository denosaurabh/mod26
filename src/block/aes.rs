/*===================================================================
 * Implementation of AES 128-bit (Advanced Encryption Standard)
 * Author: denosauabh
 * Reference: https://en.wikipedia.org/wiki/Advanced_Encryption_Standard
 * Description: Implementation of AES using paper - https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197-upd1.pdf
 
 *===================================================================*/

use std::u128;


pub struct AES {
    key: [u32; 4],
    round_keys: [u128; 11],
}

impl AES {

    const AES_SBOX: [[u8;16];16] = [
    [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16] ];

    const INVERSE_AES_SBOX: [[u8;16];16] = [
    [0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb],
    [0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb],
    [0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e],
    [0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25],
    [0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92],
    [0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84],
    [0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06],
    [0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b],
    [0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73],
    [0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e],
    [0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b],
    [0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4],
    [0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f],
    [0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef],
    [0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61],
    [0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d] ];

    const RC: [u8;11] = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];


    pub fn new(key: u128) -> AES {
        let key_u32_arr = Self::u128_to_u32_array(key); 

        Self {
            key: key_u32_arr, 
            round_keys: Self::key_expansion(key),
        }
    }


    pub fn u128_to_u32_array(value: u128) -> [u32; 4] {
        [
            (value >> 96) as u32,
            (value >> 64) as u32,
            (value >> 32) as u32,
            value as u32,
        ]
    }

    pub fn u32_array_to_u128(arr: [u32; 4]) -> u128 {
        ((arr[0] as u128) << 96) | ((arr[1] as u128) << 64) | ((arr[2] as u128) << 32) | (arr[3] as u128)
    }

    pub fn rotate_left(val: u32) -> u32 {
        // ((val << n) | (val >> (32 - n))

        let u8_arr = val.to_be_bytes();

        let mut new_u8_arr: [u8; 4] = [0; 4];
        for i in 0..4 {
            new_u8_arr[i as usize] = u8_arr[(i + 1) % 4];
        }

        let new_val = u32::from_be_bytes(new_u8_arr);
        new_val
    }

    pub fn s_box(val: u8) -> u8 {
        Self::AES_SBOX[(val >> 4) as usize][(val & 0x0F) as usize]
    }

    // utils
    pub fn print_u32_bits(val: u32) {
        println!("{:032b}", val);
    }

    pub fn print_u32_hex(val: u32) {
        println!("0x{:016X}", val);
    }

    pub fn print_u32(text: &str, val: u32) {
        println!("{}: {:032b} 0x{:016X}", text, val, val);
    }

    pub fn print_u128(text: &str, val: u128) {
        println!("{}: 0x{:016X}", text, val);
    }

    pub fn key_expansion(_key: u128) -> [u128; 11] {
        let mut round_keys: [u128; 11] = [0; 11];

        round_keys[0] = _key;

        for i in 1..11 {
            let r = AES::u128_to_u32_array(round_keys[i-1]);
            
            let mut temp: u32 = r[3];

            // rotate left
            temp = Self::rotate_left(temp);

            // s-box
            let mut temp_after_sbox: u32 = 0;
            for j in 0..4 {
                let pos = (temp >> (8 * j)) as u8;
                let res = Self::s_box(pos) as u32;
                let shiftback: u32 = res << (8 * j);

                temp_after_sbox = temp_after_sbox | shiftback;
            }

            // round constant
            temp_after_sbox = temp_after_sbox ^ ((AES::RC[i] as u32) << 24);

            // xor
            let mut round_key: [u32; 4] = [0; 4];
            round_key[0] = r[0] ^ temp_after_sbox;
            round_key[1] = r[1] ^ round_key[0];
            round_key[2] = r[2] ^ round_key[1];
            round_key[3] = r[3] ^ round_key[2];

            round_keys[i] = AES::u32_array_to_u128(round_key);
        }

        round_keys
    }

    pub fn cipher(&self, input: u128) -> u128 {
        let mut state: u128 = input;

        state ^= self.round_keys[0];

        for i in 1..(11-1) {
            println!("ROUND: {}", i);

            state = self.sub_bytes(state); // s-boxes
            println!("sub_bytes:    0x{:016X}", state);

            state = self.shift_rows(state); // transposition
            println!("shift_rows:   0x{:016X}", state);

            state = self.mix_columns(state); // hill cipher
            println!("mix_columns:  0x{:016X}", state);

            state ^= self.round_keys[i]; // add round key
            println!("round_key:    0x{:016X}", state);
        }

        state = self.sub_bytes(state);
        state = self.shift_rows(state);
        state ^= self.round_keys[10];

        state
    }

    pub fn sub_bytes(&self, state: u128) -> u128 {
        let mut new_state: [u8; 16] = state.to_be_bytes();

        for i in 0..16 {
            new_state[i] = Self::s_box(new_state[i]); 
        }

        u128::from_be_bytes(new_state)
    }

    pub fn shift_rows(&self, state: u128) -> u128 {
        let mut new_state: u128 = 0;

        // collect rows
        let mut rows: [[u8; 4]; 4] = [[0; 4]; 4];

        for i in 0..4 {
            let mut row: [u8; 4] = [0; 4];
            row[0] = (state >> (128 - 8 - i*8))   as u8;
            row[1] = (state >> (128 - 40 - i*8))  as u8;
            row[2] = (state >> (128 - 72 - i*8))  as u8;
            row[3] = (state >> (128 - 104 - i*8)) as u8;
             
            rows[i] = row;
        }

        // shift rows
        for i in 0..4 {
            let mut row: [u8; 4] = rows[i];
            let mut new_row: [u8; 4] = [0; 4];

            for j in 0..4 {
                new_row[j] = row[(j + i) % 4];
            }

            rows[i] = new_row;
        }

        for j in 0..4 {
            for i in 0..4 {
                new_state = new_state | ((rows[i][j] as u128) << (128 - 8 - i*8 - j*32));
            }
        }

        new_state
    }

    pub fn mix_columns(&self, state: u128) -> u128 {
        let mut new_state: u128 = 0;

        // collect columns
        let mut columns: [[u8; 4]; 4] = [[0; 4]; 4];

        for i in 0..4 {
            columns[i][0] = (state >> (128 - 8  - (i*32))) as u8;
            columns[i][1] = (state >> (128 - 16 - (i*32))) as u8;
            columns[i][2] = (state >> (128 - 24 - (i*32))) as u8;
            columns[i][3] = (state >> (128 - 32 - (i*32))) as u8;
        }

        // mix columns
        for i in 0..4 {
            let column: [u8; 4] = columns[i];
            let mut new_column: [u8; 4] = [0; 4];

            new_column[0] = Self::mul(0x02, column[0]) ^ Self::mul(0x03, column[1]) ^ column[2] ^ column[3];
            new_column[1] = column[0] ^ Self::mul(0x02, column[1]) ^ Self::mul(0x03, column[2]) ^ column[3];
            new_column[2] = column[0] ^ column[1] ^ Self::mul(0x02, column[2]) ^ Self::mul(0x03, column[3]);
            new_column[3] = Self::mul(0x03, column[0]) ^ column[1] ^ column[2] ^ Self::mul(0x02, column[3]);

            columns[i] = new_column;
        }

        // update state
        for j in 0..4 {
            for i in 0..4 {
                new_state = new_state | ((columns[j][i] as u128) << (128 - 8 - i*8 - j*32));
            }
        }

        new_state
    }

    pub fn mul(a: u8, b: u8) -> u8 {
        let mut product: u8 = 0;
        let mut a_copy = a;
        let mut b_copy = b;
        let modulo: u8 = 0x1B; // x^8 + x^4 + x^3 + x + 1

        for _ in 0..8 {
            if b_copy & 1 != 0 {
                product ^= a_copy;
            }
            let high_bit_set = a_copy & 0x80 != 0;
            a_copy <<= 1;
            if high_bit_set {
                a_copy ^= modulo;
            }
            b_copy >>= 1;
        }

        product
    }



    pub fn encrypt(&self, input: u128) -> u128 {
        self.cipher(input)
    }

    pub fn decrypt(&self, input: u128) -> u128 {
        self.cipher(input)
    }

}




fn print_arr_u128_hex(arr: [u128; 11]) {
    for i in 0..11 {
        println!("arr[{:2}] = 0x{:016X}", i, arr[i]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_expansion() {
        let key  = 0x2b7e151628aed2a6abf7158809cf4f3c;
        let round_keys = AES::key_expansion(key);

        print_arr_u128_hex(round_keys);

        assert_eq!(round_keys[0], 0x2b7e151628aed2a6abf7158809cf4f3c);
        assert_eq!(round_keys[1], 0xa0fafe1788542cb123a339392a6c7605);
        assert_eq!(round_keys[2], 0xf2c295f27a96b9435935807a7359f67f);
        assert_eq!(round_keys[3], 0x3d80477d4716fe3e1e237e446d7a883b);
        // TODO: Add more tests.....
    }

    #[test]
    fn test_aes() {
        let key  = 0x2b7e151628aed2a6abf7158809cf4f3c;
        let text = 0x3243f6a8885a308d313198a2e0370734;

        let aes = AES::new(key);

        print_arr_u128_hex(aes.round_keys);

        let output = aes.encrypt(text);

        assert_eq!(output, 0x3925841d02dc09fbdc118597196a0b32);
    }
}
