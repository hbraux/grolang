use std::collections::{HashMap, HashSet};
use std::string::ToString;

use crate::fail::Fail;
use crate::expr::Expr;

use crate::types::Type;

mod expr;
mod parser;
mod types;
mod fail;
mod builtin;



pub struct Scope {
    values: HashMap<String, Expr>,
    mutables: HashSet<String>,
}

impl Scope {
    pub fn new() -> Scope { Scope { values: HashMap::new(), mutables: HashSet::new() } }

    pub fn get(&self, name: &str) -> Result<Expr, Fail> {
        match self.values.get(name) {
            Some(expr) => Ok(expr.clone()),
            None => Err(Fail::UndefinedSymbol(name.to_string())),
        }
    }
    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    pub fn is_mutable(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    pub fn get_type(&self, name: &str) -> Type {
        self.values.get(name).unwrap().get_type()
    }
    pub fn set(&mut self, name: &str, expr: &Expr, is_mutable: Option<bool>) {
        if is_mutable == Some(true) {
            self.mutables.insert(name.to_string());
        }
        self.values.insert(name.to_string(), expr.clone());
    }
    pub fn read(&mut self, str: &str) -> Expr { Expr::read(str, self) }
    pub fn exec(&mut self, str: &str) -> String { self.read(str).eval_or_error(self).print() }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literals() {
        let mut ctx = Scope::new();
        assert_eq!("1", ctx.exec("1"));
        assert_eq!("9123456", ctx.exec("9_123_456"));
        assert_eq!("2.0", ctx.exec("2.0"));
        assert_eq!("-1.23", ctx.exec("-1.23"));
        assert_eq!("23000.0", ctx.exec("2.3e4"));
        assert_eq!("false", ctx.exec("false"));
        assert_eq!("true", ctx.exec("true"));
        assert_eq!("nil", ctx.exec("nil"));
        assert_eq!("\"abc\"", ctx.exec("\"abc\""));
    }

    #[test]
    fn test_variables() {
        let mut ctx = Scope::new();
        assert_eq!("Error(NotDefined(\"a\"))", ctx.exec("a = 0"));
        assert_eq!("1", ctx.exec("var a = 1"));
        assert_eq!("true", ctx.exec("'z.defval(true)"));
        assert_eq!("Error(AlreadyDefined(\"a\"))", ctx.exec("var a = 3"));
        assert_eq!("3", ctx.exec("a = 3"));
        assert_eq!("0", ctx.exec("'a.set(0)"));
        assert_eq!("Error(InconsistentType(\"Float\"))", ctx.exec("a = 3.0"));
        assert_eq!("3.2", ctx.exec("val c=3.2"));
        assert_eq!("Error(InconsistentType(\"Float\"))", ctx.exec("var d: Int = 3.2"));
        assert_eq!("0", ctx.exec("a"));
        assert_eq!("3.2", ctx.exec("c"));
    }

    #[test]
    fn test_arithmetics() {
        let mut ctx = Scope::new();
        assert_eq!("14", ctx.exec("2 + 3 * 4"));
        assert_eq!("20", ctx.exec("(2 + 3) * 4"));
        assert_eq!("4", ctx.exec("4 / 1"));
        assert_eq!("2", ctx.exec("22%10"));
        assert_eq!("2", ctx.exec("-2 * -1"));
        assert_eq!("3.3", ctx.exec("1 + 2.3"));
        assert_eq!("Error(DivisionByZero)", ctx.exec("1 / 0"));
    }

    #[test]
    fn test_comparisons() {
        let mut ctx = Scope::new();
        assert_eq!("1", ctx.exec("var a = 1"));
        assert_eq!("2", ctx.exec("var b = 2"));
        assert_eq!("true", ctx.exec("a == a"));
        assert_eq!("true", ctx.exec("1 == a"));
        assert_eq!("false", ctx.exec("a == b"));
        assert_eq!("true", ctx.exec("a != b"));
        assert_eq!("true", ctx.exec("a == 1 && b == 2"));
        assert_eq!("false", ctx.exec("a == 1 && b == 1"));
    }

    #[test]
    fn test_ifelse() {
        let mut ctx = Scope::new();
        assert_eq!("14", ctx.exec("if (true) { 1 } else { 0 }"))
    }
}
