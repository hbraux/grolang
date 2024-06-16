pub mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use crate::ast::{Expr, Opcode};

const EXCEPT_DIV0: &str = "Division par 0";

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    ANY,
    INT
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
    name: String,
    of_type: Type
}


pub struct Context {
    symbols: HashMap<String, Symbol>,
    values: HashMap<Symbol, Expr>,
}

impl Context {
    pub fn get(&self, name: &str) -> &Expr {
        if let Some(symbol) = self.symbols.get(name) {
            self.values.get(symbol).unwrap_or_else(Expr::Error(format!("Symbole {name} non défini")))
        } else {
            &Expr::Error(format!("Symbole {name} inconnu"))
        }
    }
    pub fn set(&mut self, name: &str, of_type: Type, expr: Expr) {
        let symbol = Symbol::new(name, of_type);
        self.symbols.insert(name.to_string(), symbol);
        self.values.insert(symbol, expr);
    }
}


pub fn read_expr(str: &str) -> Expr {
    match grammar::ExprParser::new().parse(str)  {
        Ok(expr) => *expr,
        Err(e) =>  Expr::Error(e.to_string()),
    }
}

pub fn eval_expr(expr: Expr, ctx: &Context) -> Expr {
    match expr {
        Expr::Id(s) => ctx.get(&*s).unwrap().clone(),
        Expr::Op(left, code, right) => eval_op(eval_expr(*left, ctx), code, eval_expr(*right, ctx)),
        _ => expr
    }
}

fn eval_op(left: Expr, code: Opcode, right: Expr) -> Expr {
    if let (Expr::Int(a), Expr::Int(b)) = (left, right) {
        match code {
            Opcode::Add => Expr::Int(a + b),
            Opcode::Sub => Expr::Int(a - b),
            Opcode::Mul => Expr::Int(a * b),
            Opcode::Div => if b !=0 { Expr::Int(a / b) } else { Expr::Error(EXCEPT_DIV0.to_string()) },
        }
    } else {
        Expr::Error(format!("cannot {:?}", code))
    }
}

