use std::fmt::Debug;
use strum_macros::Display;

use crate::exception::Exception;
use crate::functions::{Function};
use crate::parser::parse;
use crate::scope::Scope;
use crate::types::Type;
use self::Expr::{Bool, Call, Failure, Float, Int, Nil, Str, Symbol, TypeOf, Params};


#[derive(Debug, Clone, PartialEq, Display)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    TypeOf(Type),
    Params(Vec<(String, Type)>),  // used by the parser to simplify function parsing
    Call(String, Vec<Expr>),
    Failure(Exception),
    Fun(String, Type, Function),
    Nil,
}


pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NIL: Expr = Nil;

impl Expr {
    pub fn read(str: &str, _ctx: &Scope) -> Expr {
        parse(str).unwrap_or_else(|s| Failure(Exception::CannotParse(s)))
    }
    pub fn read_type(str: &str) -> Expr {
        TypeOf(Type::new(str.replace(":", "").trim()))
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

    pub fn int(&self) -> Result<&i64, Exception> {
        match self {
            Int(x) => Ok(x),
            _ => Err(Exception::NotA(Type::Int.to_string(), self.print()))
        }
    }
    pub fn float(&self) -> Result<&f64, Exception> {
        match self {
            Float(x) => Ok(x),
            _ => Err(Exception::NotA(Type::Float.to_string(), self.print()))
        }
    }
    pub fn bool(&self) -> Result<bool, Exception> {
        match self {
            Bool(x) => Ok(x.to_owned()),
            _ => Err(Exception::NotA(Type::Bool.to_string(), self.print()))
        }
    }
    pub fn symbol(&self) -> Result<&str, Exception> {
        match self {
            Symbol(x) => Ok(x),
            _ => Err(Exception::UndefinedSymbol(self.print()))
        }
    }
    pub fn to_type(&self) -> Result<&Type, Exception> {
        match self {
            TypeOf(x) => Ok(x),
            Nil => Ok(&Type::Any),
            _ => Err(Exception::NotA("Type".to_owned(), self.print()))
        }
    }
    pub fn to_params(&self) -> Result<&Vec<(String, Type)>, Exception> {
        match self {
            Params(p) => Ok(p),
            _ => Err(Exception::NotA("Parameters".to_owned(), self.print()))
        }
    }
    pub fn eval(&self, scope: &mut Scope) -> Result<Expr, Exception> {
        match self {
            Failure(e) => Err(e.clone()),
            Nil | Int(_) | Float(_) | Str(_) | Bool(_) | TypeOf(_) => Ok(self.clone()),
            Symbol(name) => handle_symbol(name, scope),
            Call(name, args) => scope.try_lazy(name, args).unwrap_or(eval_call(name, args, scope)),
            _ => Err(Exception::NotImplemented(format!("{}", self))),
        }
    }
    pub fn eval_or_failed(&self, scope: &mut Scope) -> Expr {
        match self {
            Failure(_) => self.clone(),
            expr => expr.eval(scope).unwrap_or_else(|ex| Failure(ex))
        }
    }

    pub fn print(&self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Nil => "nil".to_owned(),
            Float(x) => format_float(x),
            Symbol(x) => x.to_owned(),
            Failure(x) => x.format(),
            _ => format!("{:?}", self),
        }
    }

}

fn format_float(x: &f64) -> String  {
    let str = x.to_string();
    if str.contains('.') {
        str
    } else {
        format!("{}.0", str)
    }
}

fn handle_symbol(name: &str, scope: &Scope) -> Result<Expr, Exception> {
    scope.get(name).ok_or_else(|| Exception::UndefinedSymbol(name.to_string()))
}

fn eval_call(name: &str, args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    args.iter().map(|e| e.eval(scope)).collect::<Result<Vec<Expr>, Exception>>().and_then(|values| {
        match scope.get_fun(name, values.get(0).map(|e| e.get_type())) {
            Some((types, fun)) => apply_fun(name, types, &values, fun, &mut scope.extend()),
            _ => Err(Exception::UndefinedFunction(name.to_string())),
    }})
}

fn apply_fun(name: &str, specs: &Type, values: &Vec<Expr>, lambda: &Function, scope: &mut Scope) ->  Result<Expr, Exception> {
    if let Type::Fun(input, _output) = specs {
        match check_arguments(name, input, values) {
            Some(ex) => Err(ex),
            _ => lambda.apply(values, scope)
        }
    } else {
        Err(Exception::NotA("Fun".to_owned(), specs.to_string()))
    }
}

fn check_arguments(name: &str, expected: &Vec<Type>, values: &Vec<Expr>) -> Option<Exception> {
    if expected.len() != values.len() {
        return Some(Exception::WrongArgumentsNumber(name.to_owned(), expected.len(), values.len()))
    }
    expected.iter().zip(values.iter()).find(|(e, v)| **e != v.get_type()).and_then(|p|
        Some(Exception::UnexpectedArgumentType(name.to_owned(), p.0.to_string()))
    )
}
