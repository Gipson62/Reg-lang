use crate::basic::{
    position::Position,
    token::{
        Token,
        TokenType,
        Keywords,
        Characters,
    },
};
use std::any::{
    Any,
    TypeId
};

pub(crate) struct Lexer {
    pub file_name:String,
    pub text:String,
    pub pos:Position,
    pub current_char:char,
    pub chars:Characters,
}

impl Lexer {
    pub fn new(filename:String, txt:String) -> Lexer {
        Lexer {
            file_name: filename.clone(),
            text:txt.clone(),
            pos: Position::new(
                0,
                0,
                0,
                filename.clone(),
                txt.clone(),
            ),
            current_char: '\0',
            chars:Characters::new(),

        }
    }

    /*pub fn make_tokens(self) -> Result<Vec<Token>, Error> {
        let mut tokens:Vec<Token> = Vec::new();

        while self.current_char != '\0' {
            if self.current_char == self.chars.skip_letters{
                self.advance();
            } else if self.current_char == self.chars.comment_symbol {
                self.skip_comment();
            } else if self.chars.digits.contains(&self.current_char) {
                tokens.push(self.make_number());
            }
        }

        return Ok(tokens)
    }

    fn advance(&mut self) {
        self.pos.advance(Some(self.current_char));
        self.current_char = if self.pos.idx < self.text.len() as u32 {
            self.text[self.pos.idx as usize]
        } else {
            '\0'
        }
    }*/
    fn advance(&mut self) {
        todo!()
    }
    fn skip_comment(&mut self) {
        self.advance();
    
        while self.current_char != self.chars.new_line && self.current_char != '\0' {
            self.advance();
        }

        self.advance();
    }
    fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let pos_start = self.pos.clone();

        while self.current_char != '\0' && self.chars.digits.contains(&self.current_char) {
            num_str.push(self.current_char);
            self.advance();
            if self.current_char == '.' {
                if dot_count == 1 { 
                    dot_count += 1;
                    break 
                }                
                num_str.push(self.current_char);
                self.advance();
            }
        }

        if dot_count == 0 {
            //let num = num_str.parse::<i32>().unwrap();
            return Token::new(TokenType::TTInt, pos_start, self.pos.clone(), num_str);
        } else {
            //let num = num_str.parse::<f32>().unwrap();
            return Token::new(TokenType::TTFloat, pos_start, self.pos.clone(), num_str)
        }
    }
    fn make_string(self) -> Token {
        todo!()
    }
    fn make_identifier(self) -> Token {
        todo!()
    }
    fn make_minus_or_arrow(self) -> Token {
        todo!()
    }
    fn make_equals(self) -> Token {
        todo!()
    }
    fn make_not_equals(self) -> Token {
        todo!()
    }
    fn make_less_than(self) -> Token {
        todo!()
    }
    fn make_greater_than(self) -> Token {
        todo!()
    }
}