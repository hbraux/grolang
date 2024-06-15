use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Identifier(String),
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


