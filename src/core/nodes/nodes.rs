use crate::core::tokens::{
    token::Token,
    position::Position,
};
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Nodes {
    Number(NumberNode),
    String(StringNode),
    List(ListNode),
    VarAccess(VarAccessNode),
    VarAssign(VarAssignNode),
    BinOp(BinOpNode),
    UnaryOp(UnaryOpNode),
    If(IfNode),
    For(ForNode),
    While(WhileNode),
    FuncDef(FuncDefNode),
    Call(CallNode),
    Return(ReturnNode),
    Continue(ContinueNode),
    Break(BreakNode),
}
impl Nodes {
    pub fn get_pos_start(&self) -> Position {
        match self {
            Nodes::Number(node) => node.pos_start.clone(),
            Nodes::String(node) => node.pos_start.clone(),
            Nodes::List(node) => node.pos_start.clone(),
            Nodes::VarAccess(node) => node.pos_start.clone(),
            Nodes::VarAssign(node) => node.pos_start.clone(),
            Nodes::BinOp(node) => node.pos_start.clone(),
            Nodes::UnaryOp(node) => node.pos_start.clone(),
            Nodes::If(node) => node.pos_start.clone(),
            Nodes::For(node) => node.pos_start.clone(),
            Nodes::While(node) => node.pos_start.clone(),
            Nodes::FuncDef(node) => node.pos_start.clone(),
            Nodes::Call(node) => node.pos_start.clone(),
            Nodes::Return(node) => node.pos_start.clone(),
            Nodes::Continue(node) => node.pos_start.clone(),
            Nodes::Break(node) => node.pos_start.clone(),
        }
    }
    pub fn get_pos_end(&self) -> Position {
        match self {
            Nodes::Number(node) => node.pos_end.clone(),
            Nodes::String(node) => node.pos_end.clone(),
            Nodes::List(node) => node.pos_end.clone(),
            Nodes::VarAccess(node) => node.pos_end.clone(),
            Nodes::VarAssign(node) => node.pos_end.clone(),
            Nodes::BinOp(node) => node.pos_end.clone(),
            Nodes::UnaryOp(node) => node.pos_end.clone(),
            Nodes::If(node) => node.pos_end.clone(),
            Nodes::For(node) => node.pos_end.clone(),
            Nodes::While(node) => node.pos_end.clone(),
            Nodes::FuncDef(node) => node.pos_end.clone(),
            Nodes::Call(node) => node.pos_end.clone(),
            Nodes::Return(node) => node.pos_end.clone(),
            Nodes::Continue(node) => node.pos_end.clone(),
            Nodes::Break(node) => node.pos_end.clone(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NumberNode {
    pub token: Token,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode {
            token: token.clone(),
            pos_start: token.pos_start,
            pos_end: token.pos_end,
        }
    }
    pub fn as_string(&self) -> String {
        return format!("{}", self.token.clone().to_string())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StringNode {
    pub token: Token,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl StringNode {
    pub fn new(token: Token) -> StringNode {
        StringNode {
            token: token.clone(),
            pos_start: token.pos_start,
            pos_end: token.pos_end,
        }
    }
    pub fn as_string(&self) -> String {
        return format!("{}", self.token.clone().to_string())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ListNode {
    pub elements: Vec<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl ListNode {
    pub fn new(elements: Vec<Nodes>, pos_start: Position, pos_end: Position) -> ListNode {
        ListNode {
            elements,
            pos_start,
            pos_end,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VarAccessNode {
    pub var_name_token: Token,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl VarAccessNode {
    pub fn new(var_name_token: Token) -> VarAccessNode {
        VarAccessNode {
            var_name_token: var_name_token.clone(),
            pos_start: var_name_token.pos_start,
            pos_end: var_name_token.pos_end,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct VarAssignNode {
    pub var_name_token: Token,
    pub value_node: Token,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl VarAssignNode {
    pub fn new(var_name_token: Token, value_node: Token) -> VarAssignNode {
        VarAssignNode {
            var_name_token: var_name_token.clone(),
            value_node: value_node.clone(),
            pos_start: var_name_token.pos_start,
            pos_end: value_node.pos_end,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BinOpNode {
    pub left_node: Vec<Nodes>,
    pub op_token: Token,
    pub right_node: Vec<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl BinOpNode {
    pub fn new(left_node: Vec<Nodes>, op_token: Token, right_node: Vec<Nodes>) -> BinOpNode {
        BinOpNode {
            left_node: left_node.clone(),
            op_token: op_token.clone(),
            right_node: right_node.clone(),
            pos_start: left_node[0].get_pos_start(),
            pos_end: right_node[0].get_pos_end(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UnaryOpNode {
    pub op_token: Token,
    pub node: Vec<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl UnaryOpNode {
    pub fn new(op_token: Token, node: Vec<Nodes>) -> UnaryOpNode {
        UnaryOpNode {
            op_token: op_token.clone(),
            node: node.clone(),
            pos_start: op_token.pos_start,
            pos_end: node[0].get_pos_start(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IfNode {
    pub cases: Vec<(BinOpNode, Vec<Nodes>)>,
    pub should_return_null: bool,
    pub else_case: Option<Vec<Nodes>>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl IfNode {
    pub fn new(cases:Vec<(BinOpNode, Vec<Nodes>)>, else_case: Option<Vec<Nodes>>, should_return_null:bool) -> IfNode {
        IfNode{
            cases: cases.clone(),
            should_return_null,
            else_case: else_case.clone(),
            pos_start: cases[0].0.pos_start.clone(),
            pos_end: if else_case != None {
                let else_case_vec = else_case.unwrap();
                else_case_vec[else_case_vec.len() -1].get_pos_end()
            } else {
                cases[cases.len() -1].0.pos_end.clone()
            } ,
        }

    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ForNode {
    pub var_name_token: Token,
    pub start_value_node: Vec<Nodes>,
    pub end_value_node: Vec<Nodes>,
    pub step_value_node: Vec<Option<Nodes>>,
    pub body_node: Vec<Nodes>,
    pub should_return_null: bool,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl ForNode {
    pub fn new(var_name_token: Token, start_value_node: Vec<Nodes>, end_value_node: Vec<Nodes>, step_value_node: Vec<Option<Nodes>>, body_node: Vec<Nodes>, should_return_null: bool) -> ForNode {
        ForNode {
            var_name_token: var_name_token.clone(),
            start_value_node,
            end_value_node,
            step_value_node,
            body_node: body_node.clone(),
            should_return_null,
            pos_start: var_name_token.pos_start,
            pos_end: body_node[body_node.len() -1].get_pos_end(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WhileNode {
    pub condition_node: Vec<Nodes>,
    pub body_node: Vec<Nodes>,
    pub should_return_null: bool,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl WhileNode {
    pub fn new(condition_node: Vec<Nodes>, body_node: Vec<Nodes>, should_return_null: bool) -> WhileNode {
        WhileNode {
            condition_node: condition_node.clone(),
            body_node: body_node.clone(),
            should_return_null,
            pos_start: condition_node[0].get_pos_start(),
            pos_end: body_node[body_node.len() -1].get_pos_end(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncDefNode {
    pub var_name_token: Option<Token>,
    pub arg_name_tokens: Option<Vec<Token>>,
    pub body_node: Vec<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl FuncDefNode {
    pub fn new(var_name_token: Option<Token>, arg_name_tokens: Option<Vec<Token>>, body_node: Vec<Nodes>) -> FuncDefNode {
        FuncDefNode {
            var_name_token: var_name_token.clone(),
            arg_name_tokens: arg_name_tokens.clone(),
            body_node: body_node.clone(),
            pos_start: if var_name_token != None {
                var_name_token.unwrap().pos_start
            } else if arg_name_tokens.clone() != None {
                let salut = arg_name_tokens.unwrap()[0].clone();
                salut.pos_start
            } else {
                body_node[0].get_pos_start()
            },
            pos_end: body_node[body_node.len() -1].get_pos_end(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CallNode {
    pub node_to_call: Vec<Nodes>,
    pub arg_nodes: Vec<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl CallNode {
    pub fn new(node_to_call: Vec<Nodes>, arg_nodes: Vec<Nodes>) -> CallNode {
        CallNode {
            node_to_call: node_to_call.clone(),
            arg_nodes: arg_nodes.clone(),
            pos_start: node_to_call[0].get_pos_start(),
            pos_end: if arg_nodes.len() > 0 {
                arg_nodes[arg_nodes.len() -1].clone().get_pos_end()
            } else {
                node_to_call[0].clone().get_pos_end()
            },
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReturnNode {
    pub node_to_return: Vec<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl ReturnNode {
    pub fn new(node_to_return: Vec<Nodes>, pos_start: Position, pos_end:Position) -> ReturnNode {
        ReturnNode {
            node_to_return,
            pos_start,
            pos_end,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ContinueNode {
    pub pos_start: Position,
    pub pos_end: Position,
}
impl ContinueNode {
    pub fn new(pos_start: Position, pos_end:Position) -> ContinueNode {
        ContinueNode {
            pos_start,
            pos_end,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BreakNode {
    pub pos_start: Position,
    pub pos_end: Position,
}
impl BreakNode {
    pub fn new(pos_start: Position, pos_end:Position) -> BreakNode {
        BreakNode {
            pos_start,
            pos_end,
        }
    }
}