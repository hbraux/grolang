use std::borrow::ToOwned;
use std::string::ToString;

use strum_macros::Display;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::if_else;
use crate::types::Type::_Undefined;

use self::Type::{Any, Bool, Float, Fun, Int, List, Map, Option, Str, Try, Struct, Macro, Number};

#[derive(Debug, Eq, PartialEq, Clone, Display)]
pub enum Type {
    _Undefined,
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
    Struct(String)
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
                "Number" => Ok(Number),
                "Macro" => Ok(Macro),
                _ => if_else!(str.chars().all(|c| c.is_alphabetic()), Ok(Struct(str.to_string())), Err(Exception::CannotParse(str.to_owned())))
            }
        }
    }
    // same derived from strum
    pub fn name(&self) -> String { self.to_string() }

    pub fn is_number(&self) -> bool {
        *self == Int || *self == Float || *self == Number
    }

    pub fn is_defined(&self) -> bool { *self != _Undefined }

    pub fn matches(&self, expected: &Type) -> bool {
        *expected == Any || *self == *expected || (*expected == Number && self.is_number())
    }

    pub fn infer_list(vec: &Vec<Expr>) -> Type {
        List(Box::new(infer(vec).clone()))
    }
    pub fn infer_map(vec: &Vec<(Expr, Expr)>) -> Type {
        Map(Box::new(infer(&vec.iter().map(|p| p.0.clone()).collect::<Vec<_>>()).clone()), Box::new(infer(&vec.iter().map(|p| p.1.clone()).collect::<Vec<_>>()).clone()))
    }

    pub fn print(&self) -> String {
        match self {
            List(t) => format!("List<{}>", t.print()),
            Map(t, u) => format!("Map<{},{}>", t.print() , u.print()),
            _ => self.name()
        }
    }
    pub fn method_name(&self, name: &str) -> String {
        self.name().to_owned() + if_else!(name.starts_with("."), "", ".") + name
    }
    pub fn all_method_names(&self, name: &str) -> Vec<String> {
        let mut vec = vec!(self.method_name(name));
        if self.is_number() {
            vec.push(Number.method_name(name));
        }
        vec.push(Any.method_name(name));
        vec
    }
}

fn infer(vec: &Vec<Expr>) -> &Type {
    if vec.is_empty() { &Any } else {
        let mut current = vec[0].get_type();
        for e in vec[1..].iter() {
            let other = e.get_type();
            if *current != *other && *current != Any {
                current = if_else!(current.is_number() && other.is_number(), &Number, &Any)
            }
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::TRUE;
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
        assert_eq!(Struct("Point".to_owned()), read("Point"));
        assert_eq!(Err(Exception::CannotParse("Poi!nt".to_string())), Type::from_str("Poi!nt"));
    }

    #[test]
    fn test_print() {
        let t = List(Box::new(Int));
        assert_eq!("List<Int>", t.print());
        assert_eq!("List", t.to_string());
    }

    #[test]
    fn test_infer() {
        assert_eq!(&Any, infer(&vec!()));
        assert_eq!(&Int, infer(&vec!(Expr::Int(1), Expr::Int(2))));
        assert_eq!(&Number, infer(&vec!(Expr::Int(1), Expr::Float(2.0))));
        assert_eq!(&Any, infer(&vec!(Expr::Int(1), TRUE)));
    }
}
