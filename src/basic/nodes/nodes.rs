use crate::basic::{
    token::Token,
    position::Position
};

pub(crate) struct NumberNode {
    pub tok:Token,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl NumberNode {
    pub fn new(tok:Token, pos_start:Position, pos_end:Position) -> NumberNode {
        NumberNode {
            tok:tok.clone(),
            pos_start:tok.pos_start.clone(),
            pos_end:tok.pos_end.clone(),
        }
    }
}

pub(crate) struct StringNode {
    pub tok:Token,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl StringNode {
    pub fn new(tok:Token, pos_start:Position, pos_end:Position) -> StringNode {
        StringNode {
            tok:tok.clone(),
            pos_start:tok.pos_start.clone(),
            pos_end:tok.pos_end.clone(),
        }
    }
}

pub(crate) struct ListNode {
    pub elements_nodes:Vec<Token>,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl ListNode {
    pub fn new(elements_nodes:Vec<Token>, pos_start:Position, pos_end:Position) -> ListNode {
        ListNode {
            elements_nodes,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct VarAccessNode {
    pub var_name_token:Token,
    pub value_node:String,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl VarAccessNode {
    pub fn new(var_name_token:Token, value_node:String, pos_start:Position, pos_end:Position) -> VarAccessNode {
        VarAccessNode {
            var_name_token,
            value_node,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct VarAssignNode {
    pub var_name_token:Token,
    pub value_node:String,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl VarAssignNode {
    pub fn new(var_name_token:Token, value_node:String, pos_start:Position, pos_end:Position) -> VarAssignNode {
        VarAssignNode {
            var_name_token,
            value_node,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct BinOpNode {
    pub left_node:String,
    pub op_token:Token,
    pub right_node:String,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl BinOpNode {
    pub fn new(left_node:String, op_token:Token, right_node:String, pos_start:Position, pos_end:Position) -> BinOpNode {
        BinOpNode {
            left_node,
            op_token,
            right_node,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct UnaryOpNode {
    pub op_token:Token,
    pub node:String,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl UnaryOpNode {
    pub fn new(op_token:Token, node:String, pos_start:Position, pos_end:Position) -> UnaryOpNode {
        UnaryOpNode {
            op_token,
            node,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct IfNode {
    pub cases:Vec<Token>,
    pub else_case:Option<Box<Token>>,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl IfNode {
    pub fn new(cases:Vec<Token>, else_case:Option<Box<Token>>, pos_start:Position, pos_end:Position) -> IfNode {
        IfNode {
            cases,
            else_case,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct ForNode {
    pub var_name_token:Token,
    pub start_value_node:String,
    pub end_value_node:String,
    pub step_value_node:String,
    pub body_node:String,
    pub should_return_null:bool,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl ForNode {
    pub fn new(var_name_token:Token, start_value_node:String, end_value_node:String, step_value_node:String, body_node:String, should_return_null:bool, pos_start:Position, pos_end:Position) -> ForNode {
        ForNode {
            var_name_token,
            start_value_node,
            end_value_node,
            step_value_node,
            body_node,
            should_return_null,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct WhileNode {
    pub condition_node:String,
    pub body_node:String,
    pub should_return_null:bool,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl WhileNode {
    pub fn new(condition_node:String, body_node:String, should_return_null:bool, pos_start:Position, pos_end:Position) -> WhileNode {
        WhileNode {
            condition_node,
            body_node,
            should_return_null,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct FuncDefNode {
    pub var_name_token:Token,
    pub arg_name_tokens:Vec<Token>,
    pub body_node:String,
    pub should_auto_return:bool,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl FuncDefNode {
    pub fn new(var_name_token:Token, arg_name_tokens:Vec<Token>, body_node:String, should_auto_return:bool, pos_start:Position, pos_end:Position) -> FuncDefNode {
        FuncDefNode {
            var_name_token,
            arg_name_tokens,
            body_node,
            should_auto_return,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct CallNode {
    pub node_to_call:String,
    pub arg_nodes:Vec<Token>,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl CallNode {
    pub fn new(node_to_call:String, arg_nodes:Vec<Token>, pos_start:Position, pos_end:Position) -> CallNode {
        CallNode {
            node_to_call,
            arg_nodes,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct ReturnNode {
    pub node_to_return:String,
    pub pos_start:Position,
    pub pos_end:Position,
}
impl ReturnNode {
    pub fn new(node_to_return:String, pos_start:Position, pos_end:Position) -> ReturnNode {
        ReturnNode {
            node_to_return,
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct ContinueNode {
    pub pos_start:Position,
    pub pos_end:Position,
}
impl ContinueNode {
    pub fn new(pos_start:Position, pos_end:Position) -> ContinueNode {
        ContinueNode {
            pos_start,
            pos_end,
        }
    }
}

pub(crate) struct BreakNode {
    pub pos_start:Position,
    pub pos_end:Position,
}
impl BreakNode {
    pub fn new(pos_start:Position, pos_end:Position) -> BreakNode {
        BreakNode {
            pos_start,
            pos_end,
        }
    }
}

