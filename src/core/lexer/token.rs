use std::fmt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
        // Single character tokens.
        LParen,
        RParen,
        LSquare,
        RSquare,
        LBrace,
        RBrace,
        Comma,
        Dot,
        Minus,
        Plus,
        Colon,
        Semicolon,
        Slash,
        Star,
        Power,
        Modulo,
        NewLine,
        // One or two characters tokens.
        Not,
        NotEqual,
        Equal,
        DoubleEqual,
        GreaterThan,
        GreaterThanEqual,
        LessThan,
        LessThanEqual,
        Arrow,
        // Literals.
        Identifier(String),
        String(String),
        Int(i64),
        Float(f64),
        Char(char),
        // Keywords.
        And,
        Class,
        Else,
        False,
        Func,
        For,
        If,
        None,
        Or,
        Print,
        Return,
        Super,
        This,
        True,
        Let,
        While,
        Loop,
        Struct,
        Break,
        Continue,
        Entity,
        Component,
        System,
        // End of file.
        EndOfFile,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LSquare => write!(f, "["),
            TokenType::RSquare => write!(f, "]"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Power => write!(f, "^"),
            TokenType::Modulo => write!(f, "%"),
            TokenType::NewLine => write!(f, "!"),
            TokenType::Not => write!(f, "!"),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::DoubleEqual => write!(f, "=="),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::GreaterThanEqual => write!(f, ">="),
            TokenType::LessThan => write!(f, "<"),
            TokenType::LessThanEqual => write!(f, "<="),
            TokenType::Arrow => write!(f, "->"),
            TokenType::Identifier(s) => write!(f, "Identifier: {}", s),
            TokenType::String(s) => write!(f, "String: {}", s),
            TokenType::Int(i) => write!(f, "Int: {}", i),
            TokenType::Float(fl) => write!(f, "Float: {}", fl),
            TokenType::Char(c) => write!(f, "Char: {}", c),
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Func => write!(f, "func"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::None => write!(f, "none"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Let => write!(f, "let"),
            TokenType::While => write!(f, "while"),
            TokenType::Loop => write!(f, "loop"),
            TokenType::Struct => write!(f, "struct"),
            TokenType::Break => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),
            TokenType::Entity => write!(f, "entity"),
            TokenType::Component => write!(f, "component"),
            TokenType::System => write!(f, "system"),
            TokenType::EndOfFile => write!(f, "EOF"),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub position: Position,
}
impl Token {
    pub fn new(token_type: TokenType, position: Position) -> Token {
        Token {
            token_type,
            position,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Position {
    line: usize,
    column: usize,
}
impl Position {
    pub(crate) fn new(line: usize, column: usize) -> Position {
        Position {
            line,
            column,
        }
    }
    pub fn advance(&mut self, character: char) {
        self.column += 1;
        if character == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }
}

pub(crate) fn keyword(s: &str, position: Position) -> Option<Token> {
    match s {
        "and" => Some(Token::new(TokenType::And, position)),
        "class" => Some(Token::new(TokenType::Class, position)),
        "else" => Some(Token::new(TokenType::Else, position)),
        "false" => Some(Token::new(TokenType::False, position)),
        "func" => Some(Token::new(TokenType::Func, position)),
        "for" => Some(Token::new(TokenType::For, position)),
        "if" => Some(Token::new(TokenType::If, position)),
        "None" => Some(Token::new(TokenType::None, position)),
        "or" => Some(Token::new(TokenType::Or, position)),
        "print" => Some(Token::new(TokenType::Print, position)),
        "return" => Some(Token::new(TokenType::Return, position)),
        "super" => Some(Token::new(TokenType::Super, position)),
        "this" => Some(Token::new(TokenType::This, position)),
        "true" => Some(Token::new(TokenType::True, position)),
        "while" => Some(Token::new(TokenType::While, position)),
        "loop" => Some(Token::new(TokenType::Loop, position)),
        "let" => Some(Token::new(TokenType::Let, position)),
        "struct" => Some(Token::new(TokenType::Struct, position)),
        "break" => Some(Token::new(TokenType::Break, position)),
        "continue" => Some(Token::new(TokenType::Continue, position)),
        "entity" => Some(Token::new(TokenType::Entity, position)),
        "component" => Some(Token::new(TokenType::Component, position)),
        "system" => Some(Token::new(TokenType::System, position)),
        &_ => None,
    }
}