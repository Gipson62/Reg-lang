#![allow(unused)]

use std::fmt;
use crate::staminars::tokens::position::Position;

#[derive(Clone, Debug, PartialEq)]
/// Token refer to each word or symbol in the source code
/// It contains the token type, the value and the position in the code
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
/// Enum for the different types of tokens
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
    TTEndOfLine,
    TTSemicolon,
    TTTabulation,
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
                TokenType::TTSemicolon => write!(f, "TTSemicolon"),
                TokenType::TTTabulation => write!(f, "TTTabulation"),
        }
    }
}

pub(crate) struct Iter<'a> {
    pub inner: &'a Keywords,
    pub index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a String;
    
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.kw_var,
            1 => &self.inner.kw_and,
            2 => &self.inner.kw_or,
            3 => &self.inner.kw_not,
            4 => &self.inner.kw_if,
            5 => &self.inner.kw_else,
            6 => &self.inner.kw_for,
            7 => &self.inner.kw_to,
            8 => &self.inner.kw_step,
            9 => &self.inner.kw_while,
            10 => &self.inner.kw_func,
            11 => &self.inner.kw_return,
            12 => &self.inner.kw_continue,
            13 => &self.inner.kw_break,
            14 => &self.inner.kw_then,
            15 => &self.inner.kw_end,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

#[derive(Debug, Clone, PartialEq,)]
/// List of all the keywords in the language
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
    /// Generate the list of keywords
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
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Contains all characters that are used in the language
pub(crate) struct Characters {
    pub digits:[char; 10],
    pub letters:[char; 52],
    pub skip_letters:char,
    pub new_line:char,
    pub comment_symbol:char,
    pub symbols:[char; 18],
}
impl Characters {
    pub fn new() -> Characters {
        Characters {
            digits:['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
            letters:['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
            skip_letters:char::from(92),
            new_line:char::from('\n'),
            comment_symbol:char::from('#'),
            symbols:['+', '*', '/', '^', '(', ')', '[', ']', ',', ';', '\n', '\t', '-', '>', '<', '=', '!', '"'],
        }
    }
}