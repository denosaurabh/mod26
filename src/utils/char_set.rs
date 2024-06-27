/*===================================================================

 * Author: denosauabh
 * Description: Character set utility, made to create a reduced character set (rather than entire unicode),
                making it easier to study ciphers.

 *===================================================================*/

use std::collections::HashSet;

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

    pub fn from_alphabet_smallcase() -> Self {
        Self::from_range('a', 'z')
    }

    pub fn from_string(s: &str) -> Self {
        let mut chars: Vec<char> = s.chars().collect();
        
        // remove dublicates
        let mut seen = HashSet::new();
        chars.retain(|item| seen.insert(item.clone()));

        Self { chars }
    }

    pub fn from_numbers() -> Self {
        Self::from_string("0123456789")
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

    pub fn index_of(&self, c: char) -> Option<usize> {
        if let Some(index) = self.chars.iter().position(|&x| x == c) {
            Some(index)
            // Some((index as u32 + 1) as usize)
        } else {
            None
        }
    }

    pub fn char_at(&self, index: usize) -> Option<char> {
        // if index == 0 {
        //     // panic!("Index begins at 1, not 0!")
        //     // return Some('\u{FFFD}');
        //     return None;    
        // }

        self.chars.get(index).cloned()
        // self.chars.get(index-1).cloned()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let charset = CharSet::new();
        assert_eq!(charset.len(), 95); // 127 - 32 = 95 printable ASCII characters
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
        let charset = CharSet::from_unicode(0x0600, 0x0610); // Arabic supplement range
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
        assert_eq!(charset.index_of('a'), Some(0));
        assert_eq!(charset.index_of('c'), Some(2));
        assert_eq!(charset.index_of('z'), None);


        let charset_num = CharSet::from_string("1234567890");
        assert_eq!(charset_num.index_of('1'), Some(0));
        assert_eq!(charset_num.index_of('0'), Some(9));
    }

    #[test]
    fn test_char_at() {
        let charset = CharSet::from_string("abcdef");
        assert_eq!(charset.char_at(1), Some('b'));
        assert_eq!(charset.char_at(10), None);


        let charset_num = CharSet::from_numbers();
        assert_eq!(charset_num.char_at(1), Some('1'));
        assert_eq!(charset_num.char_at(9), Some('9'));
    }
    
}