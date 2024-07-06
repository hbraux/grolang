use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

use strum_macros::{Display, EnumString};


use Expr::{Bool, Error, Float, Int, Nil, Str};

use crate::Expr::{Call, Symbol, TypeSpec};
use crate::parser::parse;
use crate::types::Type;

mod parser;
mod types;


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


// ********************************* Built-in Operators ******************************************

#[derive(Debug, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum BuiltIn {
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
    If,
    Var,
    Val
}

impl BuiltIn {
    fn is_macro(&self) -> bool {
        match self {
            BuiltIn::If |  BuiltIn::And | BuiltIn::Or => true,
            _ => false,
        }
    }
    fn call_args(&self) -> usize {
        match self {
            BuiltIn::ToStr => 1,
            _ => 2
        }
    }
    fn calc_int(&self, a: i64, b: i64) -> Expr {
        match self {
            BuiltIn::Add => Int(a + b),
            BuiltIn::Sub => Int(a - b),
            BuiltIn::Mul => Int(a * b),
            BuiltIn::Mod => Int(a % b),
            BuiltIn::Div => if b != 0 { Int(a / b) } else { Error(ErrorCode::DivisionByZero) }
            _ => panic!(),
        }
    }
    fn calc_float(&self, a: f64, b: f64) -> Expr {
        match self {
            BuiltIn::Add => Float(a + b),
            BuiltIn::Sub => Float(a - b),
            BuiltIn::Mul => Float(a * b),
            BuiltIn::Mod => Float(a % b),
            BuiltIn::Div => if b != 0.0 { Float(a / b) } else { Error(ErrorCode::DivisionByZero) }
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
    Call(Vec<Expr>),
    Error(ErrorCode),
    Nil,
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NIL: Expr = Nil;

impl Expr {
    pub fn read(str: &str, _ctx: &Context) -> Expr {
        parse(str).unwrap_or_else(|s| Error(ErrorCode::ParseError(s)))
    }
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
            Nil | Error(_) | Int(_) | Float(_) | Str(_) | Bool(_) | TypeSpec(_) => self,
            Symbol(name) => ctx.get(&*name),
            Call(mut args) => match args.remove(0) {
                Symbol(name) => call(&name, args, ctx),
                e => Error(ErrorCode::NotSymbol(e.to_string()))
            }
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
    fn store(self, ctx: &mut Context, args: Vec<Expr>, _fun: BuiltIn, is_new: bool) -> Expr {
        let mut value= &args[0];
        if let TypeSpec(expected) = value {
            value = &args[1];
            if value.get_type() != *expected {
                return Error(ErrorCode::InconsistentType(value.get_type().to_string()))
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
                Error(ErrorCode::InconsistentType(value.get_type().to_string()))
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
                return Error(ErrorCode::InconsistentType(expected.to_string()));
            }
        }
        self
    }
    fn is_error(&self) -> bool { matches!(self, Error(_)) }
    fn unitary_op(self, code: BuiltIn) -> Expr {
        match code {
            BuiltIn::ToStr => Str(self.print()),
            _ => panic!(),
        }
    }

    fn arithmetic_op(&self, op: BuiltIn, right: Expr) -> Expr {
        match (self, right) {
            (Int(a), Int(b))    =>  op.calc_int(*a, b),
            (Float(a), Float(b)) => op.calc_float(*a, b),
            (Int(a), Float(b))  => op.calc_float(*a as f64, b),
            (Float(a), Int(b))  => op.calc_float(*a, b as f64),
            _ => panic!(),
        }
    }
    fn comparison_op(&self, op: BuiltIn, right: Expr) -> Expr {
        let result = match op {
            BuiltIn::Eq => self.eq(&right),
            BuiltIn::Neq => !self.eq(&right),
            _ => panic!("no yet implemented"),
        };
        Bool(result)
    }
    fn binary_op(&self, op: BuiltIn, right: &Expr, ctx: &mut Context) -> Expr {
        match (op, self) {
            (BuiltIn::And, &FALSE) => FALSE,
            (BuiltIn::Or, &TRUE) => TRUE,
            (BuiltIn::And, &TRUE) => right.clone().eval(ctx).to_bool(),
            (BuiltIn::Or, &FALSE) => right.clone().eval(ctx).to_bool(),
            _ => panic!("not boolean"),
        }
    }
    fn to_bool(self) -> Expr {
       match self {
           Bool(_) => self,
           _ => Error(ErrorCode::NotBoolean)
       }
    }
}

fn call(name: &str, mut args: Vec<Expr>, ctx: &mut Context) -> Expr {
    if let Ok(op) = BuiltIn::from_str(name) {
        if args.len() < op.call_args() {
            return Error(ErrorCode::WrongArgumentsNumber(op.call_args(), args.len()))
        }
        let mut obj = args.remove(0);
        if !op.is_macro() {
            obj = obj.eval(ctx)
        }
        match op {
            BuiltIn::ToStr => obj.unitary_op(op),
            BuiltIn::Add | BuiltIn::Sub | BuiltIn::Mul | BuiltIn::Div | BuiltIn::Mod => obj.arithmetic_op(op, args[0].clone().eval(ctx)),
            BuiltIn::Eq | BuiltIn::Neq | BuiltIn::Ge | BuiltIn::Gt | BuiltIn::Le | BuiltIn::Lt => obj.comparison_op(op, args[0].clone().eval(ctx)),
            BuiltIn::And | BuiltIn::Or => obj.binary_op(op, &args[0], ctx),
            BuiltIn::Var | BuiltIn::Val => obj.store(ctx, args, op, true),
            BuiltIn::Set => obj.store(ctx, args, op, false),
            _ => panic!(),
        }
    } else {
        panic!("{} is not a built-in function", name)
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
            None => Error(ErrorCode::UndefinedSymbol(name.to_string())),
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
    pub fn read(&mut self, str: &str) -> Expr { Expr::read(str, self) }
    pub fn exec(&mut self, str: &str) -> String { self.read(str).eval(self).print() }
}

// *********************************** TESTS ******************************************

#[cfg(test)]
mod tests {
    use super::*;

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
