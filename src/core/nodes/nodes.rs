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
#[derive(Clone, Debug, PartialEq,)]
pub(crate) enum ExprNodes {
    Number(NumberNode),
    String(StringNode),
    List(ListNode),
    VarAccess(VarAccessNode),

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
    pub left_node: ExprNodes,
    pub op_token: Token,
    pub right_node: ExprNodes,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl BinOpNode {
    pub fn new(left_node: ExprNodes, op_token: Token, right_node: ExprNodes) -> BinOpNode {
        BinOpNode {
            left_node: left_node.clone(),
            op_token: op_token.clone(),
            right_node: right_node.clone(),
            pos_start: Self::get_left_pos(left_node),
            pos_end: Self::get_right_pos(right_node),
        }
    }
    pub fn get_left_pos(left:ExprNodes) -> Position {
        match left {
            ExprNodes::Number(left_node) => {
                return left_node.pos_start;
            },
            ExprNodes::String(left_node) => {
                return left_node.pos_start;
            },
            ExprNodes::List(left_node) => {
                return left_node.pos_start;
            },
            ExprNodes::VarAccess(left_node) => {
                return left_node.pos_start;
            },
        }
    }
    pub fn get_right_pos(right:ExprNodes) -> Position {
            match right {
                ExprNodes::Number(right_node) => {
                    return right_node.pos_start;
                },
                ExprNodes::String(right_node) => {
                    return right_node.pos_start;
                },
                ExprNodes::List(right_node) => {
                    return right_node.pos_start;
                },
                ExprNodes::VarAccess(right_node) => {
                    return right_node.pos_start;
                },
            }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UnaryOpNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IfNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ForNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WhileNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncDefNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CallNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ReturnNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ContinueNode {

}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BreakNode {

}