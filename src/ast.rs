use std::fmt::Debug;


#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Id(String),
    Declare(String, Box<Expr>),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Failure(String),
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}


