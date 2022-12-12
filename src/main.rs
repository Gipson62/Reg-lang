#![allow(unused)]
pub mod core {
    pub mod errors {
        pub mod errors;
    }
    pub mod lexer {
        pub mod lexer;
        pub mod token;
    }
    pub mod parser {
        pub mod nodes;
        pub mod parse_error;
        pub mod parser;
    }
}
use crate::core::{
    errors::errors::*,
    lexer::{
        lexer::Lexer,
        token::*,
    },
    parser::parser::*,
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
pub fn run_file(filename: String) {
    todo!("Make the file part...")
}
/// Run the source code from the console.
pub fn run_prompt() {
    print!(" $ ");
    loop {
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        
        if s == "exit()" {
            println!("Exiting...");
            break;
        }
        let mut lexer = Lexer::new(
            &s,
        );
        let tokens = lexer.make_tokens();
        for i in tokens {
            print!("{}", i.token_type);
        }
        print!("\n $ ");
    }
}
