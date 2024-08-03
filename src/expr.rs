use std::cmp::PartialEq;
use strum_macros::Display;

use crate::exception::Exception;
use crate::functions::Function;
use crate::functions::Function::BuiltIn;
use crate::if_else;
use crate::parser::parse;
use crate::scope::Scope;
use crate::types::Type;

use self::Expr::{Block, Bool, Call, Failure, Float, Fun, Int, List, Map, Nil, Params, Str, Symbol, TypeOf};

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Expr {
    Nil,
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Symbol(String),
    TypeOf(Type),
    Block(Vec<Expr>),
    Call(String, Vec<Expr>),
    Failure(Exception),
    Fun(String, Type, Function),
    List(Type, Vec<Expr>),
    Map(Type, Vec<(Expr, Expr)>),
    Struct(String, Vec<(String, Type)>),
    Params(Vec<(String, Type)>),
}


pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NIL: Expr = Nil;



impl Expr {
    pub fn read(str: &str, _ctx: &Scope) -> Expr {
        parse(str).unwrap_or_else(|s| Failure(Exception::CannotParse(s)))
    }
    pub fn name(&self) -> String {
        self.to_string()
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, Failure(_))
    }
    pub fn is_fun(&self) -> bool {
        matches!(self, Fun(_, _, _))
    }

    pub fn to_exception(&self) -> &Exception {
        match self { Failure(ex) => ex, _ => panic!("not a failure") }
    }

    pub fn get_type(&self) -> &Type {
        match self {
            Nil => &Type::Any,
            Bool(_) => &Type::Bool,
            Int(_) => &Type::Int,
            Float(_) => &Type::Float,
            Str(_) => &Type::Str,
            List(t, _) => t,
            Map(t, _) => t,
            _ => panic!("unknown type {:?}", self)
        }
    }
    pub fn to_str(&self) -> Result<&str, Exception> {
        match self {
            Str(str) => Ok(str),
            _ => Err(Exception::NotA(Type::Str.name(), self.print()))
        }
    }
    pub fn to_bool(&self) -> Result<bool, Exception> {
        match self {
            Bool(str) => Ok(str.to_owned()),
            _ => Err(Exception::NotA(Type::Bool.name(), self.print()))
        }
    }
    pub fn to_symbol(&self) -> Result<&str, Exception> {
        match self {
            Symbol(str) => Ok(str),
            _ => Err(Exception::UndefinedSymbol(self.print()))
        }
    }
    pub fn to_type(&self) -> Result<&Type, Exception> {
        match self {
            TypeOf(t) => Ok(t),
            Nil => Ok(&Type::_Undefined),
            _ => Err(Exception::NotA("Type".to_owned(), self.print()))
        }
    }
    pub fn to_params(&self) -> Result<&Vec<(String, Type)>, Exception> {
        match self {
            Params(v) => Ok(v),
            _ => Err(Exception::NotA("Params".to_owned(), self.print()))
        }
    }
    // simple evaluation with immutable scope
    pub fn eval(&self, scope: &Scope) -> Result<Expr, Exception> {
        match self {
            Failure(e) => Err(e.clone()),
            Nil | Int(_) | Float(_) | Str(_) | Bool(_)  | List(_,_ )  | Map(_, _)  => Ok(self.clone()),
            Symbol(name) => handle_symbol(name, scope),
            Call(name, args) => handle_call(name, args, scope),
            _ => panic!("not implemented {:?}", self),
        }
    }
    //  evaluation with mutable scope
    pub fn eval_mutable(&self, scope: &mut Scope) -> Result<Expr, Exception> {
        match self {
            Block(body) => handle_block(body, scope),
            Call(name, args) if scope.is_macro(name) => handle_macro(scope, name, args),
            _ => self.eval(scope)
        }
    }
    pub fn eval_or_failed(&self, scope: &mut Scope) -> Expr {
        match self {
            Failure(_) => self.clone(),
            expr => expr.eval_mutable(scope).unwrap_or_else(|ex| Failure(ex))
        }
    }
    pub fn expect(self, expected: &Type) -> Result<Expr, Exception> {
        let value_type = self.get_type();
        if expected.is_defined() {
            if value_type.is_defined() {
                if !value_type.matches(expected) {
                    return Err(Exception::UnexpectedType(value_type.print()));
                }
            } else {
                // soft cast
                return match self {
                    List(_, vec) => Ok(List(expected.clone(), vec.clone())),
                    Map(_, vec) => Ok(Map(expected.clone(), vec.clone())),
                    _ => Err(Exception::CannotCastType(expected.print())),
                }
            }
        } else if !value_type.is_defined() {
            return Err(Exception::CannotInferType(value_type.print()));
        }
        Ok(self)
    }


    pub fn as_block(&self) -> Expr {
        match self {
            Block(_) => self.clone(),
            _ => Block(vec!(self.clone())),
        }
    }

    pub fn print(&self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            TypeOf(x) => format!(":{}", x.print()),
            Nil => "nil".to_owned(),
            Float(x) => print_float(x),
            Symbol(x) => x.to_owned(),
            Failure(x) => x.print(),
            Params(vec) => print_vec(vec, ",", "(", ")", |p| format!("{}:{}", p.0, p.1)),
            Map(_, vec) => print_vec(vec, ",", "{", "}", |p| format!("{}:{}", p.0.print(), p.1.print())),
            List(_, vec) => print_vec(vec, ",", "[", "]", Expr::print),
            Block(vec) => print_vec(vec, ";", "{", "}", Expr::print),
            Call(name, vec) => print_vec(vec, ",", &(name.to_owned() + "("), ")",  Expr::print),
            _ => self.name()
        }
    }
}

