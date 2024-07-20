use std::collections::{HashMap, HashSet};
use std::string::ToString;

use crate::expr::Expr;
use crate::expr::Expr::Fun;
use crate::functions::{Function, load_functions};
use crate::types::Type;

#[derive(Debug)]
pub struct Scope<'a> {
    values: HashMap<String, Expr>,
    mutables: HashSet<String>,
    parent: Option<&'a Scope<'a>>,
}

impl Scope<'_> {
    pub fn new<'a>(parent: Option<&'a Scope<'_>>) -> Scope<'a>  { Scope { values: HashMap::new(), mutables: HashSet::new(), parent }}

    pub fn init<'a>() -> Scope<'a>  {
        let mut scope = Scope::new(None);
        load_functions(&mut scope);
        scope
    }
    pub fn child(&self) -> Scope {
        Scope::new(Some(self))
    }
    pub fn get_value(&self, name: &str) -> Option<Expr> {
        self.get(name).map(|e| e.clone())
    }

    fn get(&self, name: &str) -> Option<&Expr> {
        self.values.get(name).or(self.parent.map(|e| e.get(name)).flatten())
    }

    pub fn get_global(&self, name: &str) -> Option<&Expr> {
        if self.parent.is_some() {
            self.parent.unwrap().get_global(name)
        } else {
            self.values.get(name)
        }
    }
    pub fn is_mutating_fun(&self, name: &str) -> bool {
        matches!(self.get_global(name), Some(Fun(_, Type::MutatingFun, _)))
    }

    pub fn get_fun(&self, name: &str, self_type: Option<Type>) -> Option<(&String, &Type, &Function)> {
        match self.get(name) {
            Some(Fun(name, specs, fun)) => Some((name, specs, fun)),
            None if self_type.is_some() => self.get_fun(&(self_type.unwrap().method_name(name)), None),
            _ => None,
        }
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

    pub fn set(&mut self, name: &str, value: Expr, is_mutable: Option<bool>) {
        if is_mutable == Some(true) {
            self.mutables.insert(name.to_string());
        }
        self.values.insert(name.to_owned(), value);
    }
    pub fn read(&self, str: &str) -> Expr { Expr::read(str, self) }

    pub fn exec(&mut self, str: &str) -> String { self.read(str).eval_or_failed(self).print() }

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
        assert_eq!(root.get_global("b"), Some(&Int(2)));

        let mut child = root.child();
        child.set("c", Int(3), None);
        child.set("b", Int(4), None);
        assert_eq!(child.get("a"), Some(&Int(1)));
        assert_eq!(child.get("c"), Some(&Int(3)));
        assert_eq!(child.get("b"), Some(&Int(4)));
        assert_eq!(root.get_global("b"), Some(&Int(2)));

    }

}
