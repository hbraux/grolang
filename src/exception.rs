use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Exception {
    CannotParse(String),
    NotImplemented(String),
    NotSymbol(String),
    DivisionByZero,
    UndefinedSymbol(String),
    UndefinedFunction(String),
    InfiniteLoop,
    NotNumber,
    NotInt(String),
    NotFloat(String),
    NotBoolean(String),
    NotMutable(String),
    UnexpectedType(String),
    AlreadyDefined(String),
    NotDefined(String),
    WrongArgumentsNumber(String, usize,usize),
    UnexpectedInputTypes(String, String),
    UnexpectedOutputType(String, String)
}

impl Exception {
    // TODO: replace with Display
    pub fn format(&self) -> String { format!("{:?}", self).replace("\"","") }
}

