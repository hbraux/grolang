#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use crate::Expr::{Assign, Call};
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::{Display, EnumString};
use ErrorType::{CannotParse, DivisionByZero, NotANumber, UndefinedSymbol,WrongArgumentsNumber, InconsistentType};
use Expr::{BinaryOp, Bool, Declare, Error, Float, Id, Int, Null, Str};

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ErrorType {
    DivisionByZero,
    UndefinedSymbol(String),
    CannotParse(String),
    NotANumber,
    InconsistentType(String),
    AlreadyDefined(String),
    WrongArgumentsNumber
}

// *********************************** Type ******************************************

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
    Fail(Box<Type>),
    Map(Box<Type>, Box<Type>),
}

impl Type {
    pub fn new(str: &str) -> Type {
        if str.ends_with("?") {
            Type::Option(Box::new(Type::new(&str[0..str.len() - 1])))
        } else if str.ends_with("!") {
            Type::Fail(Box::new(Type::new(&str[0..str.len() - 1])))
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
                _ => {
                    if str.chars().all(|x| x.is_alphabetic()) {
                        Type::Defined(str.to_string())
                    } else {
                        panic!("{} is not a valid type name", str)
                    }
                }
            }
        }
    }
}

// *********************************** OpCode ******************************************

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum Code {
    Mul,
    Div,
    Add,
    Sub,
    Mod,
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
    In,
    ToStr,
    Defined(String)
}

impl Code {
    fn new(str: &str) -> Code {
        let camel = format!("{}{}", (&str[..1].to_string()).to_uppercase(), &str[1..]);
        match Code::from_str(&camel) {
            Ok(code) => code,
            Err(_x) => Code::Defined(str.to_string())
        }
    }
    fn call_args(&self) -> usize {
        match self {
            Code::ToStr => 0,
            Code::Defined(_s) => 99,
            _ => 1
        }
    }
    fn calc_int(&self, a: &i64, b: &i64) -> Expr {
        match self {
            Code::Add => Int(a + b),
            Code::Sub => Int(a - b),
            Code::Mul => Int(a * b),
            Code::Mod => Int(a % b),
            Code::Div => {
                if *b != 0 {
                    Int(a / b)
                } else {
                    Error(DivisionByZero)
                }
            }
            _ => panic!(),
        }
    }
    fn calc_float(&self, a: &f64, b: &f64) -> Expr {
        match self {
            Code::Add => Float(a + b),
            Code::Sub => Float(a - b),
            Code::Mul => Float(a * b),
            Code::Mod => Float(a % b),
            Code::Div => {
                if *b != 0.0 {
                    Float(a / b)
                } else {
                    Error(DivisionByZero)
                }
            }
            _ => panic!(),
        }
    }
}

// *********************************** Expr ******************************************

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Id(String),
    Declare(String, Option<Type>, Box<Expr>),
    Assign(String, Box<Expr>),
    BinaryOp(Box<Expr>, Code, Box<Expr>),
    Call(Box<Expr>, String, Vec<Box<Expr>>),
    Error(ErrorType),
    Null,
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NULL: Expr = Null;

impl Expr {
    //noinspection ALL
    pub fn read(str: &str, _ctx: &Context) -> Expr {
        match grammar::StatementParser::new().parse(str) {
            Ok(expr) => *expr,
            Err(e) => Error(CannotParse(e.to_string())),
        }
    }
    pub fn get_type(&self) -> Type {
        match self {
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            Str(_) => Type::Str,
            _ => Type::Any,
        }
    }
    fn is_error(&self) -> bool {
        matches!(self, Error(_))
    }
    fn store(self, name: &str, ctx: &mut Context, is_new: bool) -> Expr {
        if is_new && ctx.is_defined(&name) {
            Error(ErrorType::AlreadyDefined(name.to_owned()))
        } else if !is_new && ctx.get_type(name) != self.get_type() {
            Error(InconsistentType(self.get_type().to_string()))
        } else {
            ctx.set(name, self.clone());
            self
        }
    }
    fn ensure(self, spec: Option<Type>) -> Expr {
        if let Some(expected) = spec {
            if !self.is_error() && self.get_type() != expected {
                return Error(InconsistentType(expected.to_string()));
            }
        }
        self
    }
    pub fn eval(self, ctx: &mut Context) -> Expr {
        match self {
            Id(name) => ctx.get(&*name),
            Assign(name, value) => value.eval(ctx).store(&name, ctx, false),
            Declare(name, spec, value) => value.eval(ctx).ensure(spec).store(&name, ctx, true),
            BinaryOp(left, code, right) => left.eval(ctx).binary_op(code, right.eval(ctx)),
            Call(left, name, args) => left.eval(ctx)
                .method_call(&name, args.into_iter().map(|e| e.eval(ctx)).collect()),
            _ => self.clone(),
        }
    }
    pub fn print(self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Null => "null".to_string(),
            Float(x) => {
                let str = x.to_string();
                if str.contains('.') { str } else { format!("{}.0", str) }
            }
            _ => format!("{:?}", self),
        }
    }
    fn unitary_op(self, code: Code) -> Expr {
        match code {
            Code::ToStr => Str(self.print()),
            _ => panic!(),
        }
    }
    fn binary_op(&self, code: Code, right: Expr) -> Expr {
        match code {
            Code::Add | Code::Sub | Code::Mul | Code::Div | Code::Mod => self.arithmetic_op(code, &right),
            Code::Eq | Code::Neq | Code::Ge | Code::Gt | Code::Le | Code::Lt => self.comparison_op(code, &right),
            _ => panic!(),
        }
    }
    fn arithmetic_op(&self, code: Code, right: &Expr) -> Expr {
        match (self, right) {
            (Int(a), Int(b))    =>  code.calc_int(a, b),
            (Float(a), Float(b)) => code.calc_float(a, b),
            (Int(a), Float(b))  => code.calc_float(&(*a as f64), b),
            (Float(a), Int(b))  => code.calc_float(a, &(*b as f64)),
            _ =>  Error(NotANumber),
        }
    }
    fn comparison_op(&self, code: Code, right: &Expr) -> Expr {
        let result = match code {
            Code::Eq => self.eq(right),
            Code::Neq => !self.eq(right),
            _ => panic!("no yet implemented"),
        };
        Bool(result)
    }
    fn method_call(self, name: &str, args: Vec<Expr>) -> Expr {
        let code = Code::new(name);
        if code.call_args() != args.len() {
            Error(WrongArgumentsNumber)
        } else {
            match code {
                Code::Defined(_x) => todo!(),
                Code::ToStr => self.unitary_op(code),
                _ => self.binary_op(code, args[0].clone())
            }
        }
    }
}

