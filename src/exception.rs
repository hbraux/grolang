use std::collections::HashMap;
use strum_macros::Display;

use self::Exception::{CannotParse, UndefinedFunction, UndefinedMethod, UndefinedSymbol, NotDefined, NotMutable, UnexpectedType, CannotInferType, CannotCastType, AlreadyDefined, NotA, UnexpectedArgumentType, WrongArgumentsNumber};

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Exception {
    CannotParse(String),
    DivisionByZero,
    IOError,
    UndefinedSymbol(String),
    UndefinedFunction(String),
    UndefinedMethod(String),
    InfiniteLoop,
    NotA(String, String),
    NotMutable(String),
    UnexpectedType(String),
    CannotInferType(String),
    CannotCastType(String),
    AlreadyDefined(String),
    NotDefined(String),
    WrongArgumentsNumber(String, String ,String),
    UnexpectedArgumentType(String, String)
}

impl Exception {
    // same derived from strum
    pub fn name(&self) -> String { self.to_string() }

    // warning, print rely on debug string
    pub fn print(&self) -> String { format!("{:?}", self).replace("\"","") }

    pub fn format(&self, messages: &HashMap<&str, &str>) -> String {
        if let Some(msg) = messages.get(self.to_string().as_str()) {
            format!("{}", match self {
                CannotParse(x) |
                UndefinedSymbol(x) |
                UndefinedFunction(x) |
                UndefinedMethod(x) |
                NotDefined(x) |
                NotMutable(x) |
                UnexpectedType(x) |
                CannotInferType(x) |
                CannotCastType(x) |
                AlreadyDefined(x) => msg.replace("{1}",x),
                NotA(x, y) |
                UnexpectedArgumentType(x, y) => msg.replace("{1}",x).replace("{2}",y),
                WrongArgumentsNumber(x, y, z) => msg.replace("{1}",x).replace("{2}",y).replace("{3}",z),
                _ => msg.to_string(),
            })
        } else {
            self.name()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("UndefinedSymbol", UndefinedSymbol("a".to_owned()).name());
        assert_eq!("UndefinedSymbol(a)", UndefinedSymbol("a".to_owned()).print());

        let messages = HashMap::from([("UndefinedSymbol", "the symbol '{1}' is not defined")]);
        assert_eq!("the symbol 'a' is not defined", UndefinedSymbol("a".to_owned()).format(&messages));
    }
}
