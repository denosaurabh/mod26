/*
pub fn convert_to_numstr(&self, text: &str) -> Result<Vec<u32>, String> {
    let charset_len = self.char_set.len();
    let pad_length = charset_len.to_string().len();

    let mut numstr = Vec::with_capacity(text.len() / 2 + 1);

    for chunk in text.chars().collect::<Vec<char>>().chunks(2) {
        let p = if chunk.len() == 2 {
            let a = self.char_set.index_of(chunk[0])
                .ok_or_else(|| format!("Character '{}' not found in charset", chunk[0]))?;
            let b = self.char_set.index_of(chunk[1])
                .ok_or_else(|| format!("Character '{}' not found in charset", chunk[1]))?;

            format!("{:0width$}{:0width$}", a, b, width = pad_length).parse::<u32>()
                .map_err(|e| format!("Failed to parse p: {}", e))?
        } else {
            let a = self.char_set.index_of(chunk[0])
                .ok_or_else(|| format!("Character '{}' not found in charset", chunk[0]))?;

            format!("{:0width$}", a, width = pad_length * 2).parse::<u32>()
                .map_err(|e| format!("Failed to parse p: {}", e))?
        };

        numstr.push(p);
    }

    Ok(numstr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_numstr() {
        let char_set = CharSet::new("abcdefghijklmnopqrstuvwxyz").unwrap();
        let converter = YourStructName { char_set };
        
        let result = converter.convert_to_numstr("tellmethreetimes").unwrap();
        assert_eq!(result, vec![1904, 1111, 1204, 1907, 1704, 0419, 0812, 0418]);

        let result = converter.convert_to_numstr("hello").unwrap();
        assert_eq!(result, vec![0704, 1111, 1414]);

        let result = converter.convert_to_numstr("z").unwrap();
        assert_eq!(result, vec![2525]);
    }

    #[test]
    fn test_convert_to_numstr_with_custom_charset() {
        let char_set = CharSet::new("0123456789abcdefghijklmnopqrstuvwxyz").unwrap();
        let converter = YourStructName { char_set };
        
        let result = converter.convert_to_numstr("a1b2c3").unwrap();
        assert_eq!(result, vec![1001, 1102, 1203]);
    }

    #[test]
    fn test_convert_to_numstr_error() {
        let char_set = CharSet::new("abc").unwrap();
        let converter = YourStructName { char_set };
        
        let result = converter.convert_to_numstr("d");
        assert!(result.is_err());
    }
}
*/
