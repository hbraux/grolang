pub mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use crate::ast::{Expr, NULL, Opcode};
use crate::ast::Expr::{Declare, Failure, Id, Int, Op};

const EXCEPT_DIV0: &str = "Division par 0";



pub struct Context {
    values: HashMap<String, Expr>,
}

impl Context {
    pub fn new() -> Context { Context { values: HashMap::new() } }

    pub fn get(&self, name: &str) -> Expr {
        match self.values.get(name)  {
                Some(expr) => expr.clone(),
                None => Failure(format!("Symbole {name} non défini")),
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
            Err(e) =>  Failure(e.to_string()),
        }
    }
}

pub fn eval(expr: Expr, ctx: &mut Context) -> Expr {
    match expr {
        Id(s) => ctx.get(&*s).clone(),
        Declare(s, right) => { let value = eval(*right, ctx); ctx.set(s.as_str(), value) },
        Op(left, code, right) => eval_op(eval(*left, ctx), code, eval(*right, ctx)),
        _ => expr.clone()
    }
}


fn eval_op(left: Expr, code: Opcode, right: Expr) -> Expr {
    if let (Int(a), Int(b)) = (left, right) {
        match code {
            Opcode::Add => Int(a + b),
            Opcode::Sub => Int(a - b),
            Opcode::Mul => Int(a * b),
            Opcode::Div => if b !=0 { Expr::Int(a / b) } else { Failure(EXCEPT_DIV0.to_string()) },
        }
    } else {
        Expr::Failure(format!("cannot {:?}", code))
    }
}

