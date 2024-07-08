/*===================================================================
 * Implementation of CFB (Ciphertext Feedback) Mode of Operation
 * Author: denosaurabh
 * Reference: https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation
*===================================================================*/

pub struct CFB {
    iv: u128,
    block_cipher: Box<dyn Fn(u128) -> u128>,
}

impl CFB {
    pub fn new(
        iv: u128,
        block_cipher: impl Fn(u128) -> u128 + 'static,
    ) -> CFB {
        CFB {
            iv,
            block_cipher: Box::new(block_cipher),
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let blocks: Vec<u128> = plaintext
            .as_bytes()
            .chunks(16)
            .map(|chunk| {
                let mut block = [0u8; 16];
                block[..chunk.len()].copy_from_slice(chunk);
                u128::from_be_bytes(block)
            })
            .collect();

        let mut encrypted_blocks: Vec<u128> = Vec::with_capacity(blocks.len());
        let mut previous_block = self.iv;

        for &block in &blocks {
            let encrypted_iv = (self.block_cipher)(previous_block);
            let encrypted = encrypted_iv ^ block;
            encrypted_blocks.push(encrypted);
            previous_block = encrypted;
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
            let encrypted_iv = (self.block_cipher)(previous_block);
            let decrypted = encrypted_iv ^ encrypted_block;
            decrypted_blocks.push(decrypted);
            previous_block = encrypted_block;
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
    fn test_cfb() {
        let key = 0x0123456789abcdef0123456789abcdef;
        let iv = 0xfedcba9876543210fedcba9876543210;
        let aes = AES::new(key);
        let cfb = CFB::new(iv, move |block| aes.encrypt(block));

        let plaintext = "Attack Berlin at Dawn";
        let ciphertext = cfb.encrypt(plaintext);
        
        let expected = "51981b74270cf6d8cceed5c60e7519232d289f8cd1e7b8cadaa7a09293ee7335";
        assert_eq!(ciphertext, expected);

        // Test decryption
        let decrypted = cfb.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