fn print_vec<T>(vec: &[T], separ: &str, prefix: &str, suffix: &str, fmt: fn(t: &T) -> String) -> String {
    format!("{}{}{}", prefix, vec.iter().map(fmt).collect::<Vec<_>>().join(separ), suffix)
}

fn print_float(x: &f64) -> String  {
    let str = x.to_string();
    if_else!(str.contains('.'), str, format!("{}.0", str))
}

fn handle_symbol(name: &str, scope: &Scope) -> Result<Expr, Exception> {
    scope.get_value(name).ok_or_else(|| Exception::UndefinedSymbol(name.to_owned()))
}

// TODO: impl a better solution to find the eligible functions
fn handle_call(name: &str, args: &Vec<Expr>, scope: &Scope) -> Result<Expr, Exception> {
    match scope.find(name) {
        Some(Fun(name, types, fun)) => apply_fun(name, types, args, fun, scope),
        _ if args.len() == 0 => Err(Exception::UndefinedFunction(name.to_owned())),
        _ => {
            for method in args[0].eval(scope)?.get_type().all_method_names(name) {
                if let Some(Fun(name, types, fun)) =  scope.global().get(&method) {
                    return apply_fun(name, types, args, fun, scope);
                }}
            return Err(Exception::UndefinedMethod(name.to_owned()));
        }
    }
}



fn handle_macro(scope: &mut Scope, name: &String, args: &Vec<Expr>) -> Result<Expr, Exception> {
    if let Some(Fun(_, _, BuiltIn(lambda))) = scope.global().get(name) {
        lambda(args, scope)
    } else {
        Err(Exception::NotDefined(name.to_owned()))
    }
}


fn handle_block(body: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    let mut result = Ok(Nil);
    for expr in body {
        result = expr.eval_mutable(scope);
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
            _ => Err(Exception::NotA("Fun".to_owned(), specs.print())),
        }
    })
}


fn check_arguments(name: &str, expected: &Vec<Type>, values: &Vec<Expr>) -> Option<Result<Expr, Exception>> {
    //println!("#check_arguments({name},{expected:?} {values:?})");
    if matches!(expected.get(0), Some(Type::Macro)) {
        return None
    }
    if matches!(expected.get(0), Some(Type::List(..))) {
        // TODO: handle collections parameters
        return None
    }
    if expected.len() != values.len() {
        return Some(Err(Exception::WrongArgumentsNumber(name.to_owned(), expected.len().to_string(), values.len().to_string())))
    }
    if matches!(expected.get(0), Some(Type::Any)) {
        return None
    }
    expected.iter().zip(values.iter()).find(|(e, v)| !v.get_type().matches(*e)).and_then(|p|
        Some(Err(Exception::UnexpectedArgumentType(name.to_owned(), p.1.get_type().print())))
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let expr = Int(1);
        assert_eq!("1", expr.print());
        assert_eq!("Int", expr.to_string());
        assert_eq!("Int", expr.name());
    }
}
