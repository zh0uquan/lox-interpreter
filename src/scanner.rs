use std::iter::zip;

pub fn scan(file_content: String) -> String {
    file_content.lines().enumerate().map(
        |(lineno, line)| scan_line(line, lineno + 1)
    ).collect()
}

fn scan_line(line: &str, lineno: usize) -> String {
    line.chars().map(|c| match c {
        '(' => add_token(c, "LEFT_PAREN"),
        ')' => add_token(c, "RIGHT_PAREN"),
        '{' => add_token(c, "LEFT_BRACE"),
        '}' => add_token(c, "RIGHT_BRACE"),
        '*' => add_token(c, "STAR"),
        '.' => add_token(c, "DOT"),
        ',' => add_token(c, "COMMA"),
        '+' => add_token(c, "PLUS"),
        '-' => add_token(c, "MINUS"),
        ';' => add_token(c, "SEMICOLON"),
        _ => raise_err(c, lineno)
    }).collect()
}
fn add_token(ch: char, token: &str) -> String {
    format!("{token} {ch} null\n")
}

fn raise_err(ch: char, lineno: usize) -> String {
    format!("[line {lineno}] Error: Unexpected character: {ch}\n")
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
