#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use ErrorType::{DivisionByZero, NotANumber, UndefinedSymbol, CannotParse};
use Expr::{Bool, Declare, Error, Float, Id, Int, Null, BinaryOp, Str};


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Any,
    Int,
    Bool,
    Custom(String),
    List(Box<Type>),
    Map(Box<Type>,Box<Type>)
}

impl Type {
    pub fn new(str: &str) -> Type {
        if str.starts_with("List<") {
            Type::List(Box::new(Type::new(&str[5..str.len()-1])))
        } else if str.starts_with("Map<") {
            let s : Vec<&str> = (&str[4..str.len()-1]).split(',').collect();
            Type::Map(Box::new(Type::new(s[0])), Box::new(Type::new(s[1])))
        } else {
            match str {
                "Any" => Type::Any,
                "Int" => Type::Int,
                "Bool" => Type::Bool,
                _ => if str.chars().all(|x| x.is_alphabetic()) {
                    Type::Custom(str.to_string())
                } else {
                    panic!("{} is not a valid type name", str)
                }
            }
        }
    }
}


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
    SimpleType(String),
    ComplexType(String, Vec<String>),
    Declare(String, Box<Expr>),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    Call(Box<Expr>, String, Vec<Box<Expr>>),
    Error(ErrorType),
    Null
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NULL: Expr = Null;

#[derive(Debug, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}


impl Expr {
    //noinspection ALL
    pub fn new(str: &str) -> Expr {
        match grammar::StatementParser::new().parse(str)  {
            Ok(expr) => *expr,
            Err(e) =>  Error(CannotParse(e.to_string())),
        }
    }

    pub fn eval(self, ctx: &mut Context) -> Expr {
        match self {
            Id(s) => ctx.get(&*s).clone(),
            Declare(s, right) => { let value = right.eval(ctx); ctx.set(s.as_str(), value) },
            BinaryOp(left, code, right) => eval_op(code, left.eval(ctx), right.eval(ctx)),
            _ => self.clone()
        }
    }

    pub fn print(self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Float(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Null => "null".to_string(),
            _ => format!("{:?}", self)
        }
    }
}


pub struct Context {
    values: HashMap<String, Expr>,
}

impl Context {
    pub fn new() -> Context { Context { values: HashMap::new() } }

    pub fn get(&self, name: &str) -> Expr {
        match self.values.get(name)  {
            Some(expr) => expr.clone(),
            None => Error(UndefinedSymbol(name.to_string())),
        }
    }
    pub fn set(&mut self, name: &str, expr: Expr) -> Expr {
        self.values.insert(name.to_string(), expr);
        NULL
    }

}

fn eval_op(code: Opcode, left: Expr, right: Expr) -> Expr {
    if let (Int(a), Int(b)) = (left, right) {
        match code {
            Opcode::Add => Int(a + b),
            Opcode::Sub => Int(a - b),
            Opcode::Mul => Int(a * b),
            Opcode::Div => if b !=0 { Int(a / b) } else { Error(DivisionByZero) },
        }
    } else {
        Error(NotANumber)
    }
}

#[test]
fn test() {
    assert_eq!(Type::Any, Type::new("Any"));
    assert_eq!(Type::Int, Type::new("Int"));
    assert_eq!(Type::List(Box::new(Type::Int)), Type::new("List<Int>"));
    assert_eq!(Type::Map(Box::new(Type::Int),Box::new(Type::Bool)), Type::new("Map<Int,Bool>"));
}

