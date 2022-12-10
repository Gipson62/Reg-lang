use crate::core::tokens::{
    position::Position,
    tokens_type::TokenType,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
/// Most basic struct representing a `Token` with a `TokenType` its value and a `Position`. 
pub(crate) struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub pos: Position,
}
impl Token {
    /// Creates a new `Token` instance.
    pub fn new(tok_type: TokenType, lexeme: String, pos: Position) -> Token {
        Token {
            tok_type,
            lexeme,
            pos,
        }
    }
    /// String representation of the `Token` struct.
    pub fn as_string(&self) -> String {
        return format!("{} : {}", self.tok_type, self.lexeme)
    }
}