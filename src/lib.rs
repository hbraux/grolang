pub mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use crate::ast::{Expr, Opcode};


//noinspection ALL
pub fn read_expr(str: &str) -> Expr {
    match grammar::ExprParser::new().parse(str)  {
        Ok(expr) => *expr,
        Err(e) =>  Expr::Exception(e.to_string()),
    }
}

pub fn eval_expr(expr: Expr, values: &HashMap<&str, i64>) -> i64 {
    match expr {
        Expr::Integer(i) => i,
        Expr::Identifier(s) => *values.get(&*s).unwrap(),
        Expr::Op(left, code, right) => operation(code, eval_expr(*left, values), eval_expr(*right, values)),
        _ => panic!(),
    }
}

fn operation(code: Opcode, a: i64, b: i64) -> i64 {
    match code {
        Opcode::Add => a + b,
        Opcode::Sub => a - b,
        Opcode::Mul => a * b,
        Opcode::Div => a / b
    }
}


#[test]
fn test() {
    fn calc(str: &str) -> i64 {
        let values: HashMap<&str, i64> = HashMap::from([("a", 1), ("b", 2)]);
        return eval_expr(read_expr(str), &values)
    }
    assert_eq!(14, calc("2 + 3 * 4"));
    assert_eq!(20, calc("(2 + 3) * 4"));
    assert_eq!(4, calc("4 / 1"));
    assert_eq!(2, calc("-2 * -1"));
    assert_eq!(5, calc("4 + a"));
    assert_eq!(2, calc("b / a"));
}
