use std::iter::zip;

pub fn scan(file_content: String) -> String {
    let parsed = file_content.chars()
        .map(
            |c| match c { 
                '(' => "LEFT_PAREN",
                ')' => "RIGHT_PAREN",
                '{' => "LEFT_BRACE",
                '}' => "RIGHT_BRACE",
                _ => unimplemented!("skip")
            }
        );
    
    zip(file_content.chars(), parsed).map(
        |(ch, identifier)| format!("{identifier} {ch} null\n")
    ).collect()
}

pub fn add_eof(s: String) -> String {
    format!("{s}EOF  null")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_parentheses() {
        assert_eq!(
            scan("(()".into()),
            "LEFT_PAREN ( null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\n"
        );
    }
    
    #[test]
    fn test_scan_braces() {
        assert_eq!(
            scan("{{}".into()), 
            "LEFT_BRACE { null\nLEFT_BRACE { null\nRIGHT_BRACE } null\n"
        );
    }

    #[test]
    #[should_panic(expected = "skip")]
    fn test_scan_unexpected_characters() {
        scan("abc".into());
    }

    #[test]
    fn test_add_eof() {
        assert_eq!(
            add_eof("content ".into()),
            "content EOF  null"
        );
    }

    #[test]
    fn test_add_eof_empty_string() {
        assert_eq!(
            add_eof("".into()),
            "EOF  null"
        );
    }
}