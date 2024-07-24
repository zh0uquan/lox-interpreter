pub struct ExitStatus {
    pub exit_code: i32, 
    pub output: String
}


pub fn scan(file_content: String) -> ExitStatus {
    let mut exit_code = 0;
    let output = file_content.lines()
        .enumerate()
        .map(
            |(lineno, line)| scan_line(line, lineno + 1, &mut exit_code)
        ).collect();
    let output = add_eof(output);
    ExitStatus {exit_code, output}
}

fn scan_line(line: &str, lineno: usize, exit_code: &mut i32) -> String {
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
        _ => {
            *exit_code = 65;
            raise_err(c, lineno)
        }
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
        let result = scan("(()".into());
        assert_eq!(
            result.output,
            "LEFT_PAREN ( null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\nEOF  null"
        );
        assert_eq!(result.exit_code, 0);

        // Test for braces
        let result = scan("{{}".into());
        assert_eq!(
            result.output,
            "LEFT_BRACE { null\nLEFT_BRACE { null\nRIGHT_BRACE } null\nEOF  null"
        );
        assert_eq!(result.exit_code, 0);

        // Test for unexpected characters
        let result = scan("abc".into());
        assert!(result.output.contains("Error: Unexpected character"));
        assert_eq!(result.exit_code, 65);

        // Test for symbols * . , +
        let result = scan("*.,+".into());
        assert_eq!(
            result.output,
            "STAR * null\nDOT . null\nCOMMA , null\nPLUS + null\nEOF  null"
        );
        assert_eq!(result.exit_code, 0);

        // Test for mixed characters
        let result = scan("*(+}".into());
        assert_eq!(
            result.output,
            "STAR * null\nLEFT_PAREN ( null\nPLUS + null\nRIGHT_BRACE } null\nEOF  null"
        );
        assert_eq!(result.exit_code, 0);
        
        // Test for unexpected characters with special symbols
        let result = scan("$#*(+}".into());
        assert!(result.output.contains("Error: Unexpected character: $"));
        assert!(result.output.contains("Error: Unexpected character: #"));
        assert_eq!(result.exit_code, 65);
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
