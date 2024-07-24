use std::iter::zip;

pub fn scan(file_content: String) -> String {
    let parsed = file_content.chars().map(|c| match c {
        '(' => "LEFT_PAREN",
        ')' => "RIGHT_PAREN",
        '{' => "LEFT_BRACE",
        '}' => "RIGHT_BRACE", 
        '*' => "STAR",
        '.' => "DOT",
        ',' => "COMMA",
        '+' => "PLUS",
        '-' => "MINUS",
        ';' => "SEMICOLON",
        _ => unimplemented!("skip"),
    });

    zip(file_content.chars(), parsed)
        .map(|(ch, identifier)| format!("{identifier} {ch} null\n"))
        .collect()
}

pub fn add_eof(s: String) -> String {
    format!("{s}EOF  null")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scan() {
        // Test for parentheses
        assert_eq!(
            scan("(()".into()),
            "LEFT_PAREN ( null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\n"
        );

        // Test for braces
        assert_eq!(
            scan("{{}".into()), 
            "LEFT_BRACE { null\nLEFT_BRACE { null\nRIGHT_BRACE } null\n"
        );

        // Test for unexpected characters
        let result = std::panic::catch_unwind(|| scan("abc".into()));
        assert!(result.is_err());

        // Test for adding EOF
        assert_eq!(
            add_eof("content ".into()),
            "content EOF  null"
        );

        // Test for adding EOF to empty string
        assert_eq!(
            add_eof("".into()),
            "EOF  null"
        );

        // Test for symbols * . , +
        assert_eq!(
            scan("*.,+".into()),
            "STAR * null\nDOT . null\nCOMMA , null\nPLUS + null\n"
        );

        // Test for mixed characters
        assert_eq!(
            scan("*(+}".into()),
            "STAR * null\nLEFT_PAREN ( null\nPLUS + null\nRIGHT_BRACE } null\n"
        );
    }

    #[test]
    fn test_add_eof() {
        assert_eq!(add_eof("content ".into()), "content EOF  null");
    }

    #[test]
    fn test_add_eof_empty_string() {
        assert_eq!(add_eof("".into()), "EOF  null");
    }
}
