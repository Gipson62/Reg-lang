use crate::unstable::tokens::{
    position::Position,
    tokens_type::TokenType,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
    pub pos: Position,
}
impl Token {
    pub fn new(tok_type: TokenType, lexeme: String, pos: Position) -> Token {
        Token {
            tok_type,
            lexeme,
            pos,
        }
    }
    pub fn as_string(&self) -> String {
        return format!("{} : {}", self.tok_type, self.lexeme)
    }
}