#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use strum_macros::Display;
use ErrorType::{DivisionByZero, NotANumber, UndefinedSymbol, CannotParse};
use Expr::{Bool, Declare, Error, Float, Id, Int, Null, BinaryOp, Str};


#[derive(Debug, Clone, PartialEq, Display)]
pub enum ErrorType {
    DivisionByZero,
    UndefinedSymbol(String),
    CannotParse(String),
    NotANumber,
    InconsistentType(String),
    AlreadyDefined(String)
}

#[derive(Debug, Eq, PartialEq, Clone, Display)]
pub enum Type {
    Any,
    Int,
    Bool,
    Str,
    Float,
    Custom(String),
    List(Box<Type>),
    Option(Box<Type>),
    Fail(Box<Type>),
    Map(Box<Type>, Box<Type>)
}

impl Type {
    pub fn new(str: &str) -> Type {
        if str.ends_with("?") {
            Type::Option(Box::new(Type::new(&str[0..str.len()-1])))
        } else if str.ends_with("!") {
            Type::Fail(Box::new(Type::new(&str[0..str.len()-1])))
        } else if str.starts_with("List<") {
            Type::List(Box::new(Type::new(&str[5..str.len() - 1])))
        }else if str.starts_with("List<") {
            Type::List(Box::new(Type::new(&str[5..str.len()-1])))
        } else if str.starts_with("Map<") {
            let s : Vec<&str> = (&str[4..str.len()-1]).split(',').collect();
            Type::Map(Box::new(Type::new(s[0])), Box::new(Type::new(s[1])))
        } else {
            match str {
                "Any" => Type::Any,
                "Int" => Type::Int,
                "Bool" => Type::Bool,
                "Str" => Type::Str,
                "Float" => Type::Float,
                _ => if str.chars().all(|x| x.is_alphabetic()) {
                    Type::Custom(str.to_string())
                } else {
                    panic!("{} is not a valid type name", str)
                }
            }
        }
    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Id(String),
    Declare(String, Option<Type>, Box<Expr>),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    Call(Box<Expr>, String, Vec<Box<Expr>>),
    Error(ErrorType),
    Null
}

pub const TRUE: Expr = Bool(true);
pub const FALSE: Expr = Bool(false);
pub const NULL: Expr = Null;

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
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
    In
}
impl Opcode {
    fn is_arithmetic(&self) -> bool {
        match self {
            Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod => true,
            _ => false
        }
    }
    fn calc_int(&self, a: &i64, b: &i64) -> Expr {
        match self {
            Opcode::Add => Int(a + b),
            Opcode::Sub => Int(a - b),
            Opcode::Mul => Int(a * b),
            Opcode::Div => if *b != 0 { Int(a / b) } else { Error(DivisionByZero) },
            _ =>  panic!()
        }
    }
    fn calc_float(&self, a: &f64, b: &f64) -> Expr {
        match self {
            Opcode::Add => Float(a + b),
            Opcode::Sub => Float(a - b),
            Opcode::Mul => Float(a * b),
            Opcode::Div => if *b != 0.0 { Float(a / b) } else { Error(DivisionByZero) },
            _ =>  panic!()
        }
    }
}


