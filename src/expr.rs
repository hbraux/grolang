use std::fmt::Debug;

use strum_macros::Display;

use crate::exception::Exception;
use crate::expr::Expr::Mac;
use crate::functions::{Function, Macro};
use crate::parser::parse;
use crate::scope::Scope;
use crate::types::Type;

use self::Expr::{Bool, Call, Failure, Float, Int, Nil, Params, Str, Symbol, TypeOf};

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    TypeOf(Type),
    Params(Vec<(String, Type)>),
    Call(String, Vec<Expr>),
    Failure(Exception),
    Fun(String, Type, Function),
    Mac(String, Macro),
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
    pub fn eval(&self, scope: &Scope) -> Result<Expr, Exception> {
        match self {
            Failure(e) => Err(e.clone()),
            Nil | Int(_) | Float(_) | Str(_) | Bool(_) | TypeOf(_) => Ok(self.clone()),
            Symbol(name) => handle_symbol(name, scope),
            Call(name, args) => handle_call(name, args, scope),
            _ => Err(Exception::NotImplemented(format!("{}", self))),
        }
    }
    pub fn mut_eval(&self, scope: &mut Scope) -> Result<Expr, Exception> {
        match self {
            Call(name, args) if scope.is_macro(name) => handle_macro(scope, name, args),
            _ => self.eval(scope)
        }
    }

    pub fn eval_or_failed(&self, scope: &mut Scope) -> Expr {
        match self {
            Failure(_) => self.clone(),
            expr => expr.mut_eval(scope).unwrap_or_else(|ex| Failure(ex))
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

fn handle_macro(scope: &mut Scope, name: &String, args: &Vec<Expr>) -> Result<Expr, Exception> {
    match scope.get_global(name) {
        Some(Mac(_, lambda)) => lambda.clone().apply(args, scope),
        _ => Err(Exception::NotDefined(name.to_string())),
    }
}

fn handle_symbol(name: &str, scope: &Scope) -> Result<Expr, Exception> {
    scope.get_value(name).ok_or_else(|| Exception::UndefinedSymbol(name.to_string()))
}

fn handle_call(name: &str, args: &Vec<Expr>, scope: &Scope) -> Result<Expr, Exception> {
    let first = args.get(0).map(|e| e.eval(scope)).and_then();
    match scope.get_fun(name, first.map(|e| e.get_type())) {
        Some((full_name, types, fun)) => apply_fun(full_name, types, args, fun, &mut scope.local()),
        _ => Err(Exception::UndefinedFunction(name.to_string())),
    }
}

fn apply_fun(name: &str, specs: &Type, args: &Vec<Expr>, fun: &Function, scope: &mut Scope) ->  Result<Expr, Exception> {
    match specs {
        Type::LazyFun => fun.apply(args, scope),
        Type::Fun(input, _output) => args.iter().map(|e| e.eval(scope)).collect::<Result<Vec<Expr>, Exception>>().and_then(|values| {
            check_arguments(name, input, &values).or(Some(fun.apply(&values, scope))).unwrap()
        }),
        _ => Err(Exception::NotA("Fun".to_owned(), specs.to_string())),
    }
}

fn check_arguments(name: &str, expected: &Vec<Type>, values: &Vec<Expr>) -> Option<Result<Expr, Exception>> {
    if expected.len() != values.len() {
        return Some(Err(Exception::WrongArgumentsNumber(name.to_owned(), expected.len(), values.len())))
    }
    expected.iter().zip(values.iter()).find(|(e, v)| **e != v.get_type()).and_then(|p|
        Some(Err(Exception::UnexpectedArgumentType(name.to_owned(), p.1.to_string())))
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!("Nil", Nil.format())
    }
}
