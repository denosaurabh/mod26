/*===================================================================
 * Implementation of ECB 128-bit (Electronic Code Book) Mode of Operation
 * Author: denosaurabh
 * Reference: https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation
*===================================================================*/

pub struct ECB {
    padding: u8,
    block_cipher: Box<dyn Fn(u128) -> u128>,
}

impl ECB {
    pub fn new(
        padding: u8,
        block_cipher: impl Fn(u128) -> u128 + 'static,
    ) -> ECB {
        ECB {
            padding,
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

                if chunk.len() < 16 {
                    block[chunk.len()..].fill(self.padding);
                }

                u128::from_be_bytes(block)
            })
            .collect();

        // Encrypt each block
        let encrypted_blocks: Vec<u128> = blocks
            .iter()
            .map(|&block| (self.block_cipher)(block))
            .collect();

        // Convert the vec to a hex string and return it
        encrypted_blocks
            .iter()
            .map(|&block| format!("{:032x}", block))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::aes::AES;

    #[test]
    fn test_ecb() {
        let key = 0x0123456789abcdef0123456789abcdef;
        let aes = AES::new(key);
        let ecb = ECB::new(0x00, move |block| aes.encrypt(block));

        let plaintext = "Attack Berlin at Dawn";
        let ciphertext = ecb.encrypt(plaintext);
        
        let expected = "c877851e6ac018a48678f3ea4c628d58db7a30bdbb32cf9cd7422f1ab98bb62b";
        assert_eq!(ciphertext, expected);
    }
}
