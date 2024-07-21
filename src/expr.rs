use std::fmt::{Debug, Display, Formatter};

use crate::exception::Exception;
use crate::expr::Expr::Map;
use crate::functions::Function;
use crate::functions::Function::BuiltIn;
use crate::parser::parse;
use crate::scope::Scope;
use crate::types::Type;

use self::Expr::{Block, Bool, Call, Failure, Float, Fun, Int, List, Null, Params, Str, Symbol, TypeOf};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    TypeOf(Type),
    Params(Vec<(String, Type)>),
    Block(Vec<Expr>),
    List(Vec<Expr>),
    Map(Vec<(Expr,Expr)>),
    Call(String, Vec<Expr>),
    Failure(Exception),
    Fun(String, Type, Function),
    Null,
}


pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NULL: Expr = Null;

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Null => "null".to_owned(),
            Float(x) => format_float(x),
            Symbol(x) => x.to_owned(),
            Failure(x) => x.format(),
            TypeOf(x) => x.to_string(),
            Params(v) => format_vec(&v.iter().map(|p| format!("{}:{}", p.0, p.1)).collect::<Vec<_>>(),",", "(",")"),
            Map(v) => format_vec(&v.iter().map(|p| format!("{}:{}", p.0, p.1)).collect::<Vec<_>>(),",", "{","}"),
            List(vec) => format_vec(vec, ",", "[", "]"),
            Block(vec) => format_vec(vec, ";", "{", "}"),
            Call(name, vec) => format_vec(vec, ",", &(name.to_string() + "("), ")"),
            _ => format!("{:?}", self),
        };
        write!(f, "{}", str)
    }
}


impl Expr {
    pub fn read(str: &str, _ctx: &Scope) -> Expr {
        parse(str).unwrap_or_else(|s| Failure(Exception::CannotParse(s)))
    }
    pub fn read_type(str: &str) -> Expr {
        TypeOf(Type::new(str.replace(":", "").trim()))
    }

    pub fn get_type(&self) -> Type {
        match self {
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            Str(_) => Type::Str,
            _ => Type::Any,
        }
    }
    pub fn to_str(&self) -> Result<&str, Exception> {
        match self {
            Str(x) => Ok(x),
            _ => Err(Exception::NotA(Type::Str.to_string(), self.print()))
        }
    }
    pub fn to_int(&self) -> Result<&i64, Exception> {
        match self {
            Int(x) => Ok(x),
            _ => Err(Exception::NotA(Type::Int.to_string(), self.print()))
        }
    }
    pub fn to_float(&self) -> Result<&f64, Exception> {
        match self {
            Float(x) => Ok(x),
            _ => Err(Exception::NotA(Type::Float.to_string(), self.print()))
        }
    }
    pub fn to_bool(&self) -> Result<bool, Exception> {
        match self {
            Bool(x) => Ok(x.to_owned()),
            _ => Err(Exception::NotA(Type::Bool.to_string(), self.print()))
        }
    }
    pub fn to_symbol(&self) -> Result<&str, Exception> {
        match self {
            Symbol(x) => Ok(x),
            _ => Err(Exception::UndefinedSymbol(self.print()))
        }
    }
    pub fn to_type(&self) -> Result<&Type, Exception> {
        match self {
            TypeOf(x) => Ok(x),
            Null => Ok(&Type::Any),
            _ => Err(Exception::NotA("Type".to_owned(), self.print()))
        }
    }
    pub fn to_params(&self) -> Result<&Vec<(String, Type)>, Exception> {
        match self {
            Params(v) => Ok(v),
            _ => Err(Exception::NotA("Params".to_owned(), self.print()))
        }
    }
    pub fn eval(&self, scope: &Scope) -> Result<Expr, Exception> {
        match self {
            Failure(e) => Err(e.clone()),
            Null | Int(_) | Float(_) | Str(_) | Bool(_) | TypeOf(_) | List(_)  | Map(_)  => Ok(self.clone()),
            Symbol(name) => handle_symbol(name, scope),
            Call(name, args) => handle_call(name, args, scope),
            _ => panic!("not implemented {:?}", self),
        }
    }
    pub fn mut_eval(&self, scope: &mut Scope) -> Result<Expr, Exception> {
        match self {
            Block(body) => handle_block(body, scope),
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
    pub fn as_block(&self) -> Expr {
        match self {
            Block(_) => self.clone(),
            _ => Block(vec!(self.clone())),
        }
    }

    pub fn print(&self) -> String {
        self.to_string()
    }
}

fn format_vec<T: ToString>(vec: &[T], separ: &str, prefix: &str, suffix: &str) -> String {
    format!("{}{}{}", prefix, vec.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(separ), suffix)
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
    scope.get_value(name).ok_or_else(|| Exception::UndefinedSymbol(name.to_string()))
}

fn handle_call(name: &str, args: &Vec<Expr>, scope: &Scope) -> Result<Expr, Exception> {
    match scope.find(name) {
        Some(Fun(name, types, fun)) => apply_fun(name, types, args, fun, scope),
        _ if args.len() == 0 => Err(Exception::UndefinedFunction(name.to_string())),
        _ => {
            let method_name = args[0].eval(scope)?.get_type().method_name(name);
            match scope.global().get(&method_name) {
                Some(Fun(name, types, fun)) => apply_fun(name, types, args, fun, scope),
                _ => Err(Exception::UndefinedMethod(method_name)),
            }
        }
    }
}



fn handle_macro(scope: &mut Scope, name: &String, args: &Vec<Expr>) -> Result<Expr, Exception> {
    if let Some(Fun(_, _, BuiltIn(lambda))) = scope.global().get(name) {
        lambda(args, scope)
    } else {
        Err(Exception::NotDefined(name.to_string()))
    }
}


fn handle_block(body: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    let mut result = Ok(Null);
    for expr in body {
        result = expr.mut_eval(scope);
        if result.is_err() {
            break;
        }
    }
    result
}

fn apply_fun(name: &str, specs: &Type, args: &Vec<Expr>, fun: &Function, scope: &Scope) ->  Result<Expr, Exception> {
    args.iter().map(|e| e.eval(scope)).collect::<Result<Vec<Expr>, Exception>>().and_then(|values| {
        match specs {
            Type::Fun(input, _output) => check_arguments(name, input, &values).or(Some(fun.apply(&values, scope))).unwrap(),
            _ => Err(Exception::NotA("Fun".to_owned(), specs.to_string())),
        }
    })
}

fn check_arguments(name: &str, expected: &Vec<Type>, values: &Vec<Expr>) -> Option<Result<Expr, Exception>> {
    if matches!(expected.get(0), Some(Type::List(..))) {
        return None
    }
    if expected.len() != values.len() {
        return Some(Err(Exception::WrongArgumentsNumber(name.to_owned(), expected.len(), values.len())))
    }
    expected.iter().zip(values.iter()).find(|(e, v)| **e != v.get_type()).and_then(|p|
        Some(Err(Exception::UnexpectedArgumentType(name.to_owned(), p.1.get_type().to_string())))
    )
}


