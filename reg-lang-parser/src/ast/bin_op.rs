#![allow(unstable_features)]
use crate::ast::expr::*;
#[derive(Debug, Clone)]

pub(crate) struct BinOp {
    left: Box<Expr>,
    op: Operator,
    right: Box<Expr>,
}
#[derive(Debug, Clone)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow
}

impl BinOp {
    pub fn new(left: Box<Expr>, op: Operator, right: Box<Expr>) -> BinOp {
        BinOp {
            left,
            op,
            right
        }
    }
    pub fn resolve(&mut self) -> Number {
        match self.op {
            Operator::Add => {
                let left = &*self.left;
                let right = &*self.right;
            }
            
            _ => todo!()
        }
        todo!()
    }
}