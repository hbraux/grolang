

use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

use strum_macros::{Display, EnumString};

use ErrorCode::{DivisionByZero, InconsistentType, NotANumber, UndefinedSymbol, WrongArgumentsNumber};
use Expr::{Bool, Error, Float, Id, Int, Null, Str};
use crate::ErrorCode::EvalIssue;

use crate::Expr::{Call, Symbol};
use crate::parser::parse;

mod parser;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ErrorCode {
    DivisionByZero,
    UndefinedSymbol(String),
    SyntaxError(String),
    NotANumber,
    InconsistentType(String),
    AlreadyDefined(String),
    WrongArgumentsNumber,
    EvalIssue
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
                _ => Type::Defined(str.to_string()),
            }
        }
    }
}

// ********************************* Built-in Functions ******************************************

#[derive(Debug, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Fun {
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
    Set,
    Var,
    Val,
    Const,
    Defined(String)
}


impl Fun {
    fn new(str: &str) -> Fun {
        match Fun::from_str(&str) {
            Ok(x) => x,
            Err(_x) => Fun::Defined(str.to_string())
        }
    }
    fn call_args(&self) -> usize {
        match self {
            Fun::ToStr => 0,
            Fun::Defined(_s) => 99,
            _ => 1
        }
    }
    fn calc_int(&self, a: &i64, b: &i64) -> Expr {
        match self {
            Fun::Add => Int(a + b),
            Fun::Sub => Int(a - b),
            Fun::Mul => Int(a * b),
            Fun::Mod => Int(a % b),
            Fun::Div => if *b != 0 { Int(a / b) } else { Error(DivisionByZero) }
            _ => panic!(),
        }
    }
    fn calc_float(&self, a: &f64, b: &f64) -> Expr {
        match self {
            Fun::Add => Float(a + b),
            Fun::Sub => Float(a - b),
            Fun::Mul => Float(a * b),
            Fun::Mod => Float(a % b),
            Fun::Div => if *b != 0.0 { Float(a / b) } else { Error(DivisionByZero) }
            _ => panic!(),
        }
    }
}

// *********************************** Expr ******************************************

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Id(String),
    Symbol(String),
    TypeSpec(String),
    FunOperator(Fun),
    Call(Box<Expr>, Box<Expr>, Vec<Expr>),
    Error(ErrorCode),
    Null,
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NULL: Expr = Null;

impl Expr {
    pub fn read(str: &str, _ctx: &Context) -> Expr { parse(str) }
    // recursive format with debug
    pub fn format(&self) -> String { format!("{:?}", self) }

    pub fn get_type(&self) -> Type {
        match self {
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            Str(_) => Type::Str,
            _ => Type::Any,
        }
    }

