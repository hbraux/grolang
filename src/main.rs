mod ast;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use assert_approx_eq::assert_approx_eq;
use crate::ast::{Expr, Opcode};


fn main() {

}

fn calc(str: &str) -> f64 {
    let values = HashMap::from([("A", 1.0), ("B", 0.5)]);
    match grammar::ExprParser::new().parse(str)  {
        Ok(expr) => eval(*expr, &values),
        Err(e) => panic!("Cannot parse: {e}"),
    }
}

fn eval(expr: Expr, values: &HashMap<&str, f64>) -> f64 {
    match expr {
        Expr::Number(f) => f,
        Expr::Identifier(s) => *values.get(&*s).unwrap(),
        Expr::Op(left, code, right) => operation(code, eval(*left, values), eval(*right, values))
    }
}

fn operation(code: Opcode, a: f64, b: f64) -> f64 {
    match code {
        Opcode::Add => a + b,
        Opcode::Sub => a - b,
        Opcode::Mul => a * b,
        Opcode::Div => a / b
    }
}

#[test]
fn test() {
    assert_eq!(14.0, calc("2 + 3 * 4"));
    assert_eq!(20.0, calc("(2 + 3) * 4"));
    assert_eq!(4.0, calc("4 / 1.0"));
    assert_approx_eq!(3.3, calc("-2.2 * -1.5"), 0.0001);
    assert_eq!(5.0, calc("4 + A"));
    assert_eq!(2.0, calc("A / B"));
}
