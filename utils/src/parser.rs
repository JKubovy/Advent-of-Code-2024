pub trait StrParser {
    fn parse_usize(&self) -> usize;
}
impl StrParser for &str {
    fn parse_usize(&self) -> usize {
        self.parse::<usize>().expect("string is NOT number")
    }
}
