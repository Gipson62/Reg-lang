use crate::{
    lib::string_with_arrows::{
        string_with_arrows,
    },
    core::tokens::position::Position
};
use std::fmt;
/// Base Error Type for the interpreter
pub(crate) struct Error {
    pub details: String,
    pub pos_start: Position,
    pub pos_end: Position,
    pub error_type: ErrorType,
}
impl Error {
    pub fn new(
        pos_start:Position,
        pos_end:Position,
        details: String,
        error_type: ErrorType,
    ) -> Error {
        let error = Error {
            details,
            pos_start,
            pos_end,
            error_type,
        };
        error
        
    }
    ///String representation of the [`Error`] struct
    pub fn as_string(&self) -> String {
        let mut result = format!("{}: {}\n", self.error_type, self.details);

        result.push_str(&format!("File {}, line {}, pos {}", self.pos_start.file_name, self.pos_start.ln.to_string(), self.pos_start.idx));
        //result.push_str(&format!("\n\n{}", string_with_arrows(self.pos_start.text.clone(), self.pos_start.clone(), self.pos_end.clone())));
        //println!("{}", result);

        return result;
    }
    /// Generate a runtime error with a traceback
    /// TODO!
    pub fn new_runtime_error(
        pos_start:Position,
        pos_end:Position,
        details: String,
    ) -> Error {
        let error = Error {
            details,
            pos_start,
            pos_end,
            error_type: ErrorType::RunTimeError,
        };
        error.as_string();
        error
    }
    fn generate_traceback(&self) -> String {
        let mut result = String::new();
        let pos = self.pos_start.clone();
        

        return result
    }
}

/// Enum for the different types of errors
/// Todo add errors to list
pub(crate) enum ErrorType {
    IllegalCharError,
    ExpectedCharError,
    InvalidSyntaxError,
    RunTimeError,
}
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::IllegalCharError => write!(f, "Illegal Character"),
            ErrorType::ExpectedCharError => write!(f, "Expected Character"),
            ErrorType::InvalidSyntaxError => write!(f, "Invalid Syntax"),
            ErrorType::RunTimeError => write!(f, "Runtime Error"),
        }
    }
}