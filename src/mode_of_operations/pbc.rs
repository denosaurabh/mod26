/*===================================================================
 * Implementation of PBC (Plaintext Block Chaining) Mode of Operation
 * Author: denosaurabh 
 * Reference: https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation
*===================================================================*/

pub struct PBC {
    iv: u128,
    block_cipher: Box<dyn Fn(u128) -> u128>,
}

impl PBC {
    pub fn new(
        iv: u128,
        block_cipher: impl Fn(u128) -> u128 + 'static,
    ) -> PBC {
        PBC {
            iv,
            block_cipher: Box::new(block_cipher),
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let plaintext_blocks: Vec<u128> = plaintext
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

        for &plaintext in &plaintext_blocks {
            let chained_block = plaintext ^ previous_block;
            let encrypted = (self.block_cipher)(chained_block);
            encrypted_blocks.push(encrypted);
            previous_block = plaintext;
        }

        encrypted_blocks
            .iter()
            .map(|&block| format!("{:032x}", block))
            .collect()
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String, Box<dyn std::error::Error>> {
        todo!("Implement PBC decryption")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::aes::AES;

    #[test]
    fn test_pbc() {
        let key = 0x0123456789abcdef0123456789abcdef;
        let iv = 0xfedcba9876543210fedcba9876543210;
        let aes = AES::new(key);
        let pbc = PBC::new(iv, move |block| aes.encrypt(block));

        let plaintext = "Attack Berlin at Dawn";
        let ciphertext = pbc.encrypt(plaintext);
        
        let expected = "b9844b42a00e0a83cbb0ccc346daceeb379c79a735a90c4c9af847dc98b546c8";
        assert_eq!(ciphertext, expected);

        // Test decryption
        // let decrypted = pbc.decrypt(&ciphertext).unwrap();
        // assert_eq!(decrypted, plaintext);
    }
}
