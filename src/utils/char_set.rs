/*===================================================================

 * Author: denosauabh
 * Description: Character set utility, made to create a reduced character set (rather than entire unicode),
                making it easier to study ciphers.

 *===================================================================*/

use std::collections::HashSet;

#[derive(Clone)]
pub struct CharSet {
    pub chars: Vec<char>,
}

impl CharSet {
    // creation
    pub fn new() -> Self {
        Self::from_reduced_ascii()
    }
    
    pub fn from_range(start: char, end: char) -> Self {
        let chars: Vec<char> = (start as u32..=end as u32)
            .filter_map(std::char::from_u32)
            .collect();
        Self { chars }
    }

    // https://symbl.cc/en/unicode-table/#arabic-supplement
    pub fn from_unicode(start: u32, end: u32) -> Self {
        let mut chars = Vec::new();

        let mut index: usize = 0;
        for i in start..end {
            chars.insert(index as usize, std::char::from_u32(i).unwrap());
            index += 1;
        }
        Self { chars }
    }

    // https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html
    pub fn from_ascii() -> Self {
        Self::from_unicode(0, 128)
    }


    pub fn from_reduced_ascii() -> Self {
        Self::from_unicode(32, 127)
    }

    pub fn from_alphabet_lowercase() -> Self {
        Self::from_range('a', 'z')
    }

    pub fn from_numbers() -> Self {
        Self::from_string("0123456789")
    }

    pub fn from_string(s: &str) -> Self {
        let mut chars: Vec<char> = s.chars().collect();
        
        // remove dublicates
        let mut seen = HashSet::new();
        chars.retain(|item| seen.insert(item.clone()));

        Self { chars }
    }

    // methods
    pub fn validate(&self, s: &str) -> bool {
        s.chars().all(|c| self.chars.contains(&c))
    }

    pub fn panic_if_invalid(&self, s: &str) {
        if !self.validate(s) {
            panic!("Invalid character in string");
        }
    }

    pub fn contains(&self, c: char) -> bool {
        self.chars.contains(&c)
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn first(&self) -> Option<&char> {
        self.chars.iter().next()
    }

    pub fn last(&self) -> Option<&char> {
        self.chars.iter().last()
    }

    pub fn index_of(&self, c: char) -> usize {
        if let Some(index) = self.chars.iter().position(|&x| x == c) {
            index
        } else {
            panic!("character does't exist in charset");
        }
    }

    pub fn char_at(&self, index: usize) -> char {
        if let Some(c) = self.chars.get(index).cloned() {
            c
        } else {
            panic!("Index out of bounds");
        }
    }
}


// Converter
pub struct Converter {
    char_set: CharSet,
    pub pad_length: usize,
}

impl Converter {
    pub fn new(char_set: CharSet) -> Self {
        let pad_length = char_set.len().to_string().len();

        Self { char_set, pad_length }
    }

    pub fn convert_to_numvec(&self, text: &str) -> Result<Vec<u32>, String> {
        let mut numstr = Vec::with_capacity(text.len() / 2 + 1);

        for chunk in text.chars().collect::<Vec<char>>().chunks(2) {
            let p = if chunk.len() == 2 {
                let a = self.char_set.index_of(chunk[0]);
                let b = self.char_set.index_of(chunk[1]);

                format!("{:0width$}{:0width$}", a, b, width = self.pad_length).parse::<u32>()
                    .map_err(|e| format!("Failed to parse p: {}", e))?
            } else {
                let a = self.char_set.index_of(chunk[0]);

                format!("{:0width$}", a, width = self.pad_length * 2).parse::<u32>()
                    .map_err(|e| format!("Failed to parse p: {}", e))?
            };

            numstr.push(p);
        }

        Ok(numstr)
    }

    pub fn numvec_to_string(&self, numvec: Vec<u32>) -> String {
        let mut text = String::new();

        for p in numvec.iter() {
            let p_str = format!("{:0width$}", p, width = self.pad_length * 2);

            let a = p_str[..self.pad_length].parse::<usize>().unwrap();

            if a != 0 {
                text.push(self.char_set.char_at(a));
            }

            if self.pad_length * 2 == p_str.len() {
                let b = p_str[self.pad_length..].parse::<usize>().unwrap();
                text.push(self.char_set.char_at(b));
            }
        }

        text
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let charset = CharSet::new();
        assert_eq!(charset.len(), 95);
        assert_eq!(charset.first(), Some(&' '));
        assert_eq!(charset.last(), Some(&'~'));
    }

    #[test]
    fn test_from_range() {
        let charset = CharSet::from_range('A', 'Z');
        assert_eq!(charset.len(), 26);
        assert_eq!(charset.first(), Some(&'A'));
        assert_eq!(charset.last(), Some(&'Z'));
    }

