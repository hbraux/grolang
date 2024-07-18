use std::fmt::Debug;

use strum_macros::Display;

use crate::exception::Exception;
use crate::expr::Expr::Parameters;
use crate::functions::{Function, LazyFunction};
use crate::parser::parse;
use crate::Scope;
use crate::types::Type;

use self::Expr::{Bool, Call, Failure, Float, Int, Nil, Str, Symbol, TypeOf};


#[derive(Debug, Clone, PartialEq, Display)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    TypeOf(Type),
    Parameters(Vec<(String, Type)>),
    Call(String, Vec<Expr>),
    Failure(Exception),
    Fun(String, Type, Function),
    LazyFun(String, LazyFunction),
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
    pub fn to_parameters(&self) -> Result<&Vec<(String,Type)>, Exception> {
        match self {
            Parameters(x) => Ok(x),
            _ => Err(Exception::NotA("Parameter".to_owned(), self.print()))
        }
    }
    pub fn eval(&self, scope: &mut Scope) -> Result<Expr, Exception> {
        match self {
            Failure(e) => Err(e.clone()),
            Nil | Int(_) | Float(_) | Str(_) | Bool(_) | TypeOf(_) => Ok(self.clone()),
            Symbol(name) => handle_symbol(name, scope),
            Call(name, args) => scope.try_lazy(name, args).unwrap_or(handle_call(name, args, scope)),
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

fn handle_call(name: &str, args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    args.iter().map(|e| e.eval(scope)).collect::<Result<Vec<Expr>, Exception>>().and_then(|values|
    match scope.get_fun(name, values.get(0).map(|e| e.get_type())) {
        Some((types, lambda)) => apply_lambda(name, types, &values, lambda),
        _ => Err(Exception::UndefinedFunction(name.to_string())),
    })
}

fn apply_lambda(name: &str, specs: &Type, values: &Vec<Expr>, lambda: &Function) ->  Result<Expr, Exception> {
    if let Type::Fun(input, output) = specs {
        if input.len() != values.len() {
            Err(Exception::WrongArgumentsNumber(name.to_owned(), input.len(), values.len()))
       // TODO
       // } else if input.len() != input.iter().zip(&values).filter(|&(a, b)| a == b.get_type()).count() {
       //     Err(Exception::UnexpectedInputTypes(name.to_owned(), "todo".to_owned()))
        } else {
            let result = lambda.apply(values);
            if result.is_ok() && result.as_ref().unwrap().get_type() != **output {
                Err(Exception::UnexpectedOutputType(name.to_owned(), "".to_owned()))
            } else {
                result
            }
        }
    } else {
        panic!("")
    }
}
