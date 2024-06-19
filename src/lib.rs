pub mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use crate::ast::{Expr, NULL, Opcode};
use crate::ast::ErrorType::{DivisionByZero, NotANumber, UndefinedSymbol, CannotParse};
use crate::ast::Expr::{Bool, Declare, Error, Float, Id, Int, Null, Op, Str};


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
    pub fn set(&mut self, name: &str, expr: Expr) -> Expr {
        self.values.insert(name.to_string(), expr);
        NULL
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

    pub fn eval(self, ctx: &mut Context) -> Expr {
        eval_expr(self, ctx)
    }

    pub fn print(self) -> String {
        match self {
            Bool(x) => x.to_string(),
            Int(x) => x.to_string(),
            Float(x) => x.to_string(),
            Str(x) => format!("\"{}\"", x),
            Null => "null".to_string(),
            _ => format!("{:?}", self)
        }
    }
}

// TODO: move to impl
fn eval_expr(expr: Expr, ctx: &mut Context) -> Expr {
    match expr {
        Id(s) => ctx.get(&*s).clone(),
        Declare(s, right) => { let value = eval_expr(*right, ctx); ctx.set(s.as_str(), value) },
        Op(left, code, right) => eval_op(eval_expr(*left, ctx), code, eval_expr(*right, ctx)),
        _ => expr.clone()
    }
}


fn eval_op(left: Expr, code: Opcode, right: Expr) -> Expr {
    if let (Int(a), Int(b)) = (left, right) {
        match code {
            Opcode::Add => Int(a + b),
            Opcode::Sub => Int(a - b),
            Opcode::Mul => Int(a * b),
            Opcode::Div => if b !=0 { Int(a / b) } else { Error(DivisionByZero) },
        }
    } else {
        Error(NotANumber)
    }
}

