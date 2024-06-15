use std::fmt::Debug;

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Identifier(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error(&'static str),
}

#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}


