use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Id(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}


