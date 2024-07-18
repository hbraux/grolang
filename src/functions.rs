use std::fmt::Debug;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Bool, Float, Fun, Int, Nil, Symbol};
use crate::Scope;
use crate::types::Type;
use crate::types::Type::Lazy;
use self::Function::{BuiltIn, Defined};

macro_rules! if_else {
    ($condition:expr => $true_branch:expr ; $false_branch:expr) => {
        if $condition { $true_branch } else { $false_branch }
    };
}


#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    BuiltIn(fn(&Vec<Expr>, &mut Scope) -> Result<Expr, Exception>),
    Defined(Vec<String>, Box<Expr>)
}


impl Function {
    pub fn apply(&self, args: &Vec<Expr>, scope:  &mut  Scope) -> Result<Expr, Exception> {
        match self {
            BuiltIn(inner) => inner(args, scope),
            Defined(_params,body) => body.eval(scope),
        }
    }
}



pub fn load_functions(scope: &mut Scope) {
    // int arithmetics
    let spec = || Type::new("(Int,Int)->Int");
    scope.add(Fun("Int.add".to_owned(), spec(), BuiltIn(|args, _| Ok(Int(args[0].int()? + args[1].int()?)))));
    scope.add(Fun("Int.sub".to_owned(), spec(), BuiltIn(|args, _| Ok(Int(args[0].int()? - args[1].int()?)))));
    scope.add(Fun("Int.mul".to_owned(), spec(), BuiltIn(|args, _| Ok(Int(args[0].int()? * args[1].int()?)))));
    scope.add(Fun("Int.div".to_owned(), spec(), BuiltIn(|args, _| divide_int(args[0].int()?, args[1].int()?))));
    scope.add(Fun("Int.mod".to_owned(), spec(), BuiltIn(|args, _| modulo_int(args[0].int()?, args[1].int()?))));

    // float arithmetics
    let spec = || Type::new("(Float,Float)->Float");
    scope.add(Fun("Float.add".to_owned(), spec(), BuiltIn(|args, _| Ok(Float(args[0].float()? + args[1].float()?)))));
    scope.add(Fun("Float.sub".to_owned(), spec(), BuiltIn(|args, _| Ok(Float(args[0].float()? - args[1].float()?)))));
    scope.add(Fun("Float.mul".to_owned(), spec(), BuiltIn(|args, _| Ok(Float(args[0].float()? * args[1].float()?)))));
    scope.add(Fun("Float.div".to_owned(), spec(), BuiltIn(|args, _| divide_float(args[0].float()?, args[1].float()?))));
    // boolean logic
    let spec = || Type::new("(Bool,Bool)->Bool");
    scope.add(Fun("Bool.and".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].bool()? && args[1].bool()?)))));
    scope.add(Fun("Bool.or".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].bool()? || args[1].bool()?)))));
    // comparisons
    let spec = || Type::new("(Int,Int)->Bool");
    scope.add(Fun("Int.eq".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].int()? == args[1].int()?)))));
    scope.add(Fun("Int.neq".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].int()? != args[1].int()?)))));
    scope.add(Fun("Int.gt".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].int()? > args[1].int()?)))));
    scope.add(Fun("Int.ge".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].int()? >= args[1].int()?)))));
    scope.add(Fun("Int.lt".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].int()? < args[1].int()?)))));
    scope.add(Fun("Int.le".to_owned(), spec(), BuiltIn(|args, _| Ok(Bool(args[0].int()? <= args[1].int()?)))));

    // lazy functions
    scope.add(Fun("var".to_owned(), Lazy, BuiltIn(|args, scope| declare(args[0].symbol()?, args[1].to_type()?, args[2].eval(scope)?, scope, true))));
    scope.add(Fun("val".to_owned(), Lazy, BuiltIn(|args, scope| declare(args[0].symbol()?, args[1].to_type()?, args[2].eval(scope)?, scope, false))));
    scope.add(Fun("fun".to_owned(), Lazy, BuiltIn(|args, scope| define(args[0].symbol()?, args[1].to_params()?, args[2].to_type()?, &args[3], scope))));
    scope.add(Fun("set".to_owned(), Lazy, BuiltIn(|args, scope| assign(args[0].symbol()?, args[1].eval(scope)?, scope))));
    scope.add(Fun("block".to_owned(), Lazy, BuiltIn(|args, scope| block(args, scope))));
    scope.add(Fun("print".to_owned(), Lazy, BuiltIn(|args, scope| print(args, scope))));
    scope.add(Fun("while".to_owned(), Lazy, BuiltIn(|args, scope| run_while(args, scope))));
    scope.add(Fun("if".to_owned(), Lazy, BuiltIn(|args, scope| if_else!(args[0].eval(scope)?.bool()? => args[1].eval(scope) ; args[2].eval(scope)))));
}

fn divide_int(a: &i64, b: &i64) ->  Result<Expr, Exception> {
    if *b != 0 { Ok(Int(a/b)) } else { Err(Exception::DivisionByZero)}
}
fn modulo_int(a: &i64, b: &i64) ->  Result<Expr, Exception> {
    if *b != 0 { Ok(Int(a % b)) } else { Err(Exception::DivisionByZero)}
}
fn divide_float(a: &f64, b: &f64) ->  Result<Expr, Exception> {
    if *b != 0.0 { Ok(Float(a/b)) } else { Err(Exception::DivisionByZero)}
}


fn declare(name: &str, expected: &Type, value: Expr, scope: &mut Scope, is_mutable: bool) -> Result<Expr, Exception> {
    if *expected != Type::Any && *expected != value.get_type()  {
        Err(Exception::UnexpectedType(value.get_type().to_string()))
    } else if scope.is_defined(&name) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        scope.set(name.to_owned(), value, Some(is_mutable));
        Ok(Symbol(name.to_owned()))
    }
}

fn define(name: &str, params: &Vec<(String, Type)>, output: &Type, expr: &Expr, scope: &mut Scope) -> Result<Expr, Exception> {
    if scope.is_defined(&name) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        let types = Type::Fun(params.iter().map(|p| p.1.clone()).collect(), Box::new(output.clone()));
        scope.add(Fun(name.to_owned(), types, Defined(params.iter().map(|p| p.0.clone()).collect(), Box::new(expr.clone()))));
        Ok(Symbol(name.to_owned()))
    }
}


fn assign(name: &str, value: Expr, scope: &mut Scope) -> Result<Expr, Exception> {
    match scope.is_mutable(&name) {
        None  => Err(Exception::NotDefined(name.to_owned())),
        Some(false) => Err(Exception::NotMutable(name.to_owned())),
        _ if scope.get_type(name) != value.get_type() => Err(Exception::UnexpectedType(value.get_type().to_string())),
        _ => {
            scope.set(name.to_owned(), value.clone(), None);
            Ok(value)
        }
    }
}

fn block(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    let mut result = Ok(Nil);
    for arg in args {
        result = arg.eval(scope);
        if result.is_err() {
            break;
        }
    }
    result
}

fn print(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    for x in args {
        print!("{}", x.eval(scope)?)
    }
    println!();
    Ok(Nil)
}

fn run_while(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    let mut count = 0;
    let mut result = Ok(Nil);
    loop {
        count += 1;
        if count >= 1000000 {
            break Err(Exception::InfiniteLoop)
        }
        if let Bool(bool) = args[0].eval(scope)? {
            if bool {
                result = args[1].eval(scope)
            } else {
                break result;
            }
        } else {
            break Err(Exception::NotA(Type::Bool.to_string(), args[0].to_string()))
        }
    }
}


