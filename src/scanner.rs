use std::iter::zip;

pub fn scan(file_content: String) -> String {
    let parsed = file_content.chars()
        .map(
            |c| match c { 
                '(' => "LEFT_PAREN",
                ')' => "RIGHT_PAREN",
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
    fn test_scan() {
        assert_eq!(
            scan(String::from("(()")),
            String::from("LEFT_PAREN ( null\nLEFT_PAREN ( null\nRIGHT_PAREN ) null\n")
        );
    }
    
    #[test]
    fn test_add_eof() {
        assert_eq!(add_eof(String::from("\n")), String::from("\nEOF  null"))
    }
}