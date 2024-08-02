use std::collections::{HashMap, HashSet};
use std::string::ToString;

use dialoguer::Completion;

use crate::expr::Expr;
use crate::expr::Expr::Fun;
use crate::functions::add_functions;
use crate::if_else;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Scope<'a> {
    values: HashMap<String, Expr>,
    mutables: HashSet<String>,
    parent: Option<&'a Scope<'a>>,
}

impl Scope<'_> {
    pub fn new<'a>(parent: Option<&'a Scope<'_>>) -> Scope<'a>  { Scope { values: HashMap::new(), mutables: HashSet::new(), parent }}

    pub fn init<'a>() -> Scope<'a>  {
        let mut scope = Scope::new(None);
        add_functions(&mut scope);
        scope
    }
    pub fn child(&self) -> Scope {
        Scope::new(Some(self))
    }
    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.values.get(name)
    }
    pub fn global(&self) -> &Scope {
        self.parent.map(|s| s.global()).or(Some(self)).unwrap()
    }
    pub fn get_value(&self, name: &str) -> Option<Expr> {
        self.get(name).map(|e| e.clone())
    }
    pub fn find(&self, name: &str) -> Option<&Expr> {
        self.values.get(name).or(self.parent.map(|s| s.find(name)).flatten())
    }
    pub fn is_macro(&self, name: &str) -> bool {
        matches!(self.global().get(name), Some(Fun(_, Type::Macro, _)))
    }

    pub fn add_fun(&mut self, value: Expr) {
        match &value {
            Fun(name, _, _) => self.values.insert(name.to_owned(), value),
            _ => panic!("cannot add {}", value)
        };
    }
    pub fn add_args(&mut self, vars: &Vec<String>, values: &Vec<Expr>) {
        values.iter().zip(vars.iter()).for_each(|(v ,n)| {
            self.values.insert(n.to_string(), v.clone());
        });
    }

    pub fn is_defined(&self, name: &str, is_global: bool) -> bool {
        if_else!(is_global, self.global().values.contains_key(name), self.values.contains_key(name))
    }
    pub fn is_mutable(&self, name: &str) -> Option<bool> {
        if self.is_defined(name, false) {
            Some(self.mutables.contains(name))
        } else { None }
    }
    pub fn get_type(&self, name: &str) -> &Type {
        self.values.get(name).unwrap().get_type()
    }

    pub fn set(&mut self, name: &str, value: Expr, is_mutable: Option<bool>) {
        if is_mutable == Some(true) {
            self.mutables.insert(name.to_string());
        }
        self.values.insert(name.to_owned(), value);
    }
    pub fn read(&self, str: &str) -> Expr { Expr::read(str, self) }

    pub fn exec(&mut self, str: &str) -> String { self.read(str).eval_or_failed(self).print() }


    pub fn find_fun(&self, prefix: &str) -> Option<String> {
        if_else!(prefix.is_empty(), None, self.values.iter().find(|i| i.1.is_fun() && i.0.starts_with(prefix)).map(|i| i.0.clone()))
    }
}

impl Completion for Scope<'_>  {
    fn get(&self, input: &str) -> Option<String> {
        let (expr, rest) = match input.rfind(".") {
            Some(p) => (&input[0..p], &input[p..]),
            None =>  (input, ""),
        };
        match self.read(expr).eval(self) {
            Ok(expr) => self.find_fun(&expr.get_type().method_name(rest)),
            _ => self.find_fun(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr::Int;

    use super::*;

    #[test]
    fn test_scope() {
        let mut root = Scope::new(None);
        root.set("a", Int(1), None);
        root.set("b", Int(2), None);

        assert_eq!(root.get("a"), Some(&Int(1)));
        assert_eq!(root.global().get("b"), Some(&Int(2)));

        let mut child = root.child();
        child.set("c", Int(3), None);
        child.set("b", Int(4), None);
        assert_eq!(child.get("a"), None);
        assert_eq!(child.find("a"), Some(&Int(1)));
        assert_eq!(child.get("c"), Some(&Int(3)));
        assert_eq!(child.get("b"), Some(&Int(4)));
        assert_eq!(child.find("b"), Some(&Int(4)));
        assert_eq!(root.global().get("b"), Some(&Int(2)));

    }

}
