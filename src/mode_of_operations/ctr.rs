/*===================================================================
 * Implementation of CTR (Counter) Mode of Operation
 * Author: denosaurabh
 * Reference: https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation
*===================================================================*/

pub struct CTR {
    nonce: u64,
    counter: u64,
    block_cipher: Box<dyn Fn(u128) -> u128>,
}

impl CTR {
    pub fn new(
        nonce: u64,
        block_cipher: impl Fn(u128) -> u128 + 'static,
    ) -> CTR {
        CTR {
            nonce,
            counter: 0,
            block_cipher: Box::new(block_cipher),
        }
    }

    fn get_keystream_block(&mut self) -> u128 {
        let input = ((self.nonce as u128) << 64) | (self.counter as u128);
        self.counter = self.counter.wrapping_add(1);
        (self.block_cipher)(input)
    }

    pub fn process(&mut self, data: &str) -> String {
        let blocks: Vec<u128> = data
            .as_bytes()
            .chunks(16)
            .map(|chunk| {
                let mut block = [0u8; 16];
                block[..chunk.len()].copy_from_slice(chunk);
                u128::from_be_bytes(block)
            })
            .collect();

        let processed_blocks: Vec<u128> = blocks
            .iter()
            .map(|&block| block ^ self.get_keystream_block())
            .collect();

        processed_blocks
            .iter()
            .map(|&block| format!("{:032x}", block))
            .collect()
    }

    // Encryption and decryption are the same operation in CTR mode
    pub fn encrypt(&mut self, plaintext: &str) -> String {
        self.process(plaintext)
    }

    pub fn decrypt(&mut self, ciphertext: &str) -> Result<String, Box<dyn std::error::Error>> {
        let encrypted_blocks: Result<Vec<u128>, _> = ciphertext
            .chars()
            .collect::<Vec<char>>()
            .chunks(32)
            .map(|chunk| u128::from_str_radix(&chunk.iter().collect::<String>(), 16))
            .collect();

        let encrypted_blocks = encrypted_blocks?;

        let decrypted_blocks: Vec<u128> = encrypted_blocks
            .iter()
            .map(|&block| block ^ self.get_keystream_block())
            .collect();

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
    fn test_ctr() {
        let key = 0x0123456789abcdef0123456789abcdef;
        let nonce = 0xfedcba9876543210;

        let aes = Rc::new(AES::new(key));
        let mut ctr = CTR::new(
            nonce,
            {
                let aes = Rc::clone(&aes);
                move |block| aes.encrypt(block)
            }
        );

        let plaintext = "Attack Berlin at Dawn";
        let ciphertext = ctr.encrypt(plaintext);
        
        let expected = "8c2e91d8d8ec52a2005e538da956dc8fd1a7c2053dc31563a5179580ca1476c2";
        assert_eq!(ciphertext, expected);

        // Reset the CTR instance for decryption
        let mut ctr = CTR::new(nonce, move |block| aes.encrypt(block));

        // Test decryption
        let decrypted = ctr.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
