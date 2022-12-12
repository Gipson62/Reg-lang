use crate::core::lexer::token::*;
pub(crate) struct Lexer {
    input: Vec<char>,
    index: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            index: 0,
        }
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        // TODO!: Add Identifier, Number(Int/Float) and Keywords support.
        let mut tokens: Vec<Token> = Vec::new();
        let mut pos = Position::new(0, 0);
        while !self.is_at_end() {
            match self.advance() {
                Some('(') => {
                    tokens.push(Token::new(TokenType::LParen, pos.clone()));
                    pos.advance('(');
                },
                Some(')') => {
                    tokens.push(Token::new(TokenType::RParen, pos.clone()));
                    pos.advance(')');
                },
                Some('{') => {
                    tokens.push(Token::new(TokenType::LBrace, pos.clone()));
                    pos.advance('{');
                },
                Some('}') => {
                    tokens.push(Token::new(TokenType::RBrace, pos.clone()));
                    pos.advance('}');
                },
                Some(',') => {
                    tokens.push(Token::new(TokenType::Comma, pos.clone()));
                    pos.advance(',');
                },
                Some('.') => {
                    tokens.push(Token::new(TokenType::Dot, pos.clone()));
                    pos.advance('.');
                },
                Some('-') => {
                    if self.peek() == Some('>') {
                        self.advance();
                        tokens.push(Token::new(TokenType::Arrow, pos.clone()));
                        pos.advance('-');
                        pos.advance('>');
                    } else {
                        tokens.push(Token::new(TokenType::Minus, pos.clone()));
                        pos.advance('-');
                    }
                },
                Some('+') => {
                    tokens.push(Token::new(TokenType::Plus, pos.clone()));
                    pos.advance('+');
                },
                Some(';') => {
                    tokens.push(Token::new(TokenType::Semicolon, pos.clone()));
                    pos.advance(';');
                },
                Some('*') => {
                    tokens.push(Token::new(TokenType::Star, pos.clone()));
                    pos.advance('*');
                },
                Some('!') => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::NotEqual, pos.clone()));
                        pos.advance('!');
                        pos.advance('=');
                    } else {
                        tokens.push(Token::new(TokenType::Not, pos.clone()));
                        pos.advance('!');
                    }
                },
                Some('=') => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::DoubleEqual, pos.clone()));
                        pos.advance('=');
                        pos.advance('=');
                    } else {
                        tokens.push(Token::new(TokenType::Equal, pos.clone()));
                        pos.advance('=');
                    }
                },
                Some('<') => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::LessThanEqual, pos.clone()));
                        pos.advance('<');
                        pos.advance('=');
                    } else {
                        tokens.push(Token::new(TokenType::LessThan, pos.clone()));
                        pos.advance('<');
                    }
                },
                Some('>') => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::GreaterThanEqual, pos.clone()));
                        pos.advance('>');
                        pos.advance('=');
                    } else {
                        tokens.push(Token::new(TokenType::GreaterThan, pos.clone()));
                        pos.advance('>');
                    }
                },
                Some('/') => {
                    if self.peek() == Some('/') {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Slash, pos.clone()));
                        pos.advance('/');
                    }
                },
                Some('%') => {
                    tokens.push(Token::new(TokenType::Modulo, pos.clone()));
                    pos.advance('%');
                },
                Some('[') => {
                    tokens.push(Token::new(TokenType::LBrace, pos.clone()));
                    pos.advance('[');
                },
                Some(']') => {
                    tokens.push(Token::new(TokenType::RBrace, pos.clone()));
                    pos.advance(']');
                },
                Some('"') => {
                    let mut string = String::new();
                    while self.peek() != Some('"') && !self.is_at_end() {
                        string.push(self.advance().unwrap());
                    }
                    if self.is_at_end() {
                        // TODO: Error handling
                        panic!("Unterminated string");
                    }
                    self.advance();
                    tokens.push(Token::new(TokenType::String(string.clone()), pos.clone()));
                    pos.advance('"');
                    for i in 0..string.clone().len() {
                        pos.advance(' ');
                    }
                    pos.advance('"');
                },
                Some('\'') => {
                    let mut character:char;
                    if self.peek() == Some('\\') {
                        self.advance();
                        character = match self.advance().unwrap() {
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '\\' => '\\',
                            '\'' => '\'',
                            _ => panic!("What you trying to do?")
                        }
                    } else {
                        character = self.advance().unwrap();
                    }
                    if self.advance().unwrap() != '\'' {
                        // TODO: Error handling
                        panic!("Unterminated char");
                    } else {
                        tokens.push(Token::new(TokenType::Char(character), pos.clone()));
                        pos.advance('\'');
                        pos.advance('A');
                        pos.advance('\'');

                    }
                },
                Some('\n') => {
                    pos.advance('\n');
                },
                Some(' ') | Some('\t') | Some('\r') => {
                    pos.advance(' ');
                },
                Some(c) => {
                    if c.is_alphabetic() {
                        let position = pos.clone();
                        let mut s = String::new();
                        s.push(c);
                        while self.peek().unwrap().is_alphanumeric() {
                            let c = self.advance().unwrap();
                            s.push(c);
                            pos.advance(c);
                        }
                        let keyword = keyword(&s, position.clone());
                        if keyword != None {
                            tokens.push(keyword.unwrap());
                        } else {
                            tokens.push(Token::new(TokenType::Identifier(s), position));
                        }
                    } else if c.is_numeric() {
                        let mut is_float: bool = false;
                        let mut s = String::new();
                        let position = pos.clone();
                        s.push(c);

                        while self.peek().unwrap().is_numeric() || self.peek() == Some('.') || self.is_at_end() {
                            let ch = self.advance().unwrap();
                            if ch == '.' {
                                is_float = true;
                            }
                            s.push(ch);
                            pos.advance(ch);
                            if self.peek() == None {
                                break;
                            }
                        }
                        if is_float {
                            tokens.push(Token::new(TokenType::Float(s.parse::<f64>().unwrap()), position));
                        } else {
                            tokens.push(Token::new(TokenType::Int(s.parse::<i64>().unwrap()), position));
                        }
                    }
                },
                _ => {}
            }
        }
        return tokens;
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        } else {
            self.index += 1;
            return Some(self.input[self.index - 1]);
        }
    }

    fn peek(&self) -> Option<char> {
        if self.index >= self.input.len() {
            None
        } else {
            Some(self.input[self.index])
        }
    }
    fn is_at_end(&self) -> bool {
        self.index >= self.input.len()
    }
}