    #[test]
    fn test_from_unicode() {
        let charset = CharSet::from_unicode(0x0600, 0x0610);
        assert_eq!(charset.len(), 16);
        assert_eq!(charset.first(), Some(&'\u{0600}'));
        assert_eq!(charset.last(), Some(&'\u{060F}'));
    }

    #[test]
    fn test_from_ascii() {
        let charset = CharSet::from_ascii();
        assert_eq!(charset.len(), 128);
        assert_eq!(charset.first(), Some(&'\0'));
        assert_eq!(charset.last(), Some(&'\x7F'));
    }

    #[test]
    fn test_from_reduced_ascii() {
        let charset = CharSet::from_reduced_ascii();
        assert_eq!(charset.len(), 95);
        assert_eq!(charset.first(), Some(&' '));
        assert_eq!(charset.last(), Some(&'~'));
    }

    #[test]
    fn test_from_string() {
        let charset = CharSet::from_string("hello123");
        assert_eq!(charset.len(), 7);
        assert!(charset.contains('h'));
        assert!(charset.contains('3'));
        assert!(!charset.contains('a'));
    }

    #[test]
    fn test_validate() {
        let charset = CharSet::from_range('a', 'z');
        assert!(charset.validate("hello"));
        assert!(!charset.validate("Hello"));
    }

    #[test]
    #[should_panic(expected = "Invalid character in string")]
    fn test_panic_if_invalid() {
        let charset = CharSet::from_range('a', 'z');
        charset.panic_if_invalid("Hello");
    }

    #[test]
    fn test_contains() {
        let charset = CharSet::from_string("abcdefg");
        assert!(charset.contains('a'));
        assert!(!charset.contains('z'));
    }

    #[test]
    fn test_len() {
        let charset = CharSet::from_string("hel lo");
        assert_eq!(charset.len(), 5);
    }

    #[test]
    fn test_first_last() {
        let charset = CharSet::from_string("abc");
        assert_eq!(charset.first(), Some(&'a'));
        assert_eq!(charset.last(), Some(&'c'));

        let empty_charset = CharSet::from_string("");
        assert_eq!(empty_charset.first(), None);
        assert_eq!(empty_charset.last(), None);
    }

    #[test]
    fn test_index_of() {
        let charset = CharSet::from_string("abcdef");
        assert_eq!(charset.index_of('a'), 0);
        assert_eq!(charset.index_of('c'), 2);

        let charset_num = CharSet::from_string("1234567890");
        assert_eq!(charset_num.index_of('1'), 0);
        assert_eq!(charset_num.index_of('0'), 9);
    }

    #[test]
    fn test_char_at() {
        let charset = CharSet::from_string("abcdef");
        assert_eq!(charset.char_at(1), 'b');

        let charset_num = CharSet::from_numbers();
        assert_eq!(charset_num.char_at(1), '1');
        assert_eq!(charset_num.char_at(9), '9');
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_char_at_panic() {
        let charset = CharSet::from_string("abcdef");
        charset.char_at(10);
    }

    #[test]
    #[should_panic(expected = "character does't exist in charset")]
    fn test_index_of_panic() {
        let charset = CharSet::from_string("abcdef");
        charset.index_of('z');
    }


    // Converter tests
    #[test]
    fn test_convert_to_numstr() {
        let char_set = CharSet::from_alphabet_lowercase();
        let converter = Converter::new(char_set);
        
        let result = converter.convert_to_numvec("tellmethreetimes").unwrap();
        assert_eq!(result, vec![1904, 1111, 1204, 1907, 1704, 0419, 0812, 0418]);

        let result = converter.convert_to_numvec("hello").unwrap();
        assert_eq!(result, vec![0704, 1111, 14]);

        let result = converter.convert_to_numvec("z").unwrap();
        assert_eq!(result, vec![25]);
    }

    #[test]
    fn test_convert_numvec_to_str() {
        let char_set = CharSet::from_alphabet_lowercase();
        let converter = Converter::new(char_set);
        
        let result = converter.numvec_to_string(vec![1904, 1111, 1204, 1907, 1704, 0419, 0812, 0418]);
        assert_eq!(result, "tellmethreetimes");

        let result = converter.numvec_to_string(vec![0704, 1111, 14]);
        assert_eq!(result, "hello");

        let result = converter.numvec_to_string(vec![25]);
        assert_eq!(result, "z");
    }

    #[test]
    fn test_convert_to_numstr_with_custom_charset() {
        let char_set = CharSet::from_string("0123456789abcdefghijklmnopqrstuvwxyz");
        let converter = Converter::new(char_set);
        
        let result = converter.convert_to_numvec("a1b2c3").unwrap();
        assert_eq!(result, vec![1001, 1102, 1203]);
    }

}

