use strum_macros::Display;

#[derive(Debug, Eq, PartialEq, Clone, Display)]
pub enum Type {
    Any,
    Int,
    Bool,
    Str,
    Float,
    Defined(String),
    List(Box<Type>),
    Option(Box<Type>),
    Try(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Lambda(Box<Type>, Vec<Type>)
}

impl Type {
    pub fn new(str: &str) -> Type {
        if str.ends_with("?") {
            Type::Option(Box::new(Type::new(&str[0..str.len() - 1])))
        } else if str.ends_with("!") {
            Type::Try(Box::new(Type::new(&str[0..str.len() - 1])))
        } else if str.starts_with("List<") {
            Type::List(Box::new(Type::new(&str[5..str.len() - 1])))
        } else if str.starts_with("List<") {
            Type::List(Box::new(Type::new(&str[5..str.len() - 1])))
        } else if str.starts_with("Map<") {
            let s: Vec<&str> = (&str[4..str.len() - 1]).split(',').collect();
            Type::Map(Box::new(Type::new(s[0])), Box::new(Type::new(s[1])))
        } else {
            match str {
                "Any" => Type::Any,
                "Int" => Type::Int,
                "Bool" => Type::Bool,
                "Str" => Type::Str,
                "Float" => Type::Float,
                _ => Type::Defined(str.to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types() {
        assert_eq!(Type::Any, Type::new("Any"));
        assert_eq!(Type::Int, Type::new("Int"));
        assert_eq!(Type::List(Box::new(Type::Int)), Type::new("List<Int>"));
        assert_eq!(Type::Map(Box::new(Type::Int), Box::new(Type::Bool)), Type::new("Map<Int,Bool>"));
        assert_eq!(Type::Option(Box::new(Type::Int)), Type::new("Int?"));
        assert_eq!(Type::Try(Box::new(Type::Int)), Type::new("Int!"));
    }
}
