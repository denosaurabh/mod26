use primes::{PrimeSet, Sieve};

use crate::utils::{char_set::{CharSet, Converter}, mod_arithmetic::ModArithmetic};


struct TPPPohigHellman {
    char_set: CharSet,

    a: u64, // private key of Alice
    b: u64, // private key of Bob

    m: u64, // modulus
}


impl TPPPohigHellman {
    pub fn new(char_set: CharSet, a: u64, b: u64, m: u64) -> Result<Self, String> {
        // let m = Self::calculate_n(&char_set)?;
        Ok(Self { char_set, a, b, m })
    }

    fn calculate_n(char_set: &CharSet) -> Result<u64, String> {
        let n = char_set.len().to_string().repeat(2).parse::<u64>()
            .map_err(|e| format!("Failed to parse n: {}", e))?;
        Ok(Self::next_prime(n))
    }

    fn next_prime(n: u64) -> u64 {
        Sieve::new().find(n).1
    }

    /// send message from Alice to Bob
    pub fn send_message(&self, message: &str) -> Result<String, String> {
        
        println!("MODULUS: {:?}", self.m);
        
        // convert the message to a vector of numbers
        let convertor = Converter::new(self.char_set.clone());
        let num_vec = convertor.convert_to_numvec(message)?;

        println!("Message: {:?}", num_vec);

        // 1. Alice encrypts the message with her private key
        let mut alice_encrypted_message = Vec::new();
        for n in num_vec.iter() {
            let c = ModArithmetic::pow(*n as u64, self.a, self.m);
            alice_encrypted_message.push(c);
        };
        println!("Alice encrypted message: {:?}", alice_encrypted_message);

        // 2. Bob encrypts the message with his private key
        let mut bob_encrypted_message = Vec::new();
        for c in alice_encrypted_message.iter() {
            let c = ModArithmetic::pow(*c as u64, self.b, self.m);
            bob_encrypted_message.push(c);
        };

        println!("Bob encrypted message: {:?}", bob_encrypted_message);

        // 3. Alice decrypts the message with her private key
        let mut alice_decrypted_message = Vec::new();
        let a_inv = ModArithmetic::pow_inverse(self.a as u128, self.m as u128).unwrap() as u64;
        println!("A inverse: {:?}", a_inv);

        for c in bob_encrypted_message.iter() {
            let c = ModArithmetic::pow(*c as u64, a_inv, self.m);
            alice_decrypted_message.push(c);
        };

        println!("Alice decrypted message: {:?}", alice_decrypted_message);


        // 4. Bob decrypts the message with his private key
        let mut bob_decrypted_message: Vec<u32> = Vec::new();
        let b_inv = ModArithmetic::pow_inverse(self.b as u128, self.m as u128).unwrap() as u64;
        println!("B inverse: {:?}", b_inv);

        for c in alice_decrypted_message.iter() {
            let c = ModArithmetic::pow(*c as u64, b_inv, self.m);
            bob_decrypted_message.push(c as u32);
        };

        println!("Bob decrypted message: {:?}", bob_decrypted_message);

        // so Bob, finally gets the plaintext
        let decrypted_message = convertor.numvec_to_string(bob_decrypted_message);

        Ok(decrypted_message)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message() {
        let char_set = CharSet::from_alphabet_lowercase();
        let tpp_pohlig_hellman = TPPPohigHellman::new(char_set, 113, 87, 2819).expect("Failed to create TPPPohigHellman");

        // let message = "tellmethreetimes";
        let message = "ufmmnfuisffujnft";
        assert_eq!(tpp_pohlig_hellman.send_message(message).unwrap(), message);
    }
}
