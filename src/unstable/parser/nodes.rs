use crate::{
    unstable::{
        tokens::{
            tokens::Token,
            tokens_type::TokenType,
        },
        errors::errors::{
            Error,
            ErrorType,
        },
    },
    Language,
};

/// List of possible `LiteralValue` types.
pub(crate) enum LiteralValue {
    Int(u64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}
/// Enum `Expr` for the `Parser`
pub(crate) enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: String,
    },
}