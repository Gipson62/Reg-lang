#![allow(unused)]
//Import Tokens from ./basic/token.rs
pub mod basic{
    pub mod token;
    pub mod position;
    pub mod lexer;
    pub mod runner;
    pub mod errors {
        pub mod errors;
    }
}
use crate::basic::{
    token::{
        Token,
        TokenType,
        Keywords,
        Characters,
    },
    position::Position,
    lexer::Lexer,
    runner::Runner,
    errors::errors::{
        Error,
        ErrorType,
    },
};

pub mod lib {
    pub mod string_with_arrows;
}

use crate::lib::string_with_arrows::{
    string_with_arrows,
    
};

mod salut;
use crate::salut::Salut;

fn main() {
    let error = Error::new(
        Position::new(
            0,
            0,
            0,
            "<stdin>".to_string(),
            "sal\nut".to_string(),
        ),
        Position::new(
            10,
            0,
            0,
            "<stdin>".to_string(),
            "salut".to_string(),
        ),
        "Some details...".to_string(),
        ErrorType::IllegalCharError,
    );
}