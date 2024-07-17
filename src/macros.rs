use std::fmt::Debug;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Bool, Mac, Nil, Symbol};
use crate::Scope;
use crate::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    inner: fn(&Vec<Expr>, &mut Scope) -> Result<Expr, Exception>,
}

impl Macro {
    pub fn new(inner: fn(&Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception>) -> Macro { Macro { inner } }
    pub fn apply(&self, args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> { (self.inner)(args, scope) }

}

pub fn load_macros(scope: &mut Scope) {
    scope.add(Mac("var".to_owned(), Macro::new(|args, scope| declare(args[0].symbol()?, args[1].to_type()?, args[2].eval(scope)?, scope, true))));
    scope.add(Mac("val".to_owned(), Macro::new(|args, scope| declare(args[0].symbol()?, args[1].to_type()?, args[2].eval(scope)?, scope, false))));
    scope.add(Mac("set".to_owned(), Macro::new(|args, scope| assign(args[0].symbol()?, args[1].eval(scope)?, scope))));
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

fn assign(name: &str, value: Expr, scope: &mut Scope) -> Result<Expr, Exception> {
    match scope.is_mutable(&name) {
        None  => Err(Exception::NotDefined(name.to_owned())),
        Some(false) => Err(Exception::NotMutable(name.to_owned())),
        _ => {
            scope.set(name.to_owned(), value.clone(), None);
            Ok(value)
        }
    }
}

fn _call_if(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    if let Bool(bool) = args[0].eval(scope)? {
        if bool {
            args[1].clone().eval(scope)
        } else {
            args[2].clone().eval(scope)
        }
    } else {
        Err(Exception::NotBool(args[0].to_string()))
    }
}


fn _call_print(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    for x in args {
        print!("{:?}", x.eval(scope)?)
    }
    Ok(Nil)
}

fn _call_while(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
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
            break Err(Exception::NotBool(args[0].to_string()))
        }
    }
}
