use crate::{
    unstable::{
        tokens::{
            tokens::Token,
            tokens_type::TokenType,
            position::Position,
            keywords::Keywords,
        },
        errors::errors::{
            Error,
            ErrorType,
        },
    }, 
    Language
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Lexer {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub pos: Position,
    pub current_char: char,
    pub lang: Language,
    pub keywords: Keywords,
}
impl Lexer {
    pub fn new(source: String, pos: Position, lang: Language) -> Lexer {
        let source_vec:Vec<char> = source.chars().collect();
        let mut lexer = Lexer {
            source: source_vec.clone(),
            tokens: Vec::new(),
            pos,
            current_char: source_vec.clone()[0],
            lang,
            keywords: Keywords::new(),
        };
        lexer.source.push('\0');
        return lexer
    }

    fn advance(&mut self) {
        self.pos.advance(Some(self.current_char.clone()));
        
        if !self.is_at_end() {
            self.current_char = self.source[self.pos.idx as usize];
        }
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            match self.current_char {
                '(' => {
                    self.tokens.push(Token::new(TokenType::TTLParen, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                ')' => {
                    self.tokens.push(Token::new(TokenType::TTRParen, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '{' => {
                    self.tokens.push(Token::new(TokenType::TTLBrace, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '}' => {
                    self.tokens.push(Token::new(TokenType::TTRBrace, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '[' => {
                    self.tokens.push(Token::new(TokenType::TTLSquare, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                ']' => {
                    self.tokens.push(Token::new(TokenType::TTRSquare, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                ',' => {
                    self.tokens.push(Token::new(TokenType::TTComma, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '.' => {
                    self.tokens.push(Token::new(TokenType::TTDot, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '-' => {
                    self.advance(); 
                    if self.current_char == '>' {
                        self.tokens.push (Token::new(TokenType::TTArrow, format!("-{}", self.current_char), self.pos.clone()));
                    }else {
                        self.tokens.push(Token::new(TokenType::TTMinus, "-".to_string(), self.pos.clone()));
                    }
                },
                '+' => {
                    self.tokens.push(Token::new(TokenType::TTPlus, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                ';' => {
                    self.tokens.push(Token::new(TokenType::TTSemicolon, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '*' => {
                    self.tokens.push(Token::new(TokenType::TTStar, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '^' => {
                    self.tokens.push(Token::new(TokenType::TTPower, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '%' => {
                    self.tokens.push(Token::new(TokenType::TTModulo, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                },
                '!' => {
                    self.advance();
                    if self.current_char == '=' {
                        self.tokens.push(Token::new(TokenType::TTNotEqual, format!("!{}", self.current_char), self.pos.clone()));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(TokenType::TTNot, "!".to_string(), self.pos.clone()));
                    }
                },
                '=' => {
                    self.advance();
                    if self.current_char == '=' {
                        self.tokens.push(Token::new(TokenType::TTDoubleEqual, format!("={}", self.current_char), self.pos.clone()));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(TokenType::TTEqual, "=".to_string(), self.pos.clone()));
                    }
                },
                '<' => {
                    self.advance();
                    if self.current_char == '=' {
                        self.tokens.push(Token::new(TokenType::TTLessThanEqual, format!("<{}", self.current_char), self.pos.clone()));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(TokenType::TTLessThanEqual, "<".to_string(), self.pos.clone()));
                    }
                },
                '>' => {
                    self.advance();
                    if self.current_char == '=' {
                        self.tokens.push(Token::new(TokenType::TTGreaterThanEqual, format!(">{}", self.current_char), self.pos.clone()));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(TokenType::TTGreaterThan, ">".to_string(), self.pos.clone()));
                    }
                },
                '#' => {
                    self.skip_comment();
                },
                '"' => {
                    self.advance();
                    self.make_string();
                },
                '\n' => {
                    self.pos.advance(Some(self.current_char));
                    self.advance();
                },
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                _ => {
                    if self.current_char.is_numeric() {
                        self.make_number();
                    } else if self.current_char.is_alphabetic() {
                        self.make_identifier();
                    } else {
                        self.lang.error(self.pos.clone(), ErrorType::ExpectedCharError, format!("Unexpected Character '{}'", self.current_char),);
                        self.advance()
                    };
                },
            }
            println!("\"{}\" {}", self.current_char, self.pos)
        }

        self.tokens.push(Token::new(TokenType::TTEndOfFile, "EOF".to_string(), self.pos.clone()));
        
        for token in self.tokens.clone() {
            println!("{}", token.as_string());
        }
        return self.tokens.clone();
    }

    fn make_identifier(& mut self) {
        let mut key = String::new();
        let mut is_keyword: bool = false;

        while self.current_char.is_alphanumeric() || self.current_char == '_' {
            key.push(self.current_char);
            self.advance();
        }

        for keyword in self.keywords.keywords.clone() {
            if key.clone() == keyword.name {
                self.tokens.push(Token::new(keyword.tok_type, key.clone(), self.pos.clone()));
                is_keyword = true;
            }
        }

        if !is_keyword {
            self.tokens.push(Token::new(TokenType::TTIdentifier, key.clone(), self.pos.clone()));
        }        
    }

    fn make_number(&mut self) {
        let mut num_str = String::new();
        let mut dot_count = 0;

        while (self.current_char.is_numeric() || self.current_char == '.') && !self.is_at_end() {
            if self.current_char == '.' {
                if dot_count == 1 {
                    self.lang.error(self.pos.clone(), ErrorType::ExpectedCharError, "Cannot have more than one decimal point in a number".to_string());
                    break;
                }
                dot_count += 1;
            }
            num_str.push(self.current_char);
            self.advance();
        }

        if dot_count == 0 {
            self.tokens.push(Token::new(TokenType::TTInt, num_str, self.pos.clone()));
        } else {
            self.tokens.push(Token::new(TokenType::TTFloat, num_str, self.pos.clone()));
        }
    }

    fn make_string(&mut self) {
        let mut string = String::new();

        while self.current_char != '"' && !self.is_at_end() {
            if self.current_char == '\n'{
                self.pos.advance(Some('\n'));
            }
            string.push(self.current_char);
            self.advance();
        }

        if self.is_at_end() || self.current_char != '"' {
            self.lang.error(self.pos.clone(), ErrorType::ExpectedCharError, "Expected '\"'".to_string());
        }

        self.advance();
        self.tokens.push(Token::new(TokenType::TTString, string, self.pos.clone()));
    }

    fn skip_comment(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.current_char == '\n' {
                break;
            }
            self.advance();
        }
        self.advance();
    }

    fn is_at_end(&self) -> bool {
        return self.pos.idx as usize >= self.source.len() - 1;
    }
}