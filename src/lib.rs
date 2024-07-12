use std::collections::{HashMap, HashSet};
use std::string::ToString;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::Symbol;
use crate::lambda::load_functions;
use crate::types::Type;

pub mod expr;
mod parser;
mod types;
mod exception;
mod builtin;
mod lambda;

pub struct Scope {
    values: HashMap<String, Expr>,
    mutables: HashSet<String>,
}

impl Scope {
    pub fn new() -> Scope { Scope { values: HashMap::new(), mutables: HashSet::new() } }

    pub fn init(&mut self) {
        load_functions(self);
    }

    pub fn get(&self, name: &str) -> Result<Expr, Exception> {
        match self.values.get(name) {
            Some(expr) => Ok(expr.clone()),
            None => Err(Exception::UndefinedSymbol(name.to_string())),
        }
    }
    pub fn add(&mut self, name: String, value: Expr) {
        self.values.insert(name, value);
    }
    pub fn add_fun(&mut self, value: Expr) {
        if let  Expr::Fun(_type, lambda) = &value {
            self.values.insert(lambda.name(), value);
        } else { panic!() }
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    pub fn is_mutable(&self, name: &str) -> bool {
        self.mutables.contains(name)
    }
    pub fn get_type(&self, name: &str) -> Type {
        self.values.get(name).unwrap().get_type()
    }

    pub fn set(&mut self, name: String, value: Expr, is_mutable: Option<bool>) {
        if is_mutable == Some(true) {
            self.mutables.insert(name.to_string());
        }
        self.add(name, value)
    }
    pub fn read(&mut self, str: &str) -> Expr { Expr::read(str, self) }
    pub fn exec(&mut self, str: &str) -> String { self.read(str).eval_or_error(self).print() }

    pub fn store(&mut self, name: String, value: Expr, is_mutable: Option<bool>) -> Result<Expr, Exception> {
        match (self.is_defined(&name), is_mutable) {
            (true, Some(_)) => Err(Exception::AlreadyDefined(name.to_owned())),
            (false, None) => Err(Exception::NotDefined(name.to_owned())),
            (true, None) if !self.is_mutable(&name) => Err(Exception::NotMutable(name.to_owned())),
            (true, None) if self.is_mutable(&name) && self.get_type(&name) != value.get_type() => Err(Exception::InconsistentType(value.get_type().to_string())),
            _ => { self.set(name.clone(), value, is_mutable); Ok(Symbol(name.to_owned())) }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literals() {
        let mut scope = Scope::new();
        assert_eq!("1", scope.exec("1"));
        assert_eq!("9123456", scope.exec("9_123_456"));
        assert_eq!("2.0", scope.exec("2.0"));
        assert_eq!("-1.23", scope.exec("-1.23"));
        assert_eq!("23000.0", scope.exec("2.3e4"));
        assert_eq!("false", scope.exec("false"));
        assert_eq!("true", scope.exec("true"));
        assert_eq!("nil", scope.exec("nil"));
        assert_eq!("\"abc\"", scope.exec("\"abc\""));
    }

    #[test]
    fn test_variables() {
        let mut scope = Scope::new();
        assert_eq!("NotDefined(a)", scope.exec("a = 0"));
        assert_eq!("1", scope.exec("var a = 1"));
        assert_eq!("true", scope.exec("z.val(nil, true)"));
        assert_eq!("AlreadyDefined(a)", scope.exec("var a = 3"));
        assert_eq!("2", scope.exec("a = a + 1"));
        assert_eq!("0", scope.exec("a.set(0)"));
        assert_eq!("InconsistentType(Float)", scope.exec("a = 3.0"));
        assert_eq!("3.2", scope.exec("val c=3.2"));
        assert_eq!("InconsistentType(Float)", scope.exec("var d: Int = 3.2"));
        assert_eq!("0", scope.exec("a"));
        assert_eq!("3.2", scope.exec("c"));
        assert_eq!("0", scope.exec("val i = 0"));
        assert_eq!("NotMutable(i)", scope.exec("i = 1"));
    }

    #[test]
    fn test_arithmetics() {
        let mut scope = Scope::new();
        assert_eq!("14", scope.exec("2 + 3 * 4"));
        assert_eq!("20", scope.exec("(2 + 3) * 4"));
        assert_eq!("4", scope.exec("4 / 1"));
        assert_eq!("2", scope.exec("22%10"));
        assert_eq!("2", scope.exec("-2 * -1"));
        assert_eq!("3.3", scope.exec("1 + 2.3"));
        assert_eq!("DivisionByZero", scope.exec("1 / 0")); // lazy evaluation
    }

    #[test]
    fn test_comparisons() {
        let mut scope = Scope::new();
        assert_eq!("1", scope.exec("var a = 1"));
        assert_eq!("2", scope.exec("var b = 2"));
        assert_eq!("true", scope.exec("a == a"));
        assert_eq!("true", scope.exec("1 == a"));
        assert_eq!("false", scope.exec("a == b"));
        assert_eq!("true", scope.exec("a != b"));
        assert_eq!("true", scope.exec("a == 1 && b == 2"));
        assert_eq!("false", scope.exec("a == 1 && b == 1"));
        assert_eq!("false", scope.exec("a == 2 && b == 2"));
        assert_eq!("false", scope.exec("a == 2 && b/0")); // lazy eval
    }

    #[test]
    fn test_others() {
        let mut scope = Scope::new();
        assert_eq!("1", scope.exec("if (true) { 1 } else { 0 }"));
        assert_eq!("nil", scope.exec("print(\"hello world\")"));
        assert_eq!("0", scope.exec("var a = 0"));
        assert_eq!("11", scope.exec("while (a < 10) { a = a + 1 }"));
    }
}
