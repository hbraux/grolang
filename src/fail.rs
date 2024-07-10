use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Fail {
    CannotParse(String),
    NotSymbol(String),
    DivisionByZero,
    UndefinedSymbol(String),
    NotNumber,
    NotBoolean,
    NotMutable(String),
    InconsistentType(String),
    AlreadyDefined(String),
    NotDefined(String),
    WrongArgumentsNumber(usize,usize)
}

