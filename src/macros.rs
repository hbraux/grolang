use std::fmt::{Debug, Formatter};

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Mac, Symbol};
use crate::Scope;
use crate::types::Type;

pub struct Macro {
    inner: fn(&Vec<Expr>, &mut Scope) -> Result<Expr, Exception>,
}

impl Macro {
    pub fn new(inner: fn(&Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception>) -> Macro { Macro { inner } }
    pub fn apply(&self, args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> { (self.inner)(args, scope) }

}

impl Clone for Macro {
    fn clone(&self) -> Self { panic!("Macro cannot be cloned") }
}
impl Debug for Macro {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("...")
    }
}
impl PartialEq for Macro {
    fn eq(&self, _other: &Self) -> bool { true }
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

