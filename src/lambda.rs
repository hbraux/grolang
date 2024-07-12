use std::fmt::{Debug, Formatter};

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Fun, Int};
use crate::Scope;
use crate::types::Type;

pub struct Lambda {
    name: String,
    fun: fn(Vec<Expr>) -> Result<Expr, Exception>,
}

impl Lambda {
    pub fn new(name: &str, fun: fn(Vec<Expr>) -> Result<Expr, Exception>) -> Lambda { Lambda { name : name.to_owned(), fun } }
    pub fn name(&self) -> String { self.name.clone() }
    pub fn apply(&self, args: Vec<Expr>) -> Result<Expr, Exception> {
        (self.fun)(args)
    }

}

impl Clone for Lambda {
    fn clone(&self) -> Self { panic!("Lambda cannot be cloned") }
}
impl Debug for Lambda {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
}


pub fn load_functions(scope: &mut Scope) {
    let spec = || Type::new("(Int,Int)->Int");
    scope.add_fun(Fun(spec(), Lambda::new("Int.add", |args| Ok(Int(args[0].int()? + args[1].int()?)))));
    scope.add_fun(Fun(spec(), Lambda::new("Int.sub", |args| Ok(Int(args[0].int()? - args[1].int()?)))));
    scope.add_fun(Fun(spec(), Lambda::new("Int.mul", |args| Ok(Int(args[0].int()? * args[1].int()?)))));
    scope.add_fun(Fun(spec(), Lambda::new("Int.div", |args| divide(args[0].int()?, args[1].int()?))));
}

fn divide(a: &i64, b: &i64) ->  Result<Expr, Exception> {
    if *b != 0 { Ok(Int(a/b)) } else { Err(Exception::DivisionByZero)}
}



#[cfg(test)]
mod tests {
    use crate::exception::Exception::NotInt;
    use crate::expr::Expr::{Float, Int};
    use super::*;

    #[test]
    fn test_apply() {
        let lambda = Lambda::new("Int.div", |args| divide(args[0].int()?, args[1].int()?));
        assert_eq!(Ok(Int(3)), lambda.apply(vec!(Int(6),Int(2))));
        assert_eq!(Err(NotInt("1.1".to_owned())), lambda.apply(vec!(Int(1),Float(1.1))));
        assert_eq!(Err(Exception::DivisionByZero), lambda.apply(vec!(Int(1),Int(0))));
    }
}
