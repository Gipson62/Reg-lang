use crate::basic::{
    lexer::Lexer,
    errors::errors::Error,
};

pub(crate) struct Runner;

impl Runner {
    pub fn run(file_name:String, text:String) /*-> Result<(), Error>*/ {
        let lexer = Lexer::new(file_name, text);
        //let tokens = lexer.make_tokens();
    }
}