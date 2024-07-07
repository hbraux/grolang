use std::str::FromStr;

use strum_macros::Display;

use crate::builtin::BuiltIn;
use crate::Env;
use crate::errors::ErrorCode;
use crate::expr::Expr::{Bool, Call, Error, Float, Int, Nil, Str, Symbol, TypeSpec};
use crate::parser::parse;
use crate::types::Type;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    TypeSpec(Type),
    Block(Vec<Expr>),
    Call(String, Vec<Expr>),
    Error(ErrorCode),
    Nil,
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NIL: Expr = Nil;

impl Expr {
    pub fn read(str: &str, _ctx: &Env) -> Expr {
        parse(str).unwrap_or_else(|s| Error(ErrorCode::ParseError(s)))
    }
    pub fn parse_type_spec(str: &str) -> Expr {
        TypeSpec(Type::new(str.replace(":", "").trim()))
    }

    // recursive format with debug
    pub fn format(&self) -> String { format!("{:?}", self).replace("\"","") }
    fn is_error(&self) -> bool { matches!(self, Error(_)) }
    pub fn get_type(&self) -> Type {
        match self {
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            Str(_) => Type::Str,
            _ => Type::Any,
        }
    }

    pub fn eval(self, ctx: &mut Env) -> Expr {
        match self {
            Nil | Error(_) | Int(_) | Float(_) | Str(_) | Bool(_) | TypeSpec(_) => self,
            Symbol(name) => ctx.get(&*name),
            Call(name, args) => if let Ok(op) = BuiltIn::from_str(&name) { op.apply(args, ctx) } else { panic!("{} is not a built-in function", name) }
            _ => panic!("{}.eval() not implemented", self)
        }
    }

    pub fn print(self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Nil => "nil".to_string(),
            Float(x) => {
                let str = x.to_string();
                if str.contains('.') { str } else { format!("{}.0", str) }
            }
            Symbol(x) => x,
            _ => format!("{:?}", self),
        }
    }

    fn ensure(self, spec: Option<Type>) -> Expr {
        if let Some(expected) = spec {
            if !self.is_error() && self.get_type() != expected {
                return Error(ErrorCode::InconsistentType(expected.to_string()));
            }
        }
        self
    }

    pub(crate) fn to_bool(self) -> Expr {
        match self {
            Bool(_) => self,
            _ => Error(ErrorCode::NotBoolean)
        }
    }
}



