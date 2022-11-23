use crate::core::{
    errors::errors::{
        Error,
        ErrorType,
    },
    tokens::{
    position::Position,
    token::{
        Token,
        TokenType,
        Keywords,
        Characters,
        }
    },
};

/// The [`Lexer`] is responsible for taking the source code and turning it into a list of [`Token`].
pub(crate) struct Lexer {
    pub file_name:String,
    pub txt:Vec<char>,
    pub pos:Position,
    pub current_char:char,
}
impl Lexer {
    /// Creates a new [`Lexer`].
    pub fn new(filename:String, txt:String) -> Lexer {
        let mut lexer = Lexer {
            file_name: filename.clone(),
            pos: Position::new(
                0,
                0,
                0,
                filename.clone(),
            ),
            txt: txt.chars().collect(),
            current_char: '\0',
        };
        lexer.advance();
        return lexer
    }
    /// Advance the position of the lexer to the next char.
    fn advance(&mut self) {
        self.pos.advance(Some(self.current_char));
        self.current_char = if self.pos.idx < self.txt.len() as u32 + 1 {
            self.txt[self.pos.idx as usize -1 ]
        } else {
            '\0'
        };
    }
    /// Creates a new [`Token`] with the given [`TokenType`] for each characters in the text.
    pub fn make_tokens(&mut self) -> Vec<Token>{
        let mut tokens:Vec<Token> = Vec::new();
        let characters = Characters::new();

        while self.current_char != '\0' {
            if self.current_char == characters.skip_letters {
                self.advance();
            } else if self.current_char == characters.comment_symbol {
                self.skip_comment();
            } else if characters.digits.contains(&self.current_char) {
                tokens.push(self.make_number(characters.clone()));
            } else if characters.letters.contains(&self.current_char) {
                tokens.push(self.make_identifier(characters.clone()));
            } else if characters.symbols.contains(&self.current_char) {
                tokens.push(self.make_symbols().unwrap());
                self.advance();
            } else if self.current_char == ' ' {
                self.advance();
            } 
            else {
                let pos_start = self.pos.clone();
                let pos_end = self.pos.clone();
                let error = Error::new(pos_start, pos_end, format!("Unexpected character: '{}'", self.current_char), ErrorType::IllegalCharError);
                panic!("{}", error.as_string());
            }
        }
        tokens.push(Token::new(TokenType::TTEndOfLine, self.pos.clone(), self.pos.clone(), '\0'.to_string()));
        for token in tokens.clone() {
            println!("[{}] ", token.to_string());
        }
        return tokens
    }
    /// Make the token for all the symbols supported (except for the comment symbol)
    /// 
    /// Symbols supported (for the moment): + * / ^ ( ) [ ] , ; \n \t > < = ! " -
    /// 
    /// - Todo : add '{' '}' => struct/dictionaries
    /// - Todo : add ':' => type hinting 
    /// - Todo : add '%' => modulo operator
    ///     - Add TypeHinting
    fn make_symbols(&mut self) -> Option<Token> {
        match self.current_char {
            '+' => {
                return Some(Token::new(TokenType::TTPlus, self.pos.clone(), self.pos.clone(), String::from("+")))
            }
            '(' => {
                return Some(Token::new(TokenType::TTLParen, self.pos.clone(), self.pos.clone(), String::from("(")))
            }
            ')' => {
                return Some(Token::new(TokenType::TTRParen, self.pos.clone(), self.pos.clone(), String::from(")")))
            }
            '[' => {
                return Some(Token::new(    TokenType::TTLSquare,    self.pos.clone(),    self.pos.clone(),    String::from("["))
                )
            }
            ']' => {
                return Some(Token::new(    TokenType::TTRSquare,    self.pos.clone(),    self.pos.clone(),    String::from("]"))
                )
            }
            '*' => {
                return Some(Token::new(    TokenType::TTMultiply,    self.pos.clone(),    self.pos.clone(),    String::from("*"))
                )
            }
            '/' => {
                return Some(Token::new(    TokenType::TTDivide,    self.pos.clone(),    self.pos.clone(),    String::from("/"))
                )
            }
            '^' => {
                return Some(Token::new(    TokenType::TTPower,    self.pos.clone(),    self.pos.clone(),    String::from("^"))
                )
            }
            ',' => {
                return Some(Token::new(    TokenType::TTComma,    self.pos.clone(),    self.pos.clone(),    String::from(","))
                )
            }
            ';' => {
                return Some(Token::new(    TokenType::TTSemicolon,    self.pos.clone(),    self.pos.clone(),    String::from(";"))
                )
            }
            '\n' => {
                return Some(Token::new(    TokenType::TTNewLine,    self.pos.clone(),    self.pos.clone(),    String::from("\n"))
                )
            }
            '\t' => {
                return Some(Token::new(    TokenType::TTTabulation,    self.pos.clone(),    self.pos.clone(),    String::from("\t"))
                )
            }
            '"' => {
                return Some(self.make_string())
            }
            '-' => {
                return Some(self.make_minus_or_arrow())
            }
            '=' => {
                return Some(self.make_equals())
            }
            '!' => {
                return Some(self.make_not_equals())
            }
            '>' => {
                return Some(self.make_greater_than())
            }
            '<' => {
                return Some(self.make_less_than())
            }
            _ => {
                return None
            }
        }
    }
    /// Make the token for '<' and '<=' (if the next char is '=')
    fn make_less_than(&mut self) -> Token {
        let pos_start = self.pos.clone();
        self.advance();
        if self.current_char == '=' {
            self.advance();
            return Token::new(
                TokenType::TTLessThanEqual,
                pos_start,
                self.pos.clone(),
                String::from("<=")
            )
        }
        return Token::new(
            TokenType::TTLessThan,
            pos_start,
            self.pos.clone(),
            String::from("<")
        )
    }
    /// Make the token for '>' or '>=' (if the next char is '=') 
    fn make_greater_than(&mut self) -> Token {
        let pos_start = self.pos.clone();
        self.advance();
        if self.current_char == '=' {
            self.advance();
            return Token::new(
                TokenType::TTGreaterThanEqual,
                pos_start,
                self.pos.clone(),
                String::from(">=")
            )
        } else {
            return Token::new(
                TokenType::TTGreaterThan,
                pos_start,
                self.pos.clone(),
                String::from(">")
            )
        }
    }
    /// Make the token for '!=' (if the next char isn't '=' Lexer will panic and return an error)
    fn make_not_equals(&mut self) -> Token {
        let pos_start = self.pos.clone();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Token::new(
                TokenType::TTNotEqual,
                pos_start,
                self.pos.clone(),
                String::from("!=")
            )
        }
        self.advance();
        let error = Error::new(
            pos_start,
            self.pos.clone(),
            String::from("Expected '=' after '!'"),
            ErrorType::ExpectedCharError
        );
        panic!("{}",error.as_string());
    }
    /// Make the token for the equal or double equal symbol.
    fn make_equals(&mut self) -> Token {
        let pos_start = self.pos.clone();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Token::new(
                TokenType::TTDoubleEqual,
                pos_start,
                self.pos.clone(),
                String::from("==")
            )
        }

        return Token::new(
            TokenType::TTEqual,
            pos_start,
            self.pos.clone(),
            String::from("=")
        )
    }
    /// Create the minus or the arrow [`Token`]
    /// 
    /// Arrow is used for the function definition
    fn make_minus_or_arrow(&mut self) -> Token {
        let pos_start = self.pos.clone();
        self.advance();

        if self.current_char == '>' {
            self.advance();
            return Token::new(
                TokenType::TTArrow,
                pos_start,
                self.pos.clone(),
                String::from("->")
            )
        } else {
            return Token::new(
                TokenType::TTMinus,
                pos_start,
                self.pos.clone(),
                String::from("-"))

        }
    }
    /// Create the string [`Token`] (string are between double quotes)
    /// 
    /// (char is not supported yet)
    fn make_string(&mut self) -> Token {
        let mut string = String::new();
        let pos_start = self.pos.clone();
        let mut escape_character = false;
        self.advance();

        let escape_characters = ['n', 't',];

        while self.current_char != '\0' && (self.current_char != '"' || escape_characters.contains(&self.current_char)) {
            if escape_character {
                if escape_characters.contains(&self.current_char) {
                    string.push(char::from(92));
                    string.push(self.current_char);
                }
            } else {
                if self.current_char == char::from(92) {
                    escape_character = true;
                } else {
                    string.push(self.current_char);
                }
            }
            self.advance();
            escape_character = false;
        }
        if self.current_char != '"' {
            let error = Error::new(
                pos_start.clone(),
                self.pos.clone(),
                String::from("Expected '\"' at the end of a string"),
                ErrorType::InvalidSyntaxError,
            );
            panic!("{}", error.as_string());
        }
        self.advance();
        return Token::new(
            TokenType::TTString,
            pos_start,
            self.pos.clone(),
            string
        )
    }
    /// Skip the line commented with `#`
    fn skip_comment(&mut self) {
        self.advance();

        while self.current_char != '\0' || self.current_char != '\n' {
            self.advance();
        }

        self.advance();
    }
    /// Create the int or the float [`Token`] (the usigned int is not supported)
    fn make_number(&mut self, characters:Characters) -> Token {
        let mut num_str = String::new();
        let mut dot_count:u8 = 0;
        let pos_start = self.pos.clone();

        while self.current_char != '\0' && (characters.digits.contains(&self.current_char) || self.current_char == '.') {
            if self.current_char == '.' {
                if dot_count == 1 {
                    break;
                }
                dot_count += 1;
            }
            num_str.push(self.current_char);
            self.advance()
        }

        if dot_count == 0 {
            return Token::new(
                TokenType::TTInt,
                pos_start,
                self.pos.clone(),
                num_str,
            )
        } else {
            return Token::new(
                TokenType::TTFloat,
                pos_start,
                self.pos.clone(),
                num_str,
            )
        }
    }
    /// Create the identifier or keyword [`Token`]
    /// 
    /// One is for the variable/func create by the user
    /// The other is for all the keywords in this language
    fn make_identifier(&mut self, characters:Characters) -> Token {
        let mut keyword = String::new();
        let pos_start = self.pos.clone();
        let all_keywords = Keywords::new();

        while self.current_char != '\0' && (characters.letters.contains(&self.current_char) || self.current_char == '_' ) {
            keyword.push(self.current_char);
            self.advance();
        }

        for key in all_keywords.iter() {
            if key == &keyword {
                return Token::new(TokenType::TTKeyword,pos_start,self.pos.clone(),keyword
                )
            }
        }
        return Token::new(TokenType::TTIdentifier, pos_start, self.pos.clone(), keyword)
    }
}