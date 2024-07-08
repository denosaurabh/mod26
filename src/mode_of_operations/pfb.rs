/*===================================================================
 * Implementation of PFB (Plaintext Feedback) Mode of Operation
 * Author: denosaurabh
 * Reference: https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation
*===================================================================*/

pub struct PFB {
    iv: u128,
    block_cipher: Box<dyn Fn(u128) -> u128>,
}

impl PFB {
    pub fn new(
        iv: u128, // Initialization Vector
        block_cipher: impl Fn(u128) -> u128 + 'static,
    ) -> PFB {
        PFB {
            iv,
            block_cipher: Box::new(block_cipher),
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut plaintext_blocks: Vec<u128> = plaintext
            .as_bytes()
            .chunks(16)
            .map(|chunk| {
                let mut block = [0u8; 16];
                block[..chunk.len()].copy_from_slice(chunk);

                u128::from_be_bytes(block)
            })
            .collect();

        let mut encrypted_blocks: Vec<u128> = Vec::with_capacity(plaintext_blocks.len());
        let mut previous_block = self.iv;

        for plaintext in plaintext_blocks.iter_mut() {
            let encrypted = (self.block_cipher)(previous_block) ^ *plaintext;
            encrypted_blocks.push(encrypted);
            previous_block = *plaintext;
        }

        encrypted_blocks
            .iter()
            .map(|&block| format!("{:032x}", block))
            .collect()
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, Box<dyn std::error::Error>> {
        let encrypted_blocks: Result<Vec<u128>, _> = ciphertext
            .chars()
            .collect::<Vec<char>>()
            .chunks(32)
            .map(|chunk| u128::from_str_radix(&chunk.iter().collect::<String>(), 16))
            .collect();

        let encrypted_blocks = encrypted_blocks?;

        let mut decrypted_blocks: Vec<u128> = Vec::with_capacity(encrypted_blocks.len());
        let mut previous_block = self.iv;

        for &encrypted_block in &encrypted_blocks {
            let decrypted = (self.block_cipher)(previous_block) ^ encrypted_block;
            decrypted_blocks.push(decrypted);
            previous_block = decrypted;
        }

        Ok(decrypted_blocks
            .iter()
            .flat_map(|&block| block.to_be_bytes().to_vec())
            .take_while(|&byte| byte != 0)
            .map(|byte| byte as char)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::aes::AES;

    #[test]
    fn test_pfb() {
        let key = 0x0123456789abcdef0123456789abcdef;
        let iv = 0xfedcba9876543210fedcba9876543210;

        let aes = AES::new(key);
        let pfb = PFB::new(iv, move |block| aes.encrypt(block));

        let plaintext = "Attack Berlin at Dawn";
        let ciphertext = pfb.encrypt(plaintext);
        
        let expected = "51981b74270cf6d8cceed5c60e751923e833e46904c018a48678f3ea4c628d58";
        assert_eq!(ciphertext, expected);

        // Test decryption
        let decrypted = pfb.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
