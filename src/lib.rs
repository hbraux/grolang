pub mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use crate::ast::{Expr, NULL, Opcode};
use crate::ast::Expr::{Declare, Failure, Id, Int, Op};

const EXCEPT_DIV0: &str = "Division par 0";

impl Expr {
    fn get_type(&self) -> String {
        "ANY".to_string()
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
                None => Failure(format!("Symbole {name} non défini")),
            }
    }
    pub fn set(&mut self, name: &str, expr: Expr) -> Expr {
        self.values.insert(name.to_string(), expr);
        NULL
    }

}

//noinspection ALL
pub fn read_expr(str: &str) -> Expr {
    match grammar::StatementParser::new().parse(str)  {
        Ok(expr) => *expr,
        Err(e) =>  Failure(e.to_string()),
    }
}

pub fn eval_expr(expr: Expr, ctx: &mut Context) -> Expr {
    match expr {
        Id(s) => ctx.get(&*s).clone(),
        Declare(s, right) => ctx.set(s.as_str(), eval_expr(*right, ctx)),
        Op(left, code, right) => eval_op(eval_expr(*left, ctx), code, eval_expr(*right, ctx)),
        Failure(s) => Failure(s),
        _ => panic!("expression {:?} not supported", expr)
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

