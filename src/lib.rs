pub mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use crate::ast::{Expr, Opcode};
use crate::ast::Expr::{Failure, Id, Int, Op};

const EXCEPT_DIV0: &str = "Division par 0";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Type {
    ANY,
    INT
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol {
    name: String,
    of_type: Type
}



pub struct Context {
    symbols: HashMap<String, Symbol>,
    values: HashMap<Symbol, Expr>,
}

impl Context {
    pub fn new() -> Context { Context { symbols: HashMap::new(), values: HashMap::new() } }

    pub fn get(&self, name: &str) -> Expr {
        if let Some(symbol) = self.symbols.get(name) {
            match self.values.get(symbol)  {
                Some(expr) => expr.clone(),
                None => Failure(format!("Symbole {name} non défini")),
            }
        } else {
            Failure(format!("Symbole {name} inconnu"))
        }
    }
    pub fn set(&mut self, name: &str, of_type: Type, expr: Expr) {
        let symbol = Symbol { name: name.to_string(), of_type };
        self.symbols.insert(name.to_string(), symbol.clone());
        self.values.insert(symbol, expr);
    }
}


pub fn read_expr(str: &str) -> Expr {
    match grammar::StatementParser::new().parse(str)  {
        Ok(expr) => *expr,
        Err(e) =>  Failure(e.to_string()),
    }
}

pub fn eval_expr(expr: Expr, ctx: &Context) -> Expr {
    match expr {
        Id(s) => ctx.get(&*s).clone(),
        Op(left, code, right) => eval_op(eval_expr(*left, ctx), code, eval_expr(*right, ctx)),
        _ => expr
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

