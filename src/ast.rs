use std::fmt::Debug;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}


