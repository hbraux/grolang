use std::collections::{HashMap, HashSet};
use std::string::ToString;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Fun, LazyFun};
use crate::functions::{Function, load_functions};
use crate::types::Type;

pub mod expr;
mod parser;
mod types;
mod exception;
mod functions;


pub struct Scope {
    values: HashMap<String, Expr>,
    mutables: HashSet<String>,
}

impl Scope {
    pub fn new() -> Scope { Scope { values: HashMap::new(), mutables: HashSet::new() }}

    pub fn init() -> Scope {
        let mut scope = Scope::new();
        load_functions(&mut scope);
        scope
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.values.get(name).map(|e| e.clone())
    }

    pub fn try_lazy(&mut self, name: &str, args: &Vec<Expr>) -> Option<Result<Expr, Exception>> {
        match self.values.get(name) {
            Some(LazyFun(_name, lambda)) => Some(lambda.clone().apply(args, self)),
            _ => None,
        }
    }
    pub fn get_fun(&self, name: &str, obj_type: Option<Type>) -> Option<(&Type, &Function)> {
        match self.values.get(name) {
            Some(Fun(_name, specs, lambda)) => Some((specs, lambda)),
            None if obj_type.is_some() => self.get_fun(&(obj_type.unwrap().method_name(name)), None),
            _ => None,
        }
    }
    pub fn add(&mut self, value: Expr) {
        match &value {
            Fun(name, _type, _lambda) => self.values.insert(name.to_owned(), value),
            LazyFun(name, _lambda) => self.values.insert(name.to_owned(), value),
            _ => panic!("cannot add {}", value)
        };
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    pub fn is_mutable(&self, name: &str) -> Option<bool> {
        if self.is_defined(name) {
            Some(self.mutables.contains(name))
        } else { None }
    }
    pub fn get_type(&self, name: &str) -> Type {
        self.values.get(name).unwrap().get_type()
    }

    pub fn set(&mut self, name: String, value: Expr, is_mutable: Option<bool>) {
        if is_mutable == Some(true) {
            self.mutables.insert(name.to_string());
        }
        self.values.insert(name, value);
    }
    pub fn read(&mut self, str: &str) -> Expr { Expr::read(str, self) }
    pub fn exec(&mut self, str: &str) -> String { self.read(str).eval_or_failed(self).print() }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literals() {
        let mut scope = Scope::init();
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
        let mut scope = Scope::init();
        assert_eq!("NotDefined(a)", scope.exec("a = 0"));
        assert_eq!("a", scope.exec("var a = 1"));
        assert_eq!("z", scope.exec("z.val(nil, true)"));
        assert_eq!("AlreadyDefined(a)", scope.exec("var a = 3"));
        assert_eq!("2", scope.exec("a = a + 1"));
        assert_eq!("0", scope.exec("a.set(0)"));
        assert_eq!("UnexpectedType(Float)", scope.exec("a = 3.0"));
        assert_eq!("c", scope.exec("val c=3.2"));
        assert_eq!("UnexpectedType(Float)", scope.exec("var d: Int = 3.2"));
        assert_eq!("0", scope.exec("a"));
        assert_eq!("3.2", scope.exec("c"));
        assert_eq!("i", scope.exec("val i = 0"));
        assert_eq!("NotMutable(i)", scope.exec("i = 1"));
    }

    #[test]
    fn test_arithmetics() {
        let mut scope = Scope::init();
        assert_eq!("14", scope.exec("2 + 3 * 4"));
        assert_eq!("20", scope.exec("(2 + 3) * 4"));
        assert_eq!("4", scope.exec("4 / 1"));
        assert_eq!("2", scope.exec("22%10"));
        assert_eq!("2", scope.exec("-2 * -1"));
        assert_eq!("3.3", scope.exec("1.0 + 2.3"));
        assert_eq!("DivisionByZero", scope.exec("1 / 0"));
    }

    #[test]
    fn test_comparisons() {
        let mut scope = Scope::init();
        scope.exec("var a = 1");
        scope.exec("var b = 2");
        assert_eq!("true", scope.exec("a == a"));
        assert_eq!("true", scope.exec("1 == a"));
        assert_eq!("false", scope.exec("a == b"));
        assert_eq!("true", scope.exec("a != b"));
        assert_eq!("true", scope.exec("a == 1 && b == 2"));
        assert_eq!("false", scope.exec("a == 1 && b == 1"));
        assert_eq!("false", scope.exec("a == 2 && b == 2"));
    }

    #[test]
    fn test_if_else() {
        let mut scope = Scope::init();
        assert_eq!("1", scope.exec("if (true) { 1 } else { 0 }"));
    }

    #[test]
    fn test_print() {
        let mut scope = Scope::init();
        assert_eq!("nil", scope.exec("print(\"hello world\")"));
    }

    #[test]
    fn test_while() {
        let mut scope = Scope::init();
        scope.exec("var a = 0");
        assert_eq!("11", scope.exec("while (a < 10) { a = a + 1 }"));
    }
}
