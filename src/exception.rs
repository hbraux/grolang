use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Exception {
    CannotParse(String),
    NotImplemented(String),
    NotSymbol(String),
    DivisionByZero,
    UndefinedSymbol(String),
    NotNumber,
    NotBoolean(String),
    NotMutable(String),
    InconsistentType(String),
    AlreadyDefined(String),
    NotDefined(String),
    WrongArgumentsNumber(usize,usize)
}

