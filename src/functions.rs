use std::fmt::Debug;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Bool, Float, Fun, Int, LazyFun, Nil, Symbol};
use crate::Scope;
use crate::types::Type;

macro_rules! if_else {
    ($condition:expr => $true_branch:expr ; $false_branch:expr) => {
        if $condition { $true_branch } else { $false_branch }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    inner: fn(&Vec<Expr>) -> Result<Expr, Exception>,
}

// arguments of lazy functions arguments are not evaluated before the call, and a lazy function can alter the scope
#[derive(Debug, Clone, PartialEq)]
pub struct LazyFunction {
    inner: fn(&Vec<Expr>, &mut Scope) -> Result<Expr, Exception>,
}

impl Function {
    pub fn new(inner: fn(&Vec<Expr>) -> Result<Expr, Exception>) -> Function { Function { inner } }
    pub fn apply(&self, args: &Vec<Expr>) -> Result<Expr, Exception> { (self.inner)(args) }
}

impl LazyFunction {
    pub fn new(inner: fn(&Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception>) -> LazyFunction { LazyFunction { inner } }
    pub fn apply(&self, args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> { (self.inner)(args, scope) }

}


pub fn load_functions(scope: &mut Scope) {
    // int arithmetics
    let spec = || Type::new("(Int,Int)->Int");
    scope.add(Fun("Int.add".to_owned(), spec(), Function::new(|args| Ok(Int(args[0].int()? + args[1].int()?)))));
    scope.add(Fun("Int.sub".to_owned(), spec(), Function::new(|args| Ok(Int(args[0].int()? - args[1].int()?)))));
    scope.add(Fun("Int.mul".to_owned(), spec(), Function::new(|args| Ok(Int(args[0].int()? * args[1].int()?)))));
    scope.add(Fun("Int.div".to_owned(), spec(), Function::new(|args| divide_int(args[0].int()?, args[1].int()?))));
    scope.add(Fun("Int.mod".to_owned(), spec(), Function::new(|args| modulo_int(args[0].int()?, args[1].int()?))));

    // float arithmetics
    let spec = || Type::new("(Float,Float)->Float");
    scope.add(Fun("Float.add".to_owned(), spec(), Function::new(|args| Ok(Float(args[0].float()? + args[1].float()?)))));
    scope.add(Fun("Float.sub".to_owned(), spec(), Function::new(|args| Ok(Float(args[0].float()? - args[1].float()?)))));
    scope.add(Fun("Float.mul".to_owned(), spec(), Function::new(|args| Ok(Float(args[0].float()? * args[1].float()?)))));
    scope.add(Fun("Float.div".to_owned(), spec(), Function::new(|args| divide_float(args[0].float()?, args[1].float()?))));
    // boolean logic
    let spec = || Type::new("(Bool,Bool)->Bool");
    scope.add(Fun("Bool.and".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].bool()? && args[1].bool()?)))));
    scope.add(Fun("Bool.or".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].bool()? || args[1].bool()?)))));
    // comparisons
    let spec = || Type::new("(Int,Int)->Bool");
    scope.add(Fun("Int.eq".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].int()? == args[1].int()?)))));
    scope.add(Fun("Int.neq".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].int()? != args[1].int()?)))));
    scope.add(Fun("Int.gt".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].int()? > args[1].int()?)))));
    scope.add(Fun("Int.ge".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].int()? >= args[1].int()?)))));
    scope.add(Fun("Int.lt".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].int()? < args[1].int()?)))));
    scope.add(Fun("Int.le".to_owned(), spec(), Function::new(|args| Ok(Bool(args[0].int()? <= args[1].int()?)))));

    // lazy functions
    scope.add(LazyFun("var".to_owned(), LazyFunction::new(|args, scope| declare(args[0].symbol()?, args[1].to_type()?, args[2].eval(scope)?, scope, true))));
    scope.add(LazyFun("val".to_owned(), LazyFunction::new(|args, scope| declare(args[0].symbol()?, args[1].to_type()?, args[2].eval(scope)?, scope, false))));
    scope.add(LazyFun("fun".to_owned(), LazyFunction::new(|args, scope| define(args[0].symbol()?, args[1].to_parameters()?, args[2].to_type()?, &args[3], scope))));
    scope.add(LazyFun("set".to_owned(), LazyFunction::new(|args, scope| assign(args[0].symbol()?, args[1].eval(scope)?, scope))));
    scope.add(LazyFun("block".to_owned(), LazyFunction::new(|args, scope| block(args, scope))));
    scope.add(LazyFun("print".to_owned(), LazyFunction::new(|args, scope| print(args, scope))));
    scope.add(LazyFun("while".to_owned(), LazyFunction::new(|args, scope| run_while(args, scope))));
    scope.add(LazyFun("if".to_owned(), LazyFunction::new(|args, scope| if_else!(args[0].eval(scope)?.bool()? => args[1].eval(scope) ; args[2].eval(scope)))));
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

fn define(name: &str, params: &Vec<(String,Type)>, output: &Type, expr: &Expr, scope: &mut Scope) -> Result<Expr, Exception> {
    if scope.is_defined(&name) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        let spec = Type::Fun(params.iter().map(|e| e.1.clone()).collect(), Box::new(output.clone()));
        scope.add(Fun(name.to_owned(), spec, Function::new(|args| todo!())));
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


#[cfg(test)]
mod tests {
    use crate::expr::Expr::{Float, Int};

    use super::*;

    #[test]
    fn test_apply() {
        let fun = Function::new(|args| divide_int(args[0].int()?, args[1].int()?));
        assert_eq!(Ok(Int(3)), fun.apply(&vec!(Int(6), Int(2))));
        assert_eq!(Err(Exception::NotA("Int".to_owned(), "1.1".to_owned())), fun.apply(&vec!(Int(1), Float(1.1))));
        assert_eq!(Err(Exception::DivisionByZero), fun.apply(&vec!(Int(1), Int(0))));
    }
}
