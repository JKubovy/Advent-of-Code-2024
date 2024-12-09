pub trait StrParser {
    fn parse_usize(&self) -> usize;
}
impl StrParser for &str {
    fn parse_usize(&self) -> usize {
        self.parse::<usize>().expect("string is NOT number")
    }
}

impl StrParser for char {
    fn parse_usize(&self) -> usize {
        self.to_digit(10).expect("Invalid character, not a digit") as usize
    }
}
