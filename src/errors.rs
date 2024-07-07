use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ErrorCode {
    ParseError(String),
    NotSymbol(String),
    DivisionByZero,
    UndefinedSymbol(String),
    NotNumber,
    NotBoolean,
    InconsistentType(String),
    AlreadyDefined(String),
    NotDefined(String),
    WrongArgumentsNumber(usize,usize),
    EvalIssue
}
