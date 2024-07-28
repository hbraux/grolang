use std::collections::HashMap;
use strum_macros::Display;

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
    WrongArgumentsNumber(String, usize,usize),
    UnexpectedArgumentType(String, String)
}

impl Exception {
    pub fn format(&self, msg: &HashMap<&str, &str>) -> String {
        // let fmt = msg.get(&self.to_string()).or(Some(&"{:?}")).unwrap().to_string();
        format!("{}", self).replace("\"", "")
    }
}

