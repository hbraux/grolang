use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum ErrorType {
    DivisionByZero,
    UndefinedSymbol(String),
    CannotParse(String),
    NotANumber
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Id(String),
    Declare(String, Box<Expr>),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    Error(ErrorType),
    Null
}

pub const TRUE: Expr = Expr::Bool(true);
pub const FALSE: Expr = Expr::Bool(false);
pub const NULL: Expr = Expr::Null;

#[derive(Debug, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

