use std::borrow::ToOwned;
use std::string::ToString;

use strum_macros::Display;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::if_else;

use self::Type::{Unknown, Any, Bool, Float, Fun, Int, List, Map, Option, Str, Try, Class};

#[derive(Debug, Eq, PartialEq, Clone, Display)]
pub enum Type {
    Unknown,
    Any,
    Int,
    Bool,
    Str,
    Float,
    Number,
    List(Box<Type>),
    Option(Box<Type>),
    Try(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Fun(Vec<Type>, Box<Type>),
    Macro,
    Class(String)
}

impl Type {
    pub fn from_str(str: &str) -> Result<Type, Exception> {
        if str.starts_with(":") {
            Type::from_str(&str[1..])
        } else if str.starts_with("(") {
            let args: Vec<&str>  = str[1..str.len()].split(")->").collect();
            args[0].split(",").map(Type::from_str).collect::<Result<Vec<_>, _> >().and_then(
                |vec| Type::from_str(args[1]).map(|o| Fun(vec, Box::new(o)))
            )
        } else if str.ends_with("?") {
            Type::from_str(&str[0..str.len() - 1]).map(|t| Option(Box::new(t)))
        } else if str.ends_with("!") {
            Type::from_str(&str[0..str.len() - 1]).map(|t| Try(Box::new(t)))
        } else if str.starts_with("List<") {
            Type::from_str(&str[5..str.len() - 1]).map(|t| List(Box::new(t)))
        } else if str.starts_with("Map<") {
            let args: Vec<&str> = (&str[4..str.len() - 1]).split(',').collect();
            if args.len() == 2 {
                args.into_iter().map(Type::from_str).collect::<Result<Vec<_>, _>>().and_then(
                    |vec| Ok(Map(Box::new(vec[0].clone()), Box::new(vec[1].clone())))
                )
            } else {  Err(Exception::CannotParse("Map type".to_owned())) }
        } else {
            match str {
                "Any" => Ok(Any),
                "Int" => Ok(Int),
                "Bool" => Ok(Bool),
                "Str" => Ok(Str),
                "Float" => Ok(Float),
                _ => if_else!(str.chars().all(|c| c.is_alphabetic()), Ok(Class(str.to_string())), Err(Exception::CannotParse(str.to_owned())))
            }
        }
    }
    pub fn from_exprs(args: &Vec<Expr>) -> Type {
        if args.is_empty() { Unknown } else {
            let types: Vec<Type> = args.iter().map(Expr::get_type).collect();
            // TODO: match higher type
            types[0].clone()
        }
    }
    pub fn print(&self) -> String {
        match self {
            List(t) => format!("List<{}>", t.print()),
            Map(t, u) => format!("Map<{},{}>", t.print() , u.print()),
            _ => self.to_string()
        }
    }
    pub fn method_name(&self, name: &str) -> String {
        self.to_string().to_owned() + if_else!(name.starts_with("."), "", ".") + name
    }

    pub fn is_defined(&self) -> bool {
        match self {
            Unknown => false,
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
    fn read(str: &str) -> Type { Type::from_str(str).unwrap() }

    #[test]
    fn test_parse() {
        assert_eq!(Any, read("Any"));
        assert_eq!(Int, read("Int"));
        assert_eq!(Bool, read("Bool"));
        assert_eq!(List(Box::new(Int)), read("List<Int>"));
        assert_eq!(Map(Box::new(Int), Box::new(Bool)), read("Map<Int,Bool>"));
        assert_eq!(Option(Box::new(Int)), read("Int?"));
        assert_eq!(Try(Box::new(Int)), read("Int!"));
        assert_eq!(Fun(vec!(Int, Float), Box::new(Float)), read("(Int,Float)->Float"));
    }
    #[test]
    fn test_print() {
        assert_eq!("List<Int>", read("List<Int>").print());
    }
}
