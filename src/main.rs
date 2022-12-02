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
use std::io::{
    stdin,
    stdout,
    Write,
};
fn main() {
    let mut s = String::new();
    print!(">>> File or Console ? [f <file_path>/c]: ");
    println!("The file is buggy, so use the console");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    if s.starts_with('f') {
        run_file(s[1..].to_string());
    } else {
        run_prompt();
    }
    /*let source = " != ! = < <= 10 10.1 -> \n - {/}\"salut\"".to_string();
    let mut lang = Language::new();
    let mut lexer = UnstableLexer::new(
        source,
        UnstablePosition::new(0, 1, 0, "<stdin>".to_string()),
        lang,
    );
    lexer.make_tokens();
    if lexer.lang.had_error {
        panic!("Lexer error");
    }*/
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
        
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        if s == "exit()" {
            println!("Exiting...");
            break;
        }
        let mut lang = Language::new();
        let mut lexer = UnstableLexer::new(
            s,
            UnstablePosition::new(0, 1, 0, "<stdin>".to_string()),
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
}

