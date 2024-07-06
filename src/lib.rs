use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

use strum_macros::{Display, EnumString};

use ErrorCode::{DivisionByZero, InconsistentType, UndefinedSymbol, WrongArgumentsNumber};
use Expr::{Bool, Error, Float, Int, Nil, Str};

use crate::ErrorCode::NotABoolean;
use crate::Expr::{Call, Symbol, TypeSpec};
use crate::parser::parse;

mod parser;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ErrorCode {
    DivisionByZero,
    UndefinedSymbol(String),
    SyntaxError(String),
    NotANumber,
    NotABoolean,
    InconsistentType(String),
    AlreadyDefined(String),
    NotDefined(String),
    WrongArgumentsNumber(usize,usize),
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
pub enum Operator {
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
    Or,
    And,
    ToStr,
    Set,
    IfElse,
    DefVar,
    DefVal,
    DefConst,
    Defined(String)
}


impl Operator {
    fn new(str: &str) -> Operator {
        match Operator::from_str(&str) {
            Ok(x) => x,
            Err(_x) => Operator::Defined(str.to_string())
        }
    }
    fn is_lazy(&self) -> bool { // lazy operators evalaate their arguments when needed
        match self {
            Operator::IfElse |  Operator::And |  Operator::Or => true,
            _ => false,
        }
    }
    fn call_args(&self) -> usize {
        match self {
            Operator::ToStr => 0,
            Operator::Defined(_s) => 99,
            _ => 1
        }
    }
    fn calc_int(&self, a: i64, b: i64) -> Expr {
        match self {
            Operator::Add => Int(a + b),
            Operator::Sub => Int(a - b),
            Operator::Mul => Int(a * b),
            Operator::Mod => Int(a % b),
            Operator::Div => if b != 0 { Int(a / b) } else { Error(DivisionByZero) }
            _ => panic!(),
        }
    }
    fn calc_float(&self, a: f64, b: f64) -> Expr {
        match self {
            Operator::Add => Float(a + b),
            Operator::Sub => Float(a - b),
            Operator::Mul => Float(a * b),
            Operator::Mod => Float(a % b),
            Operator::Div => if b != 0.0 { Float(a / b) } else { Error(DivisionByZero) }
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
    Symbol(String),
    TypeSpec(Type),
    Block(Vec<Expr>),
    ChainCall(Vec<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Error(ErrorCode),
    Nil,
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NIL: Expr = Nil;

impl Expr {
    pub fn read(str: &str, _ctx: &Context) -> Expr { parse(str) }

    pub fn parse_type_spec(str: &str) -> Expr {
        TypeSpec(Type::new(str.replace(":", "").trim()))
    }

    // recursive format with debug
    pub fn format(&self) -> String { format!("{:?}", self).replace("\"","") }

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
            Nil => Nil,
            Symbol(name) => ctx.get(&*name),
            Int(_) | Float(_) | Str(_) | Bool(_) | TypeSpec(_) => self,
            Call(op, mut args) => args.remove(0).eval(ctx).call(op.to_operator(), args, ctx),
            _ => panic!("{}.eval() not implemented", self)
        }
    }

    pub fn print(self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Nil => "nil".to_string(),
            Float(x) => {
                let str = x.to_string();
                if str.contains('.') { str } else { format!("{}.0", str) }
            }
            Symbol(x) => x,
            _ => format!("{:?}", self),
        }
    }
    // private part
    fn to_operator(&self) -> Operator {
        match self {
            Symbol(name) => Operator::new(name),
            _ => panic!("{} is not an Id", self)
        }
    }
    fn store(self, ctx: &mut Context, args: Vec<Expr>, _fun: Operator, is_new: bool) -> Expr {
        let mut value= &args[0];
        if let TypeSpec(expected) = value {
            value = &args[1];
            if value.get_type() != *expected {
                return Error(InconsistentType(value.get_type().to_string()))
            }
        }
        // TODO: handle variable type
        if let Symbol(name) = self {
            let is_defined = ctx.is_defined(&name);
            if is_new && is_defined {
                Error(ErrorCode::AlreadyDefined(name.to_owned()))
            } else if !is_new && !is_defined {
                Error(ErrorCode::NotDefined(name.to_owned()))
            } else if !is_new && ctx.get_type(&name) != value.get_type() {
                Error(InconsistentType(value.get_type().to_string()))
            } else {
                ctx.set(&name, value.clone());
                value.clone()
            }
        } else {
            panic!("{} is not an id", self)
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
    fn unitary_op(self, code: Operator) -> Expr {
        match code {
            Operator::ToStr => Str(self.print()),
            _ => panic!(),
        }
    }

    fn arithmetic_op(&self, op: Operator, right: Expr) -> Expr {
        match (self, right) {
            (Int(a), Int(b))    =>  op.calc_int(*a, b),
            (Float(a), Float(b)) => op.calc_float(*a, b),
            (Int(a), Float(b))  => op.calc_float(*a as f64, b),
            (Float(a), Int(b))  => op.calc_float(*a, b as f64),
            _ => panic!(),
        }
    }
    fn comparison_op(&self, op: Operator, right: Expr) -> Expr {
        let result = match op {
            Operator::Eq => self.eq(&right),
            Operator::Neq => !self.eq(&right),
            _ => panic!("no yet implemented"),
        };
        Bool(result)
    }
    fn binary_op(&self, op: Operator, right: &Expr, ctx: &mut Context) -> Expr {
        match (op, self) {
            (Operator::And, &FALSE) => FALSE,
            (Operator::Or, &TRUE) => TRUE,
            (Operator::And, &TRUE) => right.clone().eval(ctx).to_bool(),
            (Operator::Or, &FALSE) => right.clone().eval(ctx).to_bool(),
            _ => panic!("not boolean"),
        }
    }
    fn to_bool(self) -> Expr {
       match self {
           Bool(_) => self,
           _ => Error(NotABoolean)
       }
    }

    fn call(self, op: Operator, args: Vec<Expr>, ctx: &mut Context) -> Expr {
        if args.len() < op.call_args() {
            return Error(WrongArgumentsNumber(op.call_args(), args.len()))
        }
        match op {
            Operator::Defined(_x) => todo!(),
            Operator::ToStr => self.unitary_op(op),
            Operator::Add | Operator::Sub | Operator::Mul | Operator::Div | Operator::Mod => self.arithmetic_op(op, args[0].clone().eval(ctx)),
            Operator::Eq | Operator::Neq | Operator::Ge | Operator::Gt | Operator::Le | Operator::Lt => self.comparison_op(op, args[0].clone().eval(ctx)),
            Operator::And | Operator::Or => self.binary_op(op, &args[0], ctx),
            Operator::DefVar | Operator::DefVal | Operator::DefConst => self.store(ctx, args, op, true),
            Operator::Set => self.store(ctx, args, op, false),
            _ => panic!(),
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
    fn test_types() {
        assert_eq!(Type::Any, Type::new("Any"));
        assert_eq!(Type::Int, Type::new("Int"));
        assert_eq!(Type::List(Box::new(Type::Int)), Type::new("List<Int>"));
        assert_eq!(Type::Map(Box::new(Type::Int), Box::new(Type::Bool)), Type::new("Map<Int,Bool>"));
        assert_eq!(Type::Option(Box::new(Type::Int)), Type::new("Int?"));
        assert_eq!(Type::Fail(Box::new(Type::Int)), Type::new("Int!"));
    }

    #[test]
    fn test_fun() {
        assert_eq!(Operator::Eq, Operator::new("eq"));
        assert_eq!(Operator::Defined("other".to_string()), Operator::new("other"));
    }

    #[test]
    fn test_literals() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.exec("1"));
        assert_eq!("9123456", ctx.exec("9_123_456"));
        assert_eq!("2.0", ctx.exec("2.0"));
        assert_eq!("-1.23", ctx.exec("-1.23"));
        assert_eq!("23000.0", ctx.exec("2.3e4"));
        assert_eq!("false", ctx.exec("false"));
        assert_eq!("true", ctx.exec("true"));
        assert_eq!("nil", ctx.exec("nil"));
        assert_eq!("\"abc\"", ctx.exec("\"abc\""));
        assert_eq!("x", ctx.exec("'x"));
    }

    #[test]
    fn test_variables() {
        let mut ctx = Context::new();
        assert_eq!("Error(NotDefined(\"a\"))", ctx.exec("a = 0"));
        assert_eq!("1", ctx.exec("var a = 1"));
        assert_eq!("true", ctx.exec("'z.defval(true)"));
        assert_eq!("Error(AlreadyDefined(\"a\"))", ctx.exec("var a = 3"));
        assert_eq!("3", ctx.exec("a = 3"));
        assert_eq!("0", ctx.exec("'a.set(0)"));
        assert_eq!("Error(InconsistentType(\"Float\"))", ctx.exec("a = 3.0"));
        assert_eq!("3.2", ctx.exec("val c=3.2"));
        assert_eq!("Error(InconsistentType(\"Float\"))", ctx.exec("var d: Int = 3.2"));
        assert_eq!("0", ctx.exec("a"));
        assert_eq!("3.2", ctx.exec("c"));
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
        assert_eq!("8", ctx.exec("2.mul(3.add(1))"));
    }

    #[test]
    fn test_comparisons() {
        let mut ctx = Context::new();
        assert_eq!("1", ctx.exec("var a = 1"));
        assert_eq!("2", ctx.exec("var b = 2"));
        assert_eq!("true", ctx.exec("a == a"));
        assert_eq!("true", ctx.exec("1 == a"));
        assert_eq!("false", ctx.exec("a == b"));
        assert_eq!("true", ctx.exec("a != b"));
        assert_eq!("true", ctx.exec("a == 1 && b == 2"));
        assert_eq!("false", ctx.exec("a == 1 && b == 1"));
    }

    #[test]
    fn test_ifelse() {
        let mut ctx = Context::new();
        assert_eq!("14", ctx.exec("if (true) { 1 } else { 0 }"))
    }
}
