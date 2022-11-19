#![allow(unused)]

use std::fmt;
use super::position::Position;

#[derive(Clone)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub pos_start: Position,
    pub pos_end: Position,
    pub value: String,
}
impl Token {
    /// Creates a new [`Token`].
    pub fn new(
        token_type: TokenType,
        pos_start: Position,
        pos_end: Position,
        value: String,
    ) -> Token {
        Token {
            token_type,
            pos_start,
            pos_end,
            value,
        }
    }
    /// Check if the 2 [`Token`] matches.
    pub fn matches(self, type_: TokenType, value: String) -> bool {
        return self.token_type == type_ && self.value == value
    }
    /// Returns a string representation of the [`Token`].
    pub fn to_string(self) -> String {
        if self.value != "" {
            return format!("{}:{}", self.token_type.to_string(), self.value)
        }
        return format!("{}", self.token_type)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenType {
    TTInt,
    TTFloat,
    TTString,
    TTIdentifier,
    TTKeyword,
    TTPlus,
    TTMinus,
    TTMultiply,
    TTDivide,
    TTPower,
    TTEqual,
    TTLParen,
    TTRParen,
    TTLSquare,
    TTRSquare,
    TTDoubleEqual,
    TTNotEqual,
    TTLessThan,
    TTGreaterThan,
    TTLessThanEqual,
    TTGreaterThanEqual,
    TTComma,
    TTArrow,
    TTNewLine,
    TTEndOfLine
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
                TokenType::TTInt => write!(f, "TTInt"),
                TokenType::TTFloat => write!(f, "TTFloat"),
                TokenType::TTString => write!(f, "TTString"),
                TokenType::TTIdentifier => write!(f, "TTIdentifier"),
                TokenType::TTKeyword => write!(f, "TTKeyword"),
                TokenType::TTPlus => write!(f, "TTPlus"),
                TokenType::TTMinus => write!(f, "TTMinus"),
                TokenType::TTMultiply => write!(f, "TTMultiply"),
                TokenType::TTDivide => write!(f, "TTDivide"),
                TokenType::TTPower => write!(f, "TTPower"),
                TokenType::TTEqual => write!(f, "TTEqual"),
                TokenType::TTLParen => write!(f, "TTLParen"),
                TokenType::TTRParen => write!(f, "TTRParen"),
                TokenType::TTLSquare => write!(f, "TTLSquare"),
                TokenType::TTRSquare => write!(f, "TTRSquare"),
                TokenType::TTDoubleEqual => write!(f, "TTDoubleEqual"),
                TokenType::TTNotEqual => write!(f, "TTNotEqual"),
                TokenType::TTLessThan => write!(f, "TTLessThan"),
                TokenType::TTGreaterThan => write!(f, "TTGreaterThan"),
                TokenType::TTLessThanEqual => write!(f, "TTLessThanEqual"),
                TokenType::TTGreaterThanEqual => write!(f, "TTGreaterThanEqual"),
                TokenType::TTComma => write!(f, "TTComma"),
                TokenType::TTArrow => write!(f, "TTArrow"),
                TokenType::TTNewLine => write!(f, "TTNewLine"),
                TokenType::TTEndOfLine => write!(f, "TTEndOfLine"),
        }
    }
}

pub(crate) struct Keywords {
    pub kw_var:String,
    pub kw_and:String,
    pub kw_or:String,
    pub kw_not:String,
    pub kw_if:String,
    pub kw_else:String,
    pub kw_for:String,
    pub kw_to:String,
    pub kw_step:String,
    pub kw_while:String,
    pub kw_func:String,
    pub kw_return:String,
    pub kw_continue:String,
    pub kw_break:String,
    pub kw_then:String,
    pub kw_end:String,
}
impl Keywords {
    pub fn new() -> Keywords {
        Keywords {
            kw_var:String::from("var"),
            kw_and:String::from("and"),
            kw_or:String::from("or"),
            kw_not:String::from("not"),
            kw_if:String::from("if"),
            kw_else:String::from("else"),
            kw_for:String::from("for"),
            kw_to:String::from("to"),
            kw_step:String::from("step"),
            kw_while:String::from("while"),
            kw_func:String::from("func"),
            kw_return:String::from("return"),
            kw_continue:String::from("continue"),
            kw_break:String::from("break"),
            kw_then:String::from("then"),
            kw_end:String::from("end"),
        }
    }
}

pub(crate) struct Characters {
    pub digits:[char; 10],
    pub letters:[char; 52],
    pub skip_letters:char,
    pub new_line:char,
    pub comment_symbol:char,
}
impl Characters {
    pub fn new() -> Characters {
        Characters {
            digits:['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
            letters:['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
            skip_letters:char::from(92),
            new_line:char::from('\n'),
            comment_symbol:char::from('#'),
        }
    }
}