    pub fn eval(self, ctx: &mut Context) -> Expr {
        match self {
            Symbol(name) => Id(name),
            Id(name) => ctx.get(&*name),
            Int(_) | Float(_) | Str(_) | Bool(_) => self.clone(),
            Call(left, op, args) => if let Id(name) = *op {
                left.eval(ctx).call(ctx, Fun::new(&name), args.into_iter().map(|e| e.eval(ctx)).collect())
            } else {
                Error(EvalIssue)
            }
            _ => panic!()
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
    // private part
    fn store(self, ctx: &mut Context, name: String, is_new: bool) -> Expr {
        if is_new && ctx.is_defined(&name) {
            Error(ErrorCode::AlreadyDefined(name.to_owned()))
        } else if !is_new && ctx.get_type(&name) != self.get_type() {
            Error(InconsistentType(self.get_type().to_string()))
        } else {
            ctx.set(&name, self.clone());
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
    fn is_error(&self) -> bool { matches!(self, Error(_)) }
    fn unitary_op(self, code: Fun) -> Expr {
        match code {
            Fun::ToStr => Str(self.print()),
            _ => panic!(),
        }
    }

    fn arithmetic_op(&self, fun: Fun, right: &Expr) -> Expr {
        match (self, right) {
            (Int(a), Int(b))    =>  fun.calc_int(a, b),
            (Float(a), Float(b)) => fun.calc_float(a, b),
            (Int(a), Float(b))  => fun.calc_float(&(*a as f64), b),
            (Float(a), Int(b))  => fun.calc_float(a, &(*b as f64)),
            _ =>  Error(NotANumber),
        }
    }
    fn comparison_op(&self, code: Fun, right: &Expr) -> Expr {
        let result = match code {
            Fun::Eq => self.eq(right),
            Fun::Neq => !self.eq(right),
            _ => panic!("no yet implemented"),
        };
        Bool(result)
    }
    fn call(self, ctx: &mut Context, fun: Fun, args: Vec<Expr>) -> Expr {
        if fun.call_args() != args.len() {
            Error(WrongArgumentsNumber)
        } else {
            match fun {
                Fun::Defined(_x) => todo!(),
                Fun::ToStr => self.unitary_op(fun),
                Fun::Add | Fun::Sub | Fun::Mul | Fun::Div | Fun::Mod => self.arithmetic_op(fun, &args[0]),
                Fun::Eq | Fun::Neq | Fun::Ge | Fun::Gt | Fun::Le | Fun::Lt => self.comparison_op(fun, &args[0]),
                _ => panic!(),
            }
        }
    }
}

// *********************************** Context ******************************************

pub struct Context {
    values: HashMap<String, Expr>,
}

impl Context {
    pub fn new() -> Context { Context { values: HashMap::new() } }
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
    pub fn exec(&mut self, str: &str) -> String {
        self.read(str).eval(self).print()
    }
}

// *********************************** TESTS ******************************************

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_value_type() {
        assert_eq!(Type::Any, Type::new("Any"));
        assert_eq!(Type::Int, Type::new("Int"));
        assert_eq!(Type::List(Box::new(Type::Int)), Type::new("List<Int>"));
        assert_eq!(Type::Map(Box::new(Type::Int), Box::new(Type::Bool)), Type::new("Map<Int,Bool>"));
        assert_eq!(Type::Option(Box::new(Type::Int)), Type::new("Int?"));
        assert_eq!(Type::Fail(Box::new(Type::Int)), Type::new("Int!"));
    }

    #[test]
    fn test_fun() {
        assert_eq!(Fun::Eq, Fun::new("eq"));
        assert_eq!(Fun::Defined("other".to_string()), Fun::new("other"));
    }

    #[test]
    fn test_literals() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.exec("1"));
        assert_eq!("9123456", ctx.exec("9_123_456"));
        assert_eq!("2.0", ctx.exec("2."));
        assert_eq!("-1.23", ctx.exec("-1.23"));
        assert_eq!("23000.0", ctx.exec("2.3e4"));
        assert_eq!("false", ctx.exec("false"));
        assert_eq!("true", ctx.exec("true"));
        assert_eq!("null", ctx.exec("null"));
        assert_eq!("\"abc\"", ctx.exec("\"abc\""));
    }

    #[test]
    fn test_variables() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.exec("var a = 1"));
        assert_eq!("Error(AlreadyDefined(\"a\"))", ctx.exec("var a = 3"));
        assert_eq!("3", ctx.exec("a = 3"));
        assert_eq!("Error(InconsistentType(\"Float\"))", ctx.exec("a = 3.0"));
        assert_eq!("2", ctx.exec("var b: Int = 2"));
        assert_eq!("3.2", ctx.exec("var c=3.2"));
        assert_eq!("Error(InconsistentType(\"Int\"))", ctx.exec("var d: Int =3.2"));
        assert_eq!("3", ctx.exec("a"));
        assert_eq!("2", ctx.exec("b"));
    }

    #[test]
    fn test_arithmetics() {
        let mut ctx = Context::new();
        assert_eq!("14", ctx.exec("2 + 3 * 4"));
        assert_eq!("20", ctx.exec("(2 + 3) * 4"));
        assert_eq!("4", ctx.exec("4 / 1"));
        assert_eq!("2", ctx.exec("22%10"));
        assert_eq!("2", ctx.exec("-2 * -1"));
        assert_eq!("3.3", ctx.exec("1 + 2.3"));
        assert_eq!("Error(DivisionByZero)", ctx.exec("1 / 0"));
        assert_eq!("3", ctx.exec("2.add(1)"));
        assert_eq!("6", ctx.exec("2.mul(3)"));
    }

    #[test]
    fn test_comparisons() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.exec("var a = 1"));
        assert_eq!("2", ctx.exec("var b = 2"));
        assert_eq!("true", ctx.exec("a == a"));
        assert_eq!("true", ctx.exec("a == 1"));
        assert_eq!("true", ctx.exec("1 == a"));
        assert_eq!("false", ctx.exec("a == b"));
        assert_eq!("true", ctx.exec("a != b"));
    }
}
