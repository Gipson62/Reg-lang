#![allow(unused)]
pub mod core {
    pub mod errors {
        pub mod errors;
    }
    pub mod interpreter {
        pub mod interpreter;
    }
    pub mod lexer {
        pub mod lexer;
    }
    pub mod nodes {
        pub mod nodes;
    }
    pub mod parser {
        pub mod parser;
        pub mod parse_result;
        pub mod nodes;
    }
    pub mod tokens {
        pub mod token;
        pub mod position;
    }
}
pub mod lib {
    pub mod string_with_arrows;
}
pub mod unstable {
    pub mod tokens {
        pub mod tokens;
        pub mod position;
        pub mod tokens_type;
        pub mod keywords;
    }
    pub mod lexer {
        pub mod lexer;
    }
    pub mod errors {
        pub mod errors;
    }
}
use crate::core::{
    lexer::lexer::Lexer,
    parser::parser::Parser,
};
use crate::lib::string_with_arrows::{
    string_with_arrows,
};
use crate::unstable::{
    lexer::lexer::Lexer as UnstableLexer,
    errors::errors::{
        Error as UnstableError,
        ErrorType as UnstableErrorType,
    },
    tokens::position::Position as UnstablePosition,
};
fn main() {
    let source = " != ! = < <= 10 10.1 -> - {/}\"salut\"".to_string();
    let mut lexer = UnstableLexer::new(
        source,
        UnstablePosition::new(0, 0, 0, "<stdin>".to_string()),
        Language::new(),
    );
    lexer.make_tokens();
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Language {
    pub had_error: bool,
}
impl Language {
    pub fn new() -> Language {
        Language {
            had_error: false,
        }
    }
    pub fn error(&mut self, pos_start: UnstablePosition, error_type: UnstableErrorType, details: String) {
        self.had_error = true;
        let mut error = UnstableError::new(
            pos_start,
            details,
            error_type,
        );
        println!("{}", error.as_string());
    }
}