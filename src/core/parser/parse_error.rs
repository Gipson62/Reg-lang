use std::fmt;

pub(crate) struct ParseError;
impl ParseError {
    pub fn new() -> ParseError {
        ParseError {}
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError")
    }
}