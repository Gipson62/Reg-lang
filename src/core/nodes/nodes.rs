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
#[derive(Clone, Debug, PartialEq,)]
pub(crate) enum ExprNodes {
    Number(NumberNode),
    String(StringNode),
    List(ListNode),
    VarAccess(VarAccessNode),
    BinOp(BinOpNode),
    UnaryOp(UnaryOpNode),
}
impl ExprNodes {
    fn get_pos_start(&self) -> Position {
        match self {
            ExprNodes::Number(node) => node.pos_start.clone(),
            ExprNodes::String(node) => node.pos_start.clone(),
            ExprNodes::List(node) => node.pos_start.clone(),
            ExprNodes::VarAccess(node) => node.pos_start.clone(),
            ExprNodes::BinOp(node) => node.pos_start.clone(),
            ExprNodes::UnaryOp(node) => node.pos_start.clone(),
        }
    }
    fn get_pos_end(&self) -> Position {
        match self {
            ExprNodes::Number(node) => node.pos_end.clone(),
            ExprNodes::String(node) => node.pos_end.clone(),
            ExprNodes::List(node) => node.pos_end.clone(),
            ExprNodes::VarAccess(node) => node.pos_end.clone(),
            ExprNodes::BinOp(node) => node.pos_end.clone(),
            ExprNodes::UnaryOp(node) => node.pos_end.clone(),
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
    pub left_node: Vec<ExprNodes>,
    pub op_token: Token,
    pub right_node: Vec<ExprNodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl BinOpNode {
    pub fn new(left_node: ExprNodes, op_token: Token, right_node: ExprNodes) -> BinOpNode {
        BinOpNode {
            left_node: [left_node.clone()].to_vec(),
            op_token: op_token.clone(),
            right_node: [right_node.clone()].to_vec(),
            pos_start: left_node.get_pos_start(),
            pos_end: right_node.get_pos_end(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UnaryOpNode {
    pub op_token: Token,
    pub node: Vec<ExprNodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl UnaryOpNode {
    pub fn new(op_token: Token, node: ExprNodes) -> UnaryOpNode {
        UnaryOpNode {
            op_token: op_token.clone(),
            node: [node.clone()].to_vec(),
            pos_start: op_token.pos_start,
            pos_end: node.get_pos_start(),
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
    pub pos_start: Position,
    pub pos_end: Position,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WhileNode {
    pub pos_start: Position,
    pub pos_end: Position,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncDefNode {
    pub pos_start: Position,
    pub pos_end: Position,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CallNode {
    pub pos_start: Position,
    pub pos_end: Position,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReturnNode {
    pub pos_start: Position,
    pub pos_end: Position,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ContinueNode {
    pub pos_start: Position,
    pub pos_end: Position,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BreakNode {
    pub pos_start: Position,
    pub pos_end: Position,
}