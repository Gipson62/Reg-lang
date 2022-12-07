use crate::{
    lib::string_with_arrows::{
        string_with_arrows,
    },
    unstable::tokens::position::Position,
};
use std::fmt;
/// Base `Error` struct for the `Interpreter`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Error {
    pub details: String,
    pub pos_start: Position,
    pub error_type: ErrorType,
}
impl Error {
    /// Creates a new `Error` instance.
    pub fn new(pos_start:Position, details: String, error_type: ErrorType,) -> Error {
        let error = Error {
            details,
            pos_start,
            error_type,
        };
        error
        
    }
    ///String representation of the `Error` struct
    pub fn as_string(&self) -> String {
        let mut result = format!("{}: {}\n", self.error_type, self.details);

        result.push_str(&format!("File {}, line {}, pos {}", self.pos_start.file_name, self.pos_start.ln.to_string(), self.pos_start.idx));

        return result;
    }
    /// Generate a runtime error with a traceback
    /// TODO!
    pub fn new_runtime_error(pos_start:Position, details: String,) -> Error {
        let error = Error {
            details,
            pos_start,
            error_type: ErrorType::RunTimeError,
        };
        error.as_string();
        error
    }
    /// TODO!
    fn generate_traceback(&self) -> String {
        let mut result = String::new();
        let pos = self.pos_start.clone();

        return result
    }
}

/// Enum for the different types of `Errors`.
/// Todo add more errors to the list.
#[derive(Debug, Clone, PartialEq)]
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
            ErrorType::ExpectedCharError => write!(f, "Unexpected CharError"),
            ErrorType::InvalidSyntaxError => write!(f, "Invalid Syntax"),
            ErrorType::RunTimeError => write!(f, "Runtime Error"),
        }
    }
}