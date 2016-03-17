struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // next_char - Peek at next char, do not consume
    fn next_char(&self) -> char {
        // Possible panic?
        self.input[self.pos..].chars().next().unwrap()
    }
    // Starts with - Do next characters match given String
    fn starts_with(&self, string: &str) -> bool {
        &self.input[self.pos..string.len()] == string
    }
    // eof - Return true if we've reached the end
}

#[test]
fn test_next_char() {
    let test_parser = Parser {pos: 0, input: "H".to_string()};
    println!("{}", test_parser.next_char());
    println!("{}", test_parser.next_char());
    println!("{}", test_parser.next_char());
    println!("{}", test_parser.next_char());
}

#[test]
fn test_starts_with() {
    let test_parser = Parser {pos: 0, input: "Hello World".to_string()};
    assert!(test_parser.starts_with("Hello"));
    assert!(!test_parser.starts_with("World"));
}
