use std::borrow::ToOwned;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use crate::if_else;

use self::Type::{_Unknown, Any, Bool, Defined, Float, Fun, Int, List, Map, Option, Str, Try};

macro_rules! box_type {
    ($val:expr) => { Box::new(Type::new($val)) };
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    _Unknown,
    Any,
    Int,
    Bool,
    Str,
    Float,
    List(Box<Type>),
    Option(Box<Type>),
    Try(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Fun(Vec<Type>, Box<Type>),
    Macro,
    Defined(String),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Any => "Any",
            Bool => "Bool",
            Int => "Int",
            Float => "Float",
            Str => "Str",
            List(t) => &format!("List<{}>", t.to_string()),
            Map(t, u) => &format!("Map<{},{}>", t.to_string(), u.to_string()),
            _ => "??"
        };
        write!(f, "{}", str)
    }
}
impl Type {
    pub fn new(str: &str) -> Type {
        if str.starts_with("(") {
            let args: Vec<&str>  = str[1..str.len()].split(")->").collect();
            let v: Vec<&str> = args[0].split(",").collect();
            Fun(v.iter().map(|s| Type::new(s)).collect(), box_type!(args[1]))
        } else if str.ends_with("?") {
            Option(box_type!(&str[0..str.len() - 1]))
        } else if str.ends_with("!") {
            Try(box_type!(&str[0..str.len() - 1]))
        } else if str.starts_with("List<") {
            List(box_type!(&str[5..str.len() - 1]))
        } else if str.starts_with("List<") {
            List(box_type!(&str[5..str.len() - 1]))
        } else if str.starts_with("Map<") {
            let v: Vec<&str> = (&str[4..str.len() - 1]).split(',').collect();
            Map(box_type!(v[0]), box_type!(v[1]))
        } else {
            match str {
                "Any" => Any,
                "Int" => Int,
                "Bool" => Bool,
                "Str" => Str,
                "Float" => Float,
                _ => Defined(str.to_owned()),
            }
        }
    }
    pub fn method_name(&self, name: &str) -> String {
        self.to_string().to_owned() + if_else!(name.starts_with("."), "", ".") + name
    }

    pub fn is_defined(&self) -> bool {
        match self {
            _Unknown => false,
            List(x) => x.is_defined(),
            Map(x, y) => x.is_defined() && y.is_defined(),
            _ => true
        }
    }

    pub fn matches(&self, expected: &Type) -> bool {
        *expected == Any || self == expected
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types() {
        assert_eq!(Any, Type::new("Any"));
        assert_eq!(Int, Type::new("Int"));
        assert_eq!(Bool, Type::new("Bool"));
        assert_eq!(List(Box::new(Int)), Type::new("List<Int>"));
        assert_eq!(Map(Box::new(Int), Box::new(Type::Bool)), Type::new("Map<Int,Bool>"));
        assert_eq!(Option(Box::new(Int)), Type::new("Int?"));
        assert_eq!(Try(Box::new(Int)), Type::new("Int!"));
        assert_eq!(Fun(vec!(Int, Type::Float), Box::new(Type::Float)), Type::new("(Int,Float)->Float"));
        assert_eq!("List<Int>", Type::new("List<Int>").to_string());
    }
}
