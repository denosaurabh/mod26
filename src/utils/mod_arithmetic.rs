pub struct ModArithmetic;

impl ModArithmetic {
    pub fn add(a: i32, x: i32, m: i32) -> i32 {
        (a + x).rem_euclid(m)
    }
    pub fn add_usize(a: usize, x: i32, m: usize) -> usize {
        Self::add(a as i32, x, m as i32) as usize
    }


    pub fn mult(a: i32, x: i32, m: i32) -> i32 {
        (a * x).rem_euclid(m)
    }
    pub fn mult_usize(a: usize, x: i32, m: usize) -> usize {
        Self::mult(a as i32, x, m as i32) as usize
    }

    pub fn div(a: i32, x: i32, m: i32) -> i32 {
        if let Some(xi) = Self::mod_inverse(x, m) {
            return Self::mult(a, xi, m);
        } else {
            panic!("Modular inverse does not exist");
        }
    }
    pub fn div_usize(a: usize, x: i32, m: usize) -> usize {
        Self::div(a as i32, x, m as i32) as usize
    }

    // Extended Euclidean Algorithm
    pub fn mod_inverse(a: i32, m: i32) -> Option<i32> {
        let mut t = 0;
        let mut newt = 1;
        let mut r = m;
        let mut newr = a.abs();
    
        while newr != 0 {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }
    
        if r > 1 {
            None  // a and m are not coprime, inverse doesn't exist
        } else {
            Some((if t < 0 { t + m } else { t }) % m)
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
}


#[cfg(test)]
mod tests {
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
        assert_eq!(super::ModArithmetic::mod_inverse(3, 26), Some(9));
        assert_eq!(super::ModArithmetic::mod_inverse(21, 26), Some(5));
    }

    #[test]
    fn euclidean_gcd() {
        assert_eq!(super::ModArithmetic::euclidean_gcd(756, 210), 42);
        assert_eq!(super::ModArithmetic::euclidean_gcd(26, 6), 2);
        assert_eq!(super::ModArithmetic::euclidean_gcd(26, 3), 1);
    }


}