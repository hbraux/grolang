#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use strum_macros::Display;
use ErrorType::{DivisionByZero, NotANumber, UndefinedSymbol, CannotParse};
use Expr::{Bool, Declare, Error, Float, Id, Int, Null, BinaryOp, Str};
use crate::Expr::{TypeSpec, TypeExpr};


#[derive(Debug, Eq, PartialEq, Clone, Display)]
pub enum ClassType {
    Any,
    Int,
    Bool,
    Str,
    Float,
    Error,
    Custom(String),
    List(Box<ClassType>),
    Option(Box<ClassType>),
    Fail(Box<ClassType>),
    Map(Box<ClassType>, Box<ClassType>)
}

impl ClassType {
    pub fn new(str: &str) -> ClassType {
        if str.ends_with("?") {
            ClassType::Option(Box::new(ClassType::new(&str[0..str.len()-1])))
        } else if str.ends_with("!") {
            ClassType::Fail(Box::new(ClassType::new(&str[0..str.len()-1])))
        } else if str.starts_with("List<") {
            ClassType::List(Box::new(ClassType::new(&str[5..str.len() - 1])))
        }else if str.starts_with("List<") {
            ClassType::List(Box::new(ClassType::new(&str[5..str.len()-1])))
        } else if str.starts_with("Map<") {
            let s : Vec<&str> = (&str[4..str.len()-1]).split(',').collect();
            ClassType::Map(Box::new(ClassType::new(s[0])), Box::new(ClassType::new(s[1])))
        } else {
            match str {
                "Int" => ClassType::Int,
                "Bool" => ClassType::Bool,
                "Str" => ClassType::Str,
                "Float" => ClassType::Float,
                _ => if str.chars().all(|x| x.is_alphabetic()) {
                    ClassType::Custom(str.to_string())
                } else {
                    ClassType::Error
                }
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    DivisionByZero,
    UndefinedSymbol(String),
    CannotParse(String),
    NotANumber,
    InconsistentType(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Id(String),
    TypeSpec(String),
    TypeExpr(ClassType),
    Declare(String, Option<String>, Box<Expr>),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    Call(Box<Expr>, String, Vec<Box<Expr>>),
    Error(ErrorType),
    Null
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NULL: Expr = Null;

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Mod,
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
    In
}


impl Expr {
    //noinspection ALL
    pub fn new(str: &str) -> Expr {
        match grammar::StatementParser::new().parse(str)  {
            Ok(expr) => *expr,
            Err(e) =>  Error(CannotParse(e.to_string())),
        }
    }

    pub fn get_type(&self) -> ClassType {
        match self {
            Bool(_) => ClassType::Bool,
            Int(_) => ClassType::Int,
            Float(_) => ClassType::Int,
            Str(x) => ClassType::Str,
            _ => ClassType::Any
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Error(_))
    }


    pub fn eval(self, ctx: &mut Context) -> Expr {
        match self {
            Id(s) => ctx.get(&*s).clone(),
            TypeSpec(s) => TypeExpr(ClassType::new(s.as_str())),
            Declare(s, spec, right) => self.declare(s, spec, right.eval(ctx));
            BinaryOp(left, code, right) => eval_op(code, left.eval(ctx), right.eval(ctx)),
            _ => self.clone()
        }
    }

    fn declare(&self, name: String, spec: String, value: Expr) -> Expr {
        if !value.is_error() {
            ctx.set(name.as_str(), value.clone());
            if let Some(expected) = spec {
                if value.get_type() != ClassType::new(s.as_str()) {
                    return Error(ErrorType::InconsistentType(value.get_type().to_string()))
                }
            }
        }
        value
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
    pub fn set(&mut self, name: &str, expr: Expr) {
        self.values.insert(name.to_string(), expr);
    }

}

fn eval_op(code: Opcode, left: Expr, right: Expr) -> Expr {
    if let (Int(a), Int(b)) = (left, right) {
        match code {
            Opcode::Add => Int(a + b),
            Opcode::Sub => Int(a - b),
            Opcode::Mul => Int(a * b),
            Opcode::Div => if b !=0 { Int(a / b) } else { Error(DivisionByZero) },
            _ => panic!()
        }
    } else {
        Error(NotANumber)
    }
}

#[test]
fn test() {
    assert_eq!(ClassType::Any, ClassType::new("Any"));
    assert_eq!(ClassType::Int, ClassType::new("Int"));
    assert_eq!(ClassType::List(Box::new(ClassType::Int)), ClassType::new("List<Int>"));
    assert_eq!(ClassType::Map(Box::new(ClassType::Int), Box::new(ClassType::Bool)), ClassType::new("Map<Int,Bool>"));
    assert_eq!(ClassType::Option(Box::new(ClassType::Int)), ClassType::new("Int?"));
    assert_eq!(ClassType::Fail(Box::new(ClassType::Int)), ClassType::new("Int!"));
}

