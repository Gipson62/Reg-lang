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
    pub mod parser {
        pub mod parser;
        pub mod nodes;
        pub mod parse_error;
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
    tokens::{
        position::Position as UnstablePosition,
        tokens_type::TokenType as UnstableTokenType,
        tokens::Token as UnstableToken,
    },
};
use std::io::{
    stdin,
    stdout,
    Write,
};
fn main() {
    println!("--prompt of file path:");
    // Read a line of input from the standard input stream
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Check if the input is the "--prompt" mode flag
    if input.contains("--prompt") {
        // Run the interpreter in prompt mode
        run_prompt();
        return;
    }

    else if input.contains("--file") {
        // Otherwise, check if the input is the file path
        let filename = input.replace("--file", "").trim().to_string();
        if !filename.is_empty() {
            // Run the interpreter on the specified file
            run_file(filename);
            return;
        }
    }
    

    // Otherwise, print an error message
    eprintln!("Error: no file path or mode flag provided");
}
/// Run the source code from a file.
/// TODO! Fix it.
pub fn run_file(path: String) {
    let source = std::fs::read_to_string(path).expect("File not found");
    let mut lang = Language::new();
    let mut lexer = UnstableLexer::new(
        source,
        UnstablePosition::new(0, 0, 0, "<stdin>".to_string()),
        lang,
    );
    lexer.make_tokens();
    if lexer.lang.had_error {
        panic!("Lexer error");
    }
}
/// Run the source code from the console.
pub fn run_prompt() {
    loop {
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        
        if s == "exit()" {
            println!("Exiting...");
            break;
        }
        let mut lang = Language::new();
        let mut lexer = UnstableLexer::new(
            s,
            UnstablePosition::new(0, 1, 0, "<prompt>".to_string()),
            lang,
        );
        lexer.make_tokens();
        if lexer.lang.had_error {
            panic!("Lexer error");
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
/// Struct to check if there is an error.
pub(crate) struct Language {
    pub had_error: bool,
}
impl Language {
    /// Creates a new `Language` instance.
    pub fn new() -> Language {
        Language {
            had_error: false,
        }
    }
    /// Save if there is an error. After an operation, if there is an error, the programs need to crash.
    pub fn error(&mut self, pos_start: UnstablePosition, error_type: UnstableErrorType, details: String) {
        self.had_error = true;
        let mut error = UnstableError::new(
            pos_start,
            details,
            error_type,
        );
        println!("{}", error.as_string());
    }
    pub fn parse_error(&mut self, token: UnstableToken, message: &str) {
        if token.tok_type == UnstableTokenType::TTEndOfFile {
            println!("{} at end {}", token.pos.ln, message);
        } else {
            println!("{} at '{}' {}", token.pos.ln, token.lexeme, message);
        }
    }
}

