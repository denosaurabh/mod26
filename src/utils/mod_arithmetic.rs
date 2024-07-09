pub struct ModArithmetic;

impl ModArithmetic {
    pub fn modm(p: i32, m: i32) -> i32 {
        p.rem_euclid(m)
    }

    pub fn modm_u32(p: u32, m: u32) -> u32 {
        p.rem_euclid(m)
    }

    pub fn add(p: i32, k: i32, m: i32) -> i32 {
        Self::modm(p + k, m)
    }
    pub fn add_usize(p: usize, k: i32, m: usize) -> usize {
        Self::add(p as i32, k, m as i32) as usize
    }


    pub fn mult(p: i32, k: i32, m: i32) -> i32 {
        Self::modm(p * k, m)
    }
    pub fn mult_usize(p: usize, k: i32, m: usize) -> usize {
        Self::mult(p as i32, k, m as i32) as usize
    }

    pub fn div(p: i32, x: i32, k: i32) -> i32 {
        if let Ok(xi) = Self::mod_inverse(x, k) {
            return Self::mult(p, xi, k);
        } else {
            panic!("Modular inverse does not exist");
        }
    }
    pub fn div_usize(p: usize, k: i32, m: usize) -> usize {
        Self::div(p as i32, k, m as i32) as usize
    }
    
    pub fn pow(p: u64, e: u64, m: u64) -> u64 {
        ModArithmetic::pow_u128(p as u128, e as u128, m as u128) as u64
    }

    pub fn pow_u128(p: u128, e: u128, m: u128) -> u128 {
        let mut result = 1;
        let mut base = p % m;
        let mut exponent = e;

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % m;
            }
            exponent /= 2;
            base = (base * base) % m;
        }

        result
    }

    // Extended Euclidean Algorithm
    pub fn mod_inverse_i128(k: i128, m: i128) -> Result<i128, &'static str> {
        let mut t = 0;
        let mut newt = 1;
        let mut r = m;
        let mut newr = k.abs();
    
        while newr != 0 {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }
    
        if r > 1 {
            Err("a and m are not coprime, inverse doesn't exist")
        } else {
            Ok((if t < 0 { t + m } else { t }) % m)
        }
    }



    pub fn mod_inverse(k: i32, m: i32) -> Result<i32, &'static str> {
        match Self::mod_inverse_i128(k as i128, m as i128) {
            Ok(x) => Ok(x as i32),
            Err(e) => Err(e),
        }
    }

    pub fn euclidean_gcd(a: i32, b: i32) -> i32 {
        if b == 0 { return a; }
        if a == 0 { return b }
        if a < b  { return Self::euclidean_gcd(b, a) }

        let r = a % b;
        if r == 0 { return b; }

        Self::euclidean_gcd(b, r)
    }

    pub fn euclidean_gcd_u128(a: u128, b: u128) -> u128 {
        if b == 0 { return a; }
        if a == 0 { return b }
        if a < b  { return Self::euclidean_gcd_u128(b, a) }

        let r = a % b;
        if r == 0 { return b; }

        Self::euclidean_gcd_u128(b, r)
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn modm() {
        assert_eq!(super::ModArithmetic::modm(0, 10), 0);
        assert_eq!(super::ModArithmetic::modm(10, 10), 0);
        assert_eq!(super::ModArithmetic::modm(1, 10), 1);
        assert_eq!(super::ModArithmetic::modm(-1, 10), 9);
        assert_eq!(super::ModArithmetic::modm(5, 10), 5);
    }

    #[test]
    fn test_add() {
        assert_eq!(super::ModArithmetic::add(5, 3, 26), 8);
        assert_eq!(super::ModArithmetic::add(25, 3, 26), 2);
    }

    #[test]
    fn test_add_usize() {
        assert_eq!(super::ModArithmetic::add_usize(5, 3, 26), 8);
        assert_eq!(super::ModArithmetic::add_usize(25, 3, 26), 2);
    }

    #[test]
    fn test_mult() {
        assert_eq!(super::ModArithmetic::mult(5, 3, 26), 15);
        assert_eq!(super::ModArithmetic::mult(25, 3, 26), 23);
    }

    #[test]
    fn test_mult_usize() {
        assert_eq!(super::ModArithmetic::mult_usize(5, 3, 26), 15);
        assert_eq!(super::ModArithmetic::mult_usize(25, 3, 26), 23);

        assert_eq!(super::ModArithmetic::mult_usize(1, 10, 10), 0);
    }

    #[test]
    fn test_div() {
        assert_eq!(super::ModArithmetic::div(3, 9, 26), 9);
        assert_eq!(super::ModArithmetic::div(3, 21, 26), 15);
    }

    #[test]
    fn test_div_usize() {
        assert_eq!(super::ModArithmetic::div_usize(5, 3, 26), 19);
        assert_eq!(super::ModArithmetic::div_usize(25, 3, 26), 17);
    }

    #[test]
    fn mod_inverse() {
        assert_eq!(super::ModArithmetic::mod_inverse(3, 26), Ok(9));
        assert_eq!(super::ModArithmetic::mod_inverse(21, 26), Ok(5));
    }

    #[test]
    fn euclidean_gcd() {
        assert_eq!(super::ModArithmetic::euclidean_gcd(756, 210), 42);
        assert_eq!(super::ModArithmetic::euclidean_gcd(26, 6), 2);
        assert_eq!(super::ModArithmetic::euclidean_gcd(26, 3), 1);
    }


}
