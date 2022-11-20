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
    }
    pub mod tokens {
        pub mod token;
        pub mod position;
    }
}
pub mod lib {
    pub mod string_with_arrows;
}
use crate::core::{
    lexer::lexer::Lexer,
};
use crate::lib::string_with_arrows::{
    string_with_arrows,
    
};
fn main() {
    let mut lexer = Lexer::new(
        "<stdin>".to_string(),
        "var num = 5 + 5 * 20.3".to_string(),
    );
    lexer.make_tokens();
}