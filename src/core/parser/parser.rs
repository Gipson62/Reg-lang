///Import errors
use crate::core::{
    errors::errors::{Error, ErrorType},
    nodes::nodes::{
        Nodes,
        StringNode,
        NumberNode,
        BinOpNode,
        UnaryOpNode,
        VarAccessNode,
        VarAssignNode,
        IfNode,
        ForNode,
        WhileNode,
        FuncDefNode,
        CallNode,
        ReturnNode,
        ContinueNode,
        BreakNode,
        ListNode,
    }, tokens::token::{Token, TokenType, Keywords},
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParseResult {
    pub error: Option<Error>,
    pub nodes: Vec<Option<Nodes>>,
    pub last_register_advance_count: u32,
    pub advance_count: u32,
    pub to_reverse_count: u32,
}
impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
            error: None,
            nodes: vec![None,],
            last_register_advance_count: 0, 
            advance_count: 0,
            to_reverse_count: 0,
        }
    }
    
    pub fn register_advancement(&mut self) {
        self.last_register_advance_count = 1;
        self.advance_count += 1;
    }
    
    pub fn register(&mut self, res: ParseResult) -> Vec<Option<Nodes>> {
        self.last_register_advance_count = res.advance_count;
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.clone().error;
        }
        return res.clone().nodes.clone()
    }
    
    pub fn try_register(&mut self, res: ParseResult) -> Vec<Option<Nodes>> {
        if res.error.is_some() {
            self.to_reverse_count = res.advance_count;
            return vec![None]
        }
        return self.register(res)
    }
    
    pub fn success(&mut self, node: Vec<Option<Nodes>>) -> ParseResult {
        self.nodes = node;
        return self.clone()
    }
    
    pub fn failure(mut self, error: Error) -> ParseResult {
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
        if self.current_tok.is_some() {
            return self.current_tok.clone().unwrap()
        } else {
            panic!("No current token PANIC")
        }
    }
    
    pub fn reverse(&mut self, amount: u32) -> Option<Token>{
        self.token_index -= amount as usize;
        self.update_current_tok();
        return self.current_tok.clone();
    }
    
    pub fn update_current_tok(& mut self) {
        if self.token_index < self.tokens.len() + 1 {
            self.current_tok = Some(self.tokens[self.token_index - 1].clone());
            println!("Current token: {}", self.tokens.clone()[self.token_index.clone() - 1].clone().to_string());
        }
    }
    
    pub fn parse(&mut self) -> ParseResult {
        let mut res = self.statements();
        if res.error.is_some() &&  self.current_tok.clone().unwrap().token_type == TokenType::TTEndOfLine {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Token cannot appear after previous tokens"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }
        return res
    }
    
    pub fn statements(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut statements = Vec::new();
        let pos_start = self.current_tok.clone().unwrap().pos_start.clone();

        while self.current_tok.clone().unwrap().token_type == TokenType::TTNewLine {
            res.register_advancement();
            self.advance();
        }

        let statement = res.register(self.statement())[0].clone();
        if res.error.is_some() {
            return res
        }
        statements.push(statement.clone());
        let mut more_statements = true;

        loop {
            let mut newline_count = 0;
            while self.current_tok.clone().unwrap().token_type == TokenType::TTNewLine {
                res.register_advancement();
                self.advance();
                newline_count += 1;
            }
            if newline_count == 0 {
                more_statements = false;
            }

            if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("end")) {
                break;
            }

            if self.current_tok.clone().unwrap().token_type == TokenType::TTEndOfLine {
                break;
            }

            if !more_statements {
                break;
            }
            if statement == None {
                self.reverse(res.to_reverse_count);
                more_statements = false;
                continue;
            }
            statements.push(statement.clone());
        }
        return res.success(
            vec![Some(
                Nodes::List(
                    ListNode::new(
                        statements,
                        pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                    )
                )
            )]
        )
    }
    
    pub fn statement(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_tok.clone().unwrap().pos_start;

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("return")) {
            res.register_advancement();
            self.advance();

            let expr = res.try_register(self.expr())[0].clone();

            if expr == None {
                self.reverse(res.to_reverse_count);
            }
            return  res.success(
                vec![Some(
                    Nodes::Return(
                        ReturnNode::new(
                            vec![expr.unwrap()],
                            pos_start,
                            self.current_tok.clone().unwrap().pos_end,
                        )
                    )
                )]
            )
        }

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("continue")) {
            res.register_advancement();
            self.advance();
            return res.success(
                vec![Some(
                    Nodes::Continue(
                        ContinueNode::new(
                            pos_start,
                            self.current_tok.clone().unwrap().pos_end,
                        )
                    )
                )]
            )
        }

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("break")) {
            res.register_advancement();
            self.advance();
            return res.success(
                vec![Some(
                    Nodes::Break(
                        BreakNode::new(
                            pos_start,
                            self.current_tok.clone().unwrap().pos_end
                        )
                    )
                )]
            )
        }

        let expr = res.register(self.expr());
        if res.error.is_some() {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected: 'return', 'continue', 'break', 'var', 'if', 'for', 'while', 'func', an Int, a Float, an Identifier, '+', '-', '(', '['"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }
        return res.success(expr)
    }
    
    pub fn expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("var")) {
            res.register_advancement();
            self.advance();

            if self.current_tok.clone().unwrap().token_type != TokenType::TTIdentifier {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected Identifier"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }

            let var_name = self.current_tok.clone().unwrap();
            res.register_advancement();
            self.advance();

            if self.current_tok.clone().unwrap().token_type != TokenType::TTEqual {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected '='"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }

            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr())[0].clone();
            if res.error.is_some() {
                return res
            }
            return res.success(
                vec![Some(
                    Nodes::VarAssign(
                        VarAssignNode::new(
                            var_name,
                            expr.unwrap(),
                        )
                    )
                )]
            )
        }
        let comp_expr_a = self.comp_expr();
        self.advance();
        let comp_expr_b = self.comp_expr();
        self.reverse(1);
        let node = res.register(
            self.bin_op(
                comp_expr_a,
                vec![(TokenType::TTKeyword, String::from("and")),
                (TokenType::TTKeyword, String::from("or"))],
                comp_expr_b,
            )
        );
        if res.error.is_some() {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'var', 'if', 'for', 'while', 'func', 'not' Int, Float, Identifier, '+', '-', '(', '['"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        return res.success(node)
    }

    pub fn comp_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("not")) {
            let op_tok = self.current_tok.clone().unwrap();
            res.register_advancement();
            self.advance();

            let node = res.register(self.comp_expr());
            if res.error.is_some() {
                return res
            }
            return res.success(
                    vec![Some(
                    Nodes::UnaryOp(
                        UnaryOpNode::new(
                            op_tok,
                            vec![node[0].clone().unwrap()],
                        )
                    )
                )]
            )
        }

        let arith_expr = self.arith_expr();
        self.advance();
        let arith_expr_b = self.arith_expr();
        self.reverse(1);
        let node = res.register(
            self.bin_op(
                arith_expr,
                vec![
                    (TokenType::TTDoubleEqual, String::from("==")),
                    (TokenType::TTNotEqual, String::from("!=")),
                    (TokenType::TTLessThan, String::from("<")),
                    (TokenType::TTGreaterThan, String::from(">")),
                    (TokenType::TTLessThanEqual, String::from("<=")),
                    (TokenType::TTGreaterThanEqual, String::from(">=")),
                ], 
                arith_expr_b
            )
        );
        if res.error.is_some() {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'if', 'for', 'while', 'func', 'not' Int, Float, Identifier, '+', '-', '(', '['"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        return res.success(node)
    }
    
    pub fn arith_expr(&mut self) -> ParseResult {
        let term_a = self.term();
        self.advance();
        let term_b = self.term();
        self.reverse(1);
        return self.bin_op(
            term_a, 
            vec![
                (TokenType::TTPlus, String::from("+")),
                (TokenType::TTMinus, String::from("-")),
            ],
            term_b
        )
    }
    
    pub fn term(&mut self) -> ParseResult {
        let factor_a = self.factor();
        self.advance();
        let factor_b = self.factor();
        self.reverse(1);
        return self.bin_op(
            factor_a, 
            vec![
                (TokenType::TTMultiply, String::from("*")),
                (TokenType::TTDivide, String::from("/")),
            ],
            factor_b
        )
    }
    
    pub fn factor(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let tok = self.current_tok.clone().unwrap();

        if tok.token_type == TokenType::TTPlus || tok.token_type == TokenType::TTMinus {
            res.register_advancement();
            self.advance();
            let factor = res.register(self.factor())[0].clone();
            if res.error.is_some() {
                return res
            }
            return res.success(
                vec![Some(
                    Nodes::UnaryOp(
                        UnaryOpNode::new(
                            tok,
                            vec![factor.unwrap()],
                        )
                    )
                )]
            )
        }

        return self.power()
    }
    
    pub fn power(&mut self) -> ParseResult {
        let call_a = self.call();
        self.advance();
        let call_b = self.call();
        self.reverse(1);
        return self.bin_op(
            call_a, 
            vec![
                (TokenType::TTPower, String::from("^")),
            ],
            call_b
        )
    }
    
    pub fn call(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let atom = res.register(self.atom());
        if res.error.is_some() {
            return res
        }

        if self.current_tok.clone().unwrap().token_type == TokenType::TTLParen {
            res.register_advancement();
            self.advance();
            let mut arg_nodes = Vec::new();

            if self.current_tok.clone().unwrap().token_type == TokenType::TTRParen {
                res.register_advancement();
                self.advance();
            } else {
                arg_nodes.push(res.register(self.expr())[0].clone());
                if res.error.is_some() {
                    return res.failure(
                        Error::new(
                            self.current_tok.clone().unwrap().pos_start,
                            self.current_tok.clone().unwrap().pos_end,
                            String::from("Expected ')', 'var', 'if', 'for', 'while', 'func', 'not' Int, Float, Identifier, '+', '-', '(', '['"),
                            ErrorType::InvalidSyntaxError,
                        )
                    )
                }

                while self.current_tok.clone().unwrap().token_type == TokenType::TTComma {
                    res.register_advancement();
                    self.advance();

                    arg_nodes.push(res.register(self.expr())[0].clone());
                    if res.error.is_some() {
                        return res
                    }
                }

                if self.current_tok.clone().unwrap().token_type != TokenType::TTRParen {
                    return res.failure(
                        Error::new(
                            self.current_tok.clone().unwrap().pos_start,
                            self.current_tok.clone().unwrap().pos_end,
                            String::from("Expected ',' or ')'"),
                            ErrorType::InvalidSyntaxError,
                        )
                    )
                }
            }
            return res.success(
                vec![Some(
                    Nodes::Call(
                        CallNode::new(
                            vec![atom[0].clone().unwrap()],
                            arg_nodes,
                        )
                    )
                )]
            )
        }
        return res.success(atom)
    }
    
    pub fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let tok = self.current_tok.clone().unwrap();

        if tok.token_type == TokenType::TTInt || tok.clone().token_type == TokenType::TTFloat {
            res.register_advancement();
            self.advance();
            return res.success(
                vec![Some(
                    Nodes::Number(
                        NumberNode::new(
                            tok,
                        )
                    )
                )]
            )
        } else if tok.token_type == TokenType::TTString {
            res.register_advancement();
            self.advance();
            return res.success(
                vec![Some(
                    Nodes::String(
                        StringNode::new(
                            tok,
                        )
                    )
                )]
            )
        } else if tok.token_type == TokenType::TTIdentifier {
            res.register_advancement();
            self.advance();
            return res.success(
                vec![Some(
                    Nodes::VarAccess(
                        VarAccessNode::new(
                            tok,
                        )
                    )
                )]
            )
        } else if tok.token_type == TokenType::TTLParen {
            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res
            }
            if self.current_tok.clone().unwrap().token_type == TokenType::TTRParen {
                res.register_advancement();
                self.advance();
                return res.success(expr)
            } else {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected ')'"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }
        } else if tok.token_type == TokenType::TTLSquare {
            let list_expr = res.register(self.list_expr());
            if res.error.is_some() {
                return res
            }
            return res.success(list_expr)
        } else if tok.clone().matches(TokenType::TTKeyword, String::from("if")) {
            let if_expr = res.register(self.if_expr());
            if res.error.is_some() {
                return res
            }
            return res.success(if_expr)
        } else if tok.clone().matches(TokenType::TTKeyword, String::from("for")) {
            let for_expr = res.register(self.for_expr());
            if res.error.is_some() {
                return res
            }
            return res.success(for_expr)
        } else if tok.clone().matches(TokenType::TTKeyword, String::from("while")) {
            let while_expr = res.register(self.while_expr());
            if res.error.is_some() {
                return res
            }
            return res.success(while_expr)
        } else if tok.clone().matches(TokenType::TTKeyword, String::from("func")) {
            let func_def = res.register(self.func_def());
            if res.error.is_some() {
                return res
            }
            return res.success(func_def)
        }
        return res.failure(
            Error::new(
                tok.pos_start,
                tok.pos_end,
                String::from("Expected Int, Float, Identifier, '+', '-', '(', '[', 'if', 'for', 'while', 'func'"),
                ErrorType::InvalidSyntaxError,
            )
        )
    }
    
    pub fn list_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut element_nodes = Vec::new();
        let pos_start = self.current_tok.clone().unwrap().pos_start.clone();

        if self.current_tok.clone().unwrap().token_type != TokenType::TTLSquare {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected '['"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        if self.current_tok.clone().unwrap().token_type == TokenType::TTRSquare {
            res.register_advancement();
            self.advance();
        } else {
            element_nodes.push(res.register(self.expr())[0].clone());
            if res.error.is_some() {
                return res
            }

            while self.current_tok.clone().unwrap().token_type == TokenType::TTComma {
                res.register_advancement();
                self.advance();

                element_nodes.push(res.register(self.expr())[0].clone());
                if res.error.is_some() {
                    return res
                }
            }

            if self.current_tok.clone().unwrap().token_type == TokenType::TTRSquare {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected ',' or ']'"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }

            res.register_advancement();
            self.advance();
        }

        return res.success(
            vec![Some(
                Nodes::List(
                    ListNode::new(
                        element_nodes,
                        pos_start,
                        self.current_tok.clone().unwrap().pos_end.clone(),
                    )
                )
            )]
        )
    }
    
    pub fn if_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let all_cases = res.register(self.if_expr_cases(String::from("if")));
        if res.error.is_some() {
            return res
        }
        let cases = vec![all_cases[0].clone()];
        let else_case = if all_cases[all_cases.len() - 1].clone().is_some() {
            Some(all_cases[all_cases.len() - 1].clone().unwrap())
        } else {
            None
        };
        return res.success(
            vec![Some(
                Nodes::If(
                    IfNode::new(
                        cases,
                        vec![else_case],
                    )
                )
            )]
        )
    }
    
    pub fn if_expr_b(&mut self) -> ParseResult {
        return self.if_expr_cases(String::from("if"))
    }
    
    pub fn if_expr_c(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut statements = Vec::new();

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("else")) {
            res.register_advancement();
            self.advance();

            if self.current_tok.clone().unwrap().token_type == TokenType::TTNewLine {
                res.register_advancement();
                self.advance();

                statements = res.register(self.statements());
                if res.error.is_some() {
                    return res
                }

                if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("end")) {
                    res.register_advancement();
                    self.advance();
                } else {
                    return res.failure(
                        Error::new(
                            self.current_tok.clone().unwrap().pos_start,
                            self.current_tok.clone().unwrap().pos_end,
                            String::from("Expected 'end'"),
                            ErrorType::InvalidSyntaxError,
                        )
                    )
                }
            } else {
                statements = res.register(self.statement());
                if res.error.is_some() {
                    return res
                }
            }
        }

        return res.success(statements)
    }
    
    pub fn if_expr_b_or_c(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut cases = Vec::new();
        let mut else_case = None;

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("elif")) {
            let mut all_cases = res.register(self.if_expr_b());
            if res.error.is_some() {
                return res
            }
            cases = all_cases[0..all_cases.len() - 2].to_vec();
            else_case = if all_cases[all_cases.len() - 1].clone().is_some() {
                Some(all_cases[all_cases.len() - 1].clone().unwrap())
            } else {
                None
            };
        } else {
            else_case = res.register(self.if_expr_c())[0].clone();
            if res.error.is_some() {
                return res
            }
        }
        cases.push(else_case);
        return res.success(cases)
    }
    /// "Return" a vector containing Condition | Statements | Else Case
    pub fn if_expr_cases(&mut self, case_keyword: String) -> ParseResult {
        let mut res = ParseResult::new();
        let mut cases:Vec<Option<Nodes>> = Vec::new();
        let mut else_case: Option<Nodes> = None;

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, case_keyword.clone()) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from(format!("Expected '{}'", case_keyword.clone())),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        let condition = res.register(self.expr())[0].clone();
        if res.error.is_some() {
            return res
        }

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("then")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'then'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        if self.current_tok.clone().unwrap().token_type == TokenType::TTNewLine {
            res.register_advancement();
            self.advance();

            let statements = res.register(self.statements())[0].clone();
            if res.error.is_some() {
                return res
            }
            cases.push(condition);
            cases.push(statements);
            cases.push(else_case); // Ligne à revoir pour qu'il y ait quelque chose dans le else_case

            if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("end")) {
                res.register_advancement();
                self.advance();
            } else {
                let all_cases = res.register(self.if_expr_b_or_c());
                if res.error.is_some() {
                    return res
                }
                for case in all_cases {
                    cases.push(case);
                }
            }
        } else {
            let expr = res.register(self.statement())[0].clone();
            if res.error.is_some() {
                return res
            }
            cases.push(condition);
            cases.push(expr);
            cases.push(None);

            let all_cases = res.register(self.if_expr_b_or_c());
            if res.error.is_some() {
                return res
            }
            for case in all_cases {
                cases.push(case);
            }
        }
        return res.success(cases)
    }
    
    pub fn for_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("for")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'for'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        if self.current_tok.clone().unwrap().token_type != TokenType::TTIdentifier {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected identifier"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        let var_name = self.current_tok.clone().unwrap();
        res.register_advancement();
        self.advance();

        if self.current_tok.clone().unwrap().token_type != TokenType::TTEqual {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected '='"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        let start_value = res.register(self.expr())[0].clone().unwrap();
        if res.error.is_some() {
            return res
        }

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("to")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'to'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        let end_value = res.register(self.expr())[0].clone().unwrap();
        if res.error.is_some() {
            return res
        }
        
        let mut step_value = None;

        if self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("step")) {
            res.register_advancement();
            self.advance();

            step_value = res.register(self.expr())[0].clone();
            if res.error.is_some() {
                return res
            }
        }

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("then")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'then'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        if self.current_tok.clone().unwrap().token_type == TokenType::TTNewLine {
            res.register_advancement();
            self.advance();

            let body = res.register(self.statements())[0].clone().unwrap();
            if res.error.is_some() {
                return res
            }

            if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("end")) {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected 'end'"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }

            res.register_advancement();
            self.advance();

            return res.success(
                vec![Some(
                    Nodes::For(
                        ForNode::new(
                            var_name,
                            vec![start_value],
                            vec![end_value],
                            vec![step_value],
                            vec![body],
                            true,
                        )
                    )
                )]
            )
        }

        let body = res.register(self.statement())[0].clone().unwrap();
        if res.error.is_some() {
            return res
        }

        return res.success(
            vec![Some(
                Nodes::For(
                    ForNode::new(
                        var_name,
                        vec![start_value],
                        vec![end_value],
                        vec![step_value],
                        vec![body],
                        false,
                    )
                )
            )]
        )
    }
    
    pub fn while_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("while")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'while'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        let condition = res.register(self.expr())[0].clone().unwrap();
        if res.error.is_some() {
            return res
        }

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("then")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'then'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        if self.current_tok.clone().unwrap().token_type == TokenType::TTNewLine {
            res.register_advancement();
            self.advance();

            let body = res.register(self.statements())[0].clone().unwrap();
            if res.error.is_some() {
                return res
            }

            if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("end")) {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected 'end'"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }

            res.register_advancement();
            self.advance();

            return res.success(
                vec![Some(
                    Nodes::While(
                        WhileNode::new(
                            vec![condition],
                            vec![body],
                            true,
                        )
                    )
                )]
            )
        }

        let body = res.register(self.statement())[0].clone().unwrap();
        if res.error.is_some() {
            return res
        }

        return res.success(
            vec![Some(
                Nodes::While(
                    WhileNode::new(
                        vec![condition],
                        vec![body],
                        false,
                    )
                )
            )]
        )
    }
    
    pub fn func_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("func")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'func'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        let mut var_name_tok;

        if self.current_tok.clone().unwrap().token_type == TokenType::TTIdentifier {
            var_name_tok = self.current_tok.clone();
            res.register_advancement();
            self.advance();

            if self.current_tok.clone().unwrap().token_type != TokenType::TTLParen {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected '('"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }
        } else {
            var_name_tok = None;
            if self.current_tok.clone().unwrap().token_type != TokenType::TTLParen {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected identifier or '('"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }
        }

        res.register_advancement();
        self.advance();
        let mut args_name_toks = Vec::new();

        if self.current_tok.clone().unwrap().token_type == TokenType::TTIdentifier {
            args_name_toks.push(self.current_tok.clone().unwrap());
            res.register_advancement();
            self.advance();

            while self.current_tok.clone().unwrap().token_type == TokenType::TTComma {
                res.register_advancement();
                self.advance();

                if self.current_tok.clone().unwrap().token_type != TokenType::TTIdentifier {
                    return res.failure(
                        Error::new(
                            self.current_tok.clone().unwrap().pos_start,
                            self.current_tok.clone().unwrap().pos_end,
                            String::from("Expected identifier"),
                            ErrorType::InvalidSyntaxError,
                        )
                    )
                }

                args_name_toks.push(self.current_tok.clone().unwrap());
                res.register_advancement();
                self.advance();
            }
            
            if self.current_tok.clone().unwrap().token_type != TokenType::TTRParen {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected ',' or ')'"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }
        } else {
            if self.current_tok.clone().unwrap().token_type != TokenType::TTRParen {
                return res.failure(
                    Error::new(
                        self.current_tok.clone().unwrap().pos_start,
                        self.current_tok.clone().unwrap().pos_end,
                        String::from("Expected identifier or ')'"),
                        ErrorType::InvalidSyntaxError,
                    )
                )
            }
        }

        res.register_advancement();
        self.advance();

        let mut body;

        if self.current_tok.clone().unwrap().token_type == TokenType::TTArrow {
            res.register_advancement();
            self.advance();

            body = res.register(self.expr())[0].clone().unwrap();
            if res.error.is_some() {
                return res
            }

            return res.success(
                vec![Some(
                    Nodes::FuncDef(
                        FuncDefNode::new(
                            var_name_tok,
                            args_name_toks,
                            vec![body],
                            true,
                        )
                    )
                )]
            )
        }

        if self.current_tok.clone().unwrap().token_type != TokenType::TTNewLine {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected '->' or newline"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        body = res.register(self.statement())[0].clone().unwrap();
        if res.error.is_some() {
            return res
        }

        if !self.current_tok.clone().unwrap().matches(TokenType::TTKeyword, String::from("end")) {
            return res.failure(
                Error::new(
                    self.current_tok.clone().unwrap().pos_start,
                    self.current_tok.clone().unwrap().pos_end,
                    String::from("Expected 'end'"),
                    ErrorType::InvalidSyntaxError,
                )
            )
        }

        res.register_advancement();
        self.advance();

        return res.success(
            vec![Some(
                Nodes::FuncDef(
                    FuncDefNode::new(
                        var_name_tok,
                        args_name_toks,
                        vec![body],
                        false
                    )
                )
            )]
        )
    }
    
    pub fn bin_op(&mut self, parse_a: ParseResult, ops: Vec<(TokenType, String)>, parse_b: ParseResult) -> ParseResult {
        let mut res = ParseResult::new();
        let mut left = res.register(parse_a);
        if res.error.is_some() {
            return res
        }
        while ops.contains(&(self.current_tok.clone().unwrap().token_type, self.current_tok.clone().unwrap().value)) {
            let op_tok = self.current_tok.clone().unwrap();
            res.register_advancement();
            self.advance();
            let right = res.register(parse_b.clone())[0].clone();
            if res.error.is_some() {
                return res
            }
            left = vec![Some(
                Nodes::BinOp(
                    BinOpNode::new(
                        vec![left[0].clone().unwrap()],
                        op_tok,
                        vec![right.unwrap()],
                    )
                )
            )]
        }
        return res.success(left)
    }
}