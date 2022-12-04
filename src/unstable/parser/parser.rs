use crate::unstable::tokens::{
    tokens::Token,
    tokens_type::TokenType,
};
// TODO!
// Write the Parser
pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }
}