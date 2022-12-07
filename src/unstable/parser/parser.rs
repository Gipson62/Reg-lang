use crate::{
    unstable::{
        tokens::{
            tokens::Token,
            tokens_type::TokenType,
        },
        parser::{
            nodes::{
                Expr,
                LiteralValue
            },
            parse_error::ParseError,
        },
    },
    Language
};
// TODO!
// Write the Parser
pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
    lang: Language,
}
impl Parser {
    pub fn new(tokens: Vec<Token>, lang: Language) -> Self {
        Parser {
            tokens,
            current: 0,
            lang,
        }
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while(self.matches(vec![TokenType::TTNotEqual, TokenType::TTDoubleEqual])) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while(self.matches(vec![TokenType::TTGreaterThan, TokenType::TTGreaterThanEqual, TokenType::TTLessThan, TokenType::TTLessThanEqual])) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while(self.matches(vec![TokenType::TTMinus, TokenType::TTPlus])) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary{
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while(self.matches(vec![TokenType::TTSlash, TokenType::TTStar])) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary{
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.matches(vec![TokenType::TTNot, TokenType::TTMinus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.matches(vec![TokenType::TTFalse]) {
            return Expr::Literal {
                value: LiteralValue::False,
            };
        }
        if self.matches(vec![TokenType::TTTrue]) {
            return Expr::Literal {
                value: LiteralValue::True,
            };
        }
        if self.matches(vec![TokenType::TTNone]) {
            return Expr::Literal {
                value: LiteralValue::None,
            };
        }
        if self.matches(vec![TokenType::TTFloat, TokenType::TTString]) {
            return Expr::Literal {
                value: LiteralValue::BaseNumberFloat(self.previous().lexeme.parse::<f64>().unwrap()),
            };
        }
        if self.matches(vec![TokenType::TTString]) {
            return Expr::Literal {
                value: LiteralValue::String(self.previous().lexeme),
            };
        }
        if self.matches(vec![TokenType::TTLParen]) {
            let mut expr: Expr = self.expression();
            self.consume(TokenType::TTRParen, "Expected ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        } else {
            panic!("What you doin' ?");
        }
    }
    /// Return true if the current token matches one of the types
    fn matches(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        
        return false;
    }
    /// Advance the current token and return the previous one
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        
        return self.previous();
    }
    /// Return the current token and increment the current token index
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        return self.peek().tok_type == token_type;
    }
    /// Return true if the current token is the end of the file
    fn is_at_end(&self) -> bool {
        return self.peek().tok_type == TokenType::TTEndOfFile;
    }
    /// Return the previous token
    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
    /// Return the current token
    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn error(&mut self, token: Token, message: &str) -> ParseError {
        self.lang.parse_error(token, message);
        return ParseError::new();
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }
        panic!("{}", self.error(self.peek(), message));
    }
}