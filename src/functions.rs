use std::fmt::{Debug, Formatter};

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Float, Fun, Int};
use crate::Scope;
use crate::types::Type;

pub struct Function {
    inner: fn(&Vec<Expr>) -> Result<Expr, Exception>,
}

impl Function {
    pub fn new(inner: fn(&Vec<Expr>) -> Result<Expr, Exception>) -> Function { Function { inner } }
    pub fn apply(&self, args: &Vec<Expr>) -> Result<Expr, Exception> { (self.inner)(args) }
}

impl Clone for Function {
    fn clone(&self) -> Self { panic!("Function cannot be cloned") }
}
impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("...")
    }
}
impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool { true }
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



#[cfg(test)]
mod tests {
    use crate::exception::Exception::NotInt;
    use crate::expr::Expr::{Float, Int};

    use super::*;

    #[test]
    fn test_apply() {
        let lambda = Function::new(|args| divide_int(args[0].int()?, args[1].int()?));
        assert_eq!(Ok(Int(3)), lambda.apply(&vec!(Int(6),Int(2))));
        assert_eq!(Err(NotInt("1.1".to_owned())), lambda.apply(&vec!(Int(1),Float(1.1))));
        assert_eq!(Err(Exception::DivisionByZero), lambda.apply(&vec!(Int(1),Int(0))));
    }
}
