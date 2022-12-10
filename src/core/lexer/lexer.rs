use crate::{
    core::{
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
/// The `Lexer` is responsible for taking the source code and turning it into a list of `Tokens`.
pub(crate) struct Lexer {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub pos: Position,
    pub current_char: char,
    pub lang: Language,
    pub keywords: Keywords,
}
impl Lexer {
    /// Create a new `Lexer` instance.
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
    /// Advance the position of the `Lexer` to the next char.
    fn advance(&mut self) {
        self.pos.advance(Some(self.current_char.clone()));
        
        if !self.is_at_end() {
            self.current_char = self.source[self.pos.idx as usize];
        }
    }
    /// Create a new [`Token`] with the given [`TokenType`] for each characters in the source code.
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
                        self.advance();
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
                '/' => {
                    self.tokens.push(Token::new(TokenType::TTSlash, self.current_char.to_string(), self.pos.clone()));
                    self.advance();
                }
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
    /// Make a `Keyword` or `Identifier` token based on the `Keyword` struct.
    fn make_identifier(& mut self) {
        let mut key = String::new();
        let mut is_keyword: bool = false;

        while self.current_char.is_alphanumeric() || self.current_char == '_' {
            key.push(self.current_char);
            self.advance();
        }

        for keyword in self.keywords.keywords.clone() {
            if key == keyword.name {
                self.tokens.push(Token::new(keyword.tok_type, key.clone(), self.pos.clone()));
                is_keyword = true;
                break;
            }
        }

        if !is_keyword {
            self.tokens.push(Token::new(TokenType::TTIdentifier, key.clone(), self.pos.clone()));
        }        
    }
    /// Make an `Int` or a `Float` token based on if there's a `.` or not.
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
    /// Make a `String` token if there's no `"` at the end of the string, that add an error to the `Lang` struct.
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
    /// Skip a comment if there's a `#` at the start of the line.
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
    /// Check if the lexer is at the end of the file.
    fn is_at_end(&self) -> bool {
        return self.pos.idx as usize >= self.source.len() - 1;
    }
}