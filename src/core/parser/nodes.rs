use crate::core::tokens::{
    token::Token,
    position::Position,
};
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Nodes {
    Number(NumberNode),
    BinOp(BinOpNode),
    UnaryOp(UnaryOpNode),
}
impl Nodes {
    pub fn as_string(&self) -> String {
        match self {
            Nodes::Number(number_node) => number_node.as_string(),
            Nodes::BinOp(bin_op_node) => bin_op_node.as_string(),
            Nodes::UnaryOp(unary_op_node) => unary_op_node.as_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
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
        return format!("{}", self.token.clone().to_string());
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BinOpNode {
    pub left_node: Box<Nodes>,
    pub op_token: Token,
    pub right_node: Box<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl BinOpNode {
    pub fn new(left_node: Nodes, op_token: Token, right_node: Nodes) -> BinOpNode {
        BinOpNode {
            left_node: Box::new(left_node),
            op_token: op_token.clone(),
            right_node: Box::new(right_node),
            pos_start: op_token.pos_start,
            pos_end: op_token.pos_end,
        }
    }
    pub fn as_string(&self) -> String {
        return format!(
            "[Left: {} Op: {} Right: {}]",
            self.left_node.as_string(),
            self.op_token.clone().to_string(),
            self.right_node.as_string()
        );
    } 
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UnaryOpNode {
    pub op_token: Token,
    pub node: Box<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl UnaryOpNode {
    pub fn new(op_token: Token, node: Nodes) -> UnaryOpNode {
        UnaryOpNode {
            op_token: op_token.clone(),
            node: Box::new(node),
            pos_start: op_token.pos_start,
            pos_end: op_token.pos_end,
        }
    }
    pub fn as_string(&self) -> String {
        return format!(
            "[Op: {} Node: {}]",
            self.op_token.clone().to_string(),
            self.node.as_string()
        );
    }
}