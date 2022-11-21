///Import errors
use crate::core::{
    errors::errors::Error,
    nodes::nodes::{
        Nodes,
    }, tokens::token::Token,
};

#[derive(Debug, Clone)]
pub(crate) struct ParseResult {
    pub error: Option<Error>,
    pub node: Option<Nodes>,
    pub last_register_advance_count: u32,
    pub advance_count: u32,
    pub to_reverse_count: u32,
}
impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
            error: None,
            node: None,
            last_register_advance_count: 0, 
            advance_count: 0,
            to_reverse_count: 0,
        }
    }
    pub fn register_advancement(&mut self) {
        self.last_register_advance_count = 1;
        self.advance_count += 1;
    }
    pub fn register(&mut self, res: ParseResult) -> Option<Nodes> {
        self.last_register_advance_count = res.advance_count;
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.clone().error;
        }
        return res.clone().node
    }
    pub fn try_register(&mut self, res: ParseResult) -> Option<Nodes> {
        if res.error.is_some() {
            self.to_reverse_count = res.advance_count;
            return None
        }
        return self.register(res)
    }
    pub fn success(&mut self, node: Nodes) -> Option<Nodes> {
        self.node = Some(node);
        return self.node.clone()
    }
    pub fn failure(&mut self, error: Error) -> &ParseResult {
        if self.last_register_advance_count == 0 {
            self.error = Some(error);
        }
        return self
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: usize,
    pub current_tok: Option<Token>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut parser = Parser {
            tokens,
            token_index: 0,
            current_tok: None,
        };
        parser.advance();
        return parser
    }
    pub fn advance(&mut self) -> Token{
        self.token_index += 1;
        self.update_current_tok();
        return self.current_tok.clone().unwrap();
    }
    pub fn update_current_tok(& mut self) {
        if self.token_index < self.tokens.len() + 1 {
            self.current_tok = Some(self.tokens[self.token_index - 1].clone())
        } else {
            self.current_tok = None;
        }
    }
    pub fn parse(&mut self) -> ParseResult {
        let mut res = self.statements();
        todo!()
    }
    pub fn statements(&mut self) {
        todo!()
    }
    pub fn statement(&mut self) {
        todo!()
    }
    pub fn expr(&mut self) {
        todo!()
    }
    pub fn comp_expr(&mut self) {
        todo!()
    }
    pub fn arith_expr(&mut self) {
        todo!()
    }
    pub fn term(&mut self) {
        todo!()
    }
    pub fn factor(&mut self) {
        todo!()
    }
    pub fn power(&mut self) {
        todo!()
    }
    pub fn call(&mut self) {
        todo!()
    }
    pub fn atom(&mut self) {
        todo!()
    }
    pub fn list_expr(&mut self) {
        todo!()
    }
    pub fn if_expr(&mut self) {
        todo!()
    }
    pub fn if_expr_b(&mut self) {
        todo!()
    }
    pub fn if_expr_c(&mut self) {
        todo!()
    }
    pub fn if_expr_b_or_c(&mut self) {
        todo!()
    }
    pub fn if_expr_cases(&mut self) {
        todo!()
    }
    pub fn for_expr(&mut self) {
        todo!()
    }
    pub fn while_expr(&mut self) {
        todo!()
    }
    pub fn func_def(&mut self) {
        todo!()
    }
    pub fn bin_op(&mut self) {
        todo!()
    }
}