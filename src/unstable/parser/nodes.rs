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
/// Char, Float, UnsignedInt and Int32 are not supported yet.
/// TODO: Add UnsignedInt and different sizes of Number.
pub(crate) enum LiteralValue {
    Int64(i64),
    Int32(i32),
    UnsignedInt64(u64),
    UnsignedInt32(u32),
    Float64(f64),
    Float32(f32),
    /// First type of Number for the Language. It's temporary.
    BaseNumberFloat(f64),
    /// Second type of Number for the Language. It's temporary.
    BaseNumberInt(i64),
    String(String),
    Char(char),
    Boolean(bool),
    None,
    False,
    True
}
/// Enum `Expr` for the `Parser`
pub(crate) enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token, // + - * / % == != > >= < <=
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token, // + - * / % == != > >= < <=
        right: Box<Expr>,
    },
    Variable {
        name: String,
    },
}