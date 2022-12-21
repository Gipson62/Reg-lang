use crate::ast::{
    bin_op::*,
    float::*,
    integer::*,
};
#[derive(Debug, Clone)]
pub(crate) enum Expr {
    BinOp(BinOp),
    Float(Float),
    Integer(Integer),
    Literal(String),
}
impl Expr {
    pub fn resolve(&mut self) -> Expr {
        match self {
            Expr::BinOp(bin_op) => {
                let resolved = bin_op.resolve();
                match resolved {
                    Number::Float(float) => return Expr::Float(float),
                    Number::Integer(integer) => return Expr::Integer(integer),
                }
            },
            _ => todo!()
        }
    }
}

pub(crate) enum Number {
    Float(Float),
    Integer(Integer),
}