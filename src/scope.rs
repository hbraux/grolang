use std::collections::{HashMap, HashSet};
use std::string::ToString;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Fun};
use crate::functions::{Function, load_functions};
use crate::types::Type;


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
    pub fn extend(&self) -> Scope {
        Scope::new(Some(self))
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.values.get(name).map(|e| e.clone())
    }

    pub fn try_lazy(&mut self, name: &str, args: &Vec<Expr>) -> Option<Result<Expr, Exception>> {
        match self.values.get(name) {
            Some(Fun(_name, types, lambda)) if types.is_lazy() => Some(lambda.clone().apply(args, self)),
            _ => None,
        }
    }

    pub fn get_fun(&self, name: &str, obj_type: Option<Type>) -> Option<(&Type, &Function)> {
        match self.values.get(name) {
            Some(Fun(_name, specs, lambda)) => Some((specs, lambda)),
           // Some(Lambda(_name, specs, args, body)) => Some((specs, lambda)),
            None if obj_type.is_some() => self.get_fun(&(obj_type.unwrap().method_name(name)), None),
            _ => None,
        }
    }
    pub fn add(&mut self, value: Expr) {
        match &value {
            Fun(name, _type, _) => self.values.insert(name.to_owned(), value),
            _ => panic!("cannot add {}", value)
        };
    }

    pub fn with(&mut self, vars: Vec<String>, values: Vec<Expr>) -> &Scope {
        // TODO: extend scope
        values.iter().zip(vars.iter()).for_each(|(v ,n)| { self.values.insert(n.to_string(), v.clone()); });
        self
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

