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
    AlreadyDefined(String),
    NotDefined(String),
    WrongArgumentsNumber(String, usize,usize),
    UnexpectedArgumentType(String, String)
}

impl Exception {
    // TODO: replace with Display
    pub fn format(&self) -> String { format!("{:?}", self).replace("\"","") }
}

