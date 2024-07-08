/*===================================================================
 * Implementation of CBC (Cipher Block Chaining) Mode of Operation
 * Author: denosaurabh
 * Reference: https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation
*===================================================================*/

pub struct CBC {
    iv: u128,
    block_cipher: Box<dyn Fn(u128) -> u128>,
    block_decipher: Box<dyn Fn(u128) -> u128>,
}

impl CBC {
    pub fn new(
        iv: u128,
        block_cipher: impl Fn(u128) -> u128 + 'static,
        block_decipher: impl Fn(u128) -> u128 + 'static,
    ) -> CBC {
        CBC {
            iv,
            block_cipher: Box::new(block_cipher),
            block_decipher: Box::new(block_decipher),
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
            let xored_block = block ^ previous_block;
            let encrypted = (self.block_cipher)(xored_block);
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
            let decrypted = (self.block_decipher)(encrypted_block);
            let plaintext_block = decrypted ^ previous_block;
            decrypted_blocks.push(plaintext_block);
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
    use std::rc::Rc;

    use super::*;
    use crate::block::aes::AES;

    #[test]
    fn test_cbc() {
        let key = 0x0123456789abcdef0123456789abcdef;
        let iv = 0xfedcba9876543210fedcba9876543210;
        let aes = Rc::new(AES::new(key));
        let cbc = CBC::new(
            iv,
            {
                let aes = Rc::clone(&aes);
                move |block| aes.encrypt(block)
            },
            move |block| aes.decrypt(block)
        );

        let plaintext = "Attack Berlin at Dawn";
        let ciphertext = cbc.encrypt(plaintext);
        
        let expected = "b9844b42a00e0a83cbb0ccc346daceeb658fcf011af943fc02a8fc61050ff1a0";
        assert_eq!(ciphertext, expected);

        // Test decryption
        let decrypted = cbc.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