impl Expr {
    //noinspection ALL
    pub fn new(str: &str) -> Expr {
        match grammar::StatementParser::new().parse(str)  {
            Ok(expr) => *expr,
            Err(e) =>  Error(CannotParse(e.to_string())),
        }
    }
    pub fn get_type(&self) -> Type {
        match self {
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            Str(_) => Type::Str,
            _ => Type::Any
        }
    }
    fn is_error(&self) -> bool {
        matches!(self, Error(_))
    }
    fn store(self, name: String, ctx: &mut Context, is_new: bool) -> Expr {
        if is_new && ctx.is_defined(&name) {
            Error(ErrorType::AlreadyDefined(name.to_owned()))
        } else {
            ctx.set(name.as_str(), self.clone());
            self
        }
    }
    fn ensure(self, spec: Option<Type>) -> Expr {
        if let Some(expected) = spec {
            if !self.is_error() && self.get_type() != expected {
                return Error(ErrorType::InconsistentType(expected.to_string()))
            }
        }
        self
    }
    pub fn eval(self, ctx: &mut Context) -> Expr {
        match self {
            Id(name) => ctx.get(&*name).clone(),
            Declare(name, spec, value) => value.eval(ctx).ensure(spec).store(name, ctx, true),
            BinaryOp(left, code, right) => left.eval(ctx).binary_op(code, right.eval(ctx)),
            _ => self.clone()
        }
    }
    pub fn print(self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Float(x) => format_float(x),
            Str(x) => format!("\"{}\"", x),
            Null => "null".to_string(),
            _ => format!("{:?}", self)
        }
    }
    fn unitary_op(self) -> Expr {
        self.clone()
    }
    fn binary_op(&self, code: Opcode, right: Expr) -> Expr {
        if code.is_arithmetic() {
            self.arithmetic_op(code, &right)
        } else {
            self.comparison_op(code, &right)
        }
    }

    fn arithmetic_op(&self, code: Opcode, right: &Expr) -> Expr {
        if let (Int(a), Int(b)) = (self, right) {
            code.calc_int(a, b)
        } else if let (Float(a), Float(b)) = (self, right) {
            code.calc_float(a, b)
        } else if let (Int(a), Float(b)) = (self, right) {
            code.calc_float(&(*a as f64), b)
        } else if let (Float(a), Int(b)) = (self, right) {
            code.calc_float(a, &(*b as f64))
        } else {
            Error(NotANumber)
        }
    }

    fn comparison_op(&self, code: Opcode, right: &Expr) -> Expr {
       let result = match code {
           Opcode::Eq => self.eq(right),
           Opcode::Neq => !self.eq(right),
           _ => panic!("no yet implemented"),
       };
       Bool(result)
    }

}

fn format_float(f: f64) -> String {
    let str = f.to_string();
    if str.contains('.') {
        str
    } else {
        format!("{}.0", str)
    }
}


pub struct Context {
    values: HashMap<String, Expr>,
}

impl Context {
    pub fn new() -> Context { Context { values: HashMap::new() } }

    pub fn get(&self, name: &str) -> Expr {
        match self.values.get(name)  {
            Some(expr) => expr.clone(),
            None => Error(UndefinedSymbol(name.to_string())),
        }
    }
    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    pub fn set(&mut self, name: &str, expr: Expr) {
        self.values.insert(name.to_string(), expr);
    }

}


#[test]
fn test_type() {
    assert_eq!(Type::Any, Type::new("Any"));
    assert_eq!(Type::Int, Type::new("Int"));
    assert_eq!(Type::List(Box::new(Type::Int)), Type::new("List<Int>"));
    assert_eq!(Type::Map(Box::new(Type::Int), Box::new(Type::Bool)), Type::new("Map<Int,Bool>"));
    assert_eq!(Type::Option(Box::new(Type::Int)), Type::new("Int?"));
    assert_eq!(Type::Fail(Box::new(Type::Int)), Type::new("Int!"));
}

#[test]
fn test_eval() {
    let mut ctx = Context::new();
    let mut rep = |str: &str| -> String {
        Expr::new(str).eval(&mut ctx).print()
    };
    // literals
    assert_eq!("1", rep("1"));
    assert_eq!("9123456", rep("9_123_456"));
    assert_eq!("2.0", rep("2."));
    assert_eq!("-1.23", rep("-1.23"));
    assert_eq!("23000.0", rep("2.3e4"));
    assert_eq!("false", rep("false"));
    assert_eq!("true", rep("true"));
    assert_eq!("null", rep("null"));
    assert_eq!("\"abc\"", rep("\"abc\""));

    // variables
    assert_eq!("1", rep("var a = 1"));
    assert_eq!("Error(AlreadyDefined(\"a\"))", rep("var a = 3"));
    assert_eq!("2", rep("var b: Int = 2"));
    assert_eq!("3.2", rep("var c=3.2"));
    assert_eq!("Error(InconsistentType(\"Int\"))", rep("var d: Int =3.2"));
    assert_eq!("1", rep("a"));
    assert_eq!("2", rep("b"));

    // arithmetics
    assert_eq!("14", rep("2 + 3 * 4"));
    assert_eq!("20", rep("(2 + 3) * 4"));
    assert_eq!("4", rep("4 / 1"));
    assert_eq!("2", rep("-2 * -1"));
    assert_eq!("5", rep("4 + a"));
    assert_eq!("2", rep("b / a"));
    assert_eq!("Error(DivisionByZero)", rep("1 / 0"));
}
