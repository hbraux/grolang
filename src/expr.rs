use std::str::FromStr;

use strum_macros::Display;

use crate::builtin::BuiltIn;
use crate::exception::Exception;
use crate::Scope;
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
    Error(Exception),
    Nil,
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NIL: Expr = Nil;

impl Expr {
    pub fn read(str: &str, _ctx: &Scope) -> Expr {
        parse(str).unwrap_or_else(|s| Error(Exception::CannotParse(s)))
    }
    pub fn parse_type_spec(str: &str) -> Expr {
        TypeSpec(Type::new(str.replace(":", "").trim()))
    }
    // recursive format with debug
    pub fn format(&self) -> String { format!("{:?}", self).replace("\"","") }

    pub fn get_type(&self) -> Type {
        match self {
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            Str(_) => Type::Str,
            _ => Type::Any,
        }
    }
    // eval takes ownership!
    pub fn eval(self, scope: &mut Scope) -> Result<Expr, Exception> {
        match self {
            Error(e) => Err(e),
            Nil | Int(_) | Float(_) | Str(_) | Bool(_) | TypeSpec(_) => Ok(self),
            Symbol(name) => scope.get(&*name),
            Call(name, args) => if let Ok(op) = BuiltIn::from_str(&name) { op.apply(args, scope) } else { panic!("{} is not a built-in function", name) }
            _ => Err(Exception::NotImplemented(format!("{}.eval", self)))
        }
    }
    pub fn eval_or_error(self, scope: &mut Scope) -> Expr {
        match self {
            Error(_) => self,
            expr => expr.eval(scope).unwrap_or_else(|ex| Error(ex))
        }
    }

    pub fn print(&self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Nil => "nil".to_string(),
            Float(x) => {
                let str = x.to_string();
                if str.contains('.') { str } else { format!("{}.0", str) }
            }
            Symbol(x) => x.to_string(),
            _ => format!("{:?}", self),
        }
    }
    pub fn to_bool(self) -> Result<Expr, Exception> {
        match self {
            Bool(_) => Ok(self),
            _ => Err(Exception::NotBoolean(self.format()))
        }
    }

    fn failed(&self) -> bool { matches!(self, Error(_)) }
    fn ensure(self, spec: Option<Type>) -> Expr {
        if let Some(expected) = spec {
            if !self.failed() && self.get_type() != expected {
                return Error(Exception::InconsistentType(expected.to_string()));
            }
        }
        self
    }

}