// *********************************** Context ******************************************

pub struct Context {
    values: HashMap<String, Expr>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Expr {
        match self.values.get(name) {
            Some(expr) => expr.clone(),
            None => Error(UndefinedSymbol(name.to_string())),
        }
    }
    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    pub fn get_type(&self, name: &str) -> Type {
        self.values.get(name).unwrap().get_type()
    }
    pub fn set(&mut self, name: &str, expr: Expr) {
        self.values.insert(name.to_string(), expr);
    }
    pub fn read(&mut self, str: &str) -> Expr {
        Expr::read(str, self)
    }
    pub fn eval(&mut self, str: &str) -> String {
        self.read(str).eval(self).print()
    }
}

// *********************************** TESTS ******************************************

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
        assert_eq!(Type::Fail(Box::new(Type::Int)), Type::new("Int!"));
    }

    #[test]
    fn test_codes() {
        assert_eq!(Code::Eq, Code::new("eq"));
        assert_eq!(Code::Defined("other".to_string()), Code::new("other"));
    }

    #[test]
    fn test_literals() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.eval("1"));
        assert_eq!("9123456", ctx.eval("9_123_456"));
        assert_eq!("2.0", ctx.eval("2."));
        assert_eq!("-1.23", ctx.eval("-1.23"));
        assert_eq!("23000.0", ctx.eval("2.3e4"));
        assert_eq!("false", ctx.eval("false"));
        assert_eq!("true", ctx.eval("true"));
        assert_eq!("null", ctx.eval("null"));
        assert_eq!("\"abc\"", ctx.eval("\"abc\""));
    }

    #[test]
    fn test_variables() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.eval("var a = 1"));
        assert_eq!("Error(AlreadyDefined(\"a\"))", ctx.eval("var a = 3"));
        assert_eq!("3", ctx.eval("a = 3"));
        assert_eq!("Error(InconsistentType(\"Float\"))", ctx.eval("a = 3.0"));
        assert_eq!("2", ctx.eval("var b: Int = 2"));
        assert_eq!("3.2", ctx.eval("var c=3.2"));
        assert_eq!("Error(InconsistentType(\"Int\"))", ctx.eval("var d: Int =3.2"));
        assert_eq!("3", ctx.eval("a"));
        assert_eq!("2", ctx.eval("b"));
    }

    #[test]
    fn test_arithmetics() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.eval("var a = 1"));
        assert_eq!("2", ctx.eval("var b = 2"));
        assert_eq!("14", ctx.eval("2 + 3 * 4"));
        assert_eq!("20", ctx.eval("(2 + 3) * 4"));
        assert_eq!("4", ctx.eval("4 / 1"));
        assert_eq!("2", ctx.eval("22%10"));
        assert_eq!("2", ctx.eval("-2 * -1"));
        assert_eq!("3.3", ctx.eval("1 + 2.3"));
        assert_eq!("5", ctx.eval("4 + a"));
        assert_eq!("2", ctx.eval("b / a"));
        assert_eq!("Error(DivisionByZero)", ctx.eval("1 / 0"));
        assert_eq!("3", ctx.eval("a.add(b)"));
        assert_eq!("6", ctx.eval("b.mul(3)"));
    }

    #[test]
    fn test_comparisons() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.eval("var a = 1"));
        assert_eq!("2", ctx.eval("var b = 2"));
        assert_eq!("true", ctx.eval("a == a"));
        assert_eq!("true", ctx.eval("a == 1"));
        assert_eq!("true", ctx.eval("1 == a"));
        assert_eq!("false", ctx.eval("a == b"));
    }
}
