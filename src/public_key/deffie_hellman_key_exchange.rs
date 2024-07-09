/*===================================================================
 * Simple Implementation of Deffie-Hellman Key Exchange
 * Author: denosauabh
 * Reference: https://en.wikipedia.org/wiki/Diffie–Hellman_key_exchange
 *===================================================================*/


use crate::{utils::mod_arithmetic::ModArithmetic};

pub struct DiffieHellmanKeyExchange {
    pub prime: u64,
    pub generator: u64,
}

impl DiffieHellmanKeyExchange {
    pub fn new(prime: u64, generator: u64) -> Self {
        DiffieHellmanKeyExchange {
            prime,
            generator,
        }
    }

    pub fn setup(&self, private_key_a: u64, private_key_b: u64) -> (u64, u64, u64) {
        // generate the public keys
        let A = ModArithmetic::pow(self.generator, private_key_a, self.prime);
        let B = ModArithmetic::pow(self.generator, private_key_b, self.prime);

        // generate the shared key
        let g_ab = ModArithmetic::pow(A, private_key_b, self.prime);

        (A, B, g_ab)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffie_hellman_key_agreement() {
        let diffie_hellman = DiffieHellmanKeyExchange::new(2819, 2);
        let (A, B, g_ab) = diffie_hellman.setup(94, 305);

        assert_eq!(A, 2220);
        assert_eq!(B, 1367);
        assert_eq!(g_ab, 747);
    }
}
