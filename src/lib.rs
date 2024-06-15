pub mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;
use crate::ast::{Expr, Opcode};

const EXCEPT_DIV0: &str = "Division par 0";

//noinspection ALL
pub fn read_expr(str: &str) -> Expr {
    match grammar::ExprParser::new().parse(str)  {
        Ok(expr) => *expr,
        Err(e) =>  Expr::Error(e.to_string()),
    }
}

pub fn eval_expr(expr: Expr, values: &HashMap<&str, Expr>) -> Expr {
    match expr {
        Expr::Identifier(s) => values.get(&*s).unwrap().clone(),
        Expr::Op(left, code, right) => eval_op(eval_expr(*left, values), code, eval_expr(*right, values)),
        _ => expr
    }
}

fn eval_op(left: Expr, code: Opcode, right: Expr) -> Expr {
    if let (Expr::Integer(a), Expr::Integer(b)) = (left, right) {
        match code {
            Opcode::Add => Expr::Integer(a + b),
            Opcode::Sub => Expr::Integer(a - b),
            Opcode::Mul => Expr::Integer(a * b),
            Opcode::Div => if b !=0 { Expr::Integer(a / b) } else { Expr::Error(EXCEPT_DIV0.to_string()) },
        }
    } else {
        Expr::Error(format!("cannot {:?}", code))
    }
}


#[test]
fn test() {
    fn calc(str: &str) -> i64 {
        let values: HashMap<&str, Expr> = HashMap::from([("a", Expr::Integer(1)), ("b",  Expr::Integer(2))]);
        if let Expr::Integer(i) = eval_expr(read_expr(str), &values) { i } else { -9999999 }
    }
    assert_eq!(14, calc("2 + 3 * 4"));
    assert_eq!(20, calc("(2 + 3) * 4"));
    assert_eq!(4, calc("4 / 1"));
    assert_eq!(2, calc("-2 * -1"));
    assert_eq!(5, calc("4 + a"));
    assert_eq!(2, calc("b / a"));
}
