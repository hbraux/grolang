use std::borrow::ToOwned;
use std::string::ToString;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::{Op, PrattParser};
use pest::pratt_parser::Assoc::Left;
use pest_derive::Parser;

use crate::expr::{Expr, FALSE, NIL, TRUE};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GroParser;

lazy_static! {
    static ref PARSER: PrattParser<Rule> = {
        // operator priorities from lowest to highest
        PrattParser::new()
        .op(Op::infix(Rule::Or, Left) | Op::infix(Rule::And, Left))
        .op(Op::infix(Rule::Eq, Left) | Op::infix(Rule::Neq, Left) | Op::infix(Rule::Ge, Left) | Op::infix(Rule::Gt, Left) | Op::infix(Rule::Le, Left) | Op::infix(Rule::Lt, Left))
        .op(Op::infix(Rule::Add, Left) | Op::infix(Rule::Sub, Left))
        .op(Op::infix(Rule::Mul, Left) | Op::infix(Rule::Div, Left) | Op::infix(Rule::Mod, Left))
        .op(Op::infix(Rule::Exp, Left))
        .op(Op::infix(Rule::Dot, Left))
    };
}

pub fn parse(str: &str) -> Result<Expr, String> {
    match GroParser::parse(Rule::Statement, str) {
        Ok(pairs) => Ok(parse_pairs(pairs)),
        Err(e)    => Err(e.variant.message().to_string()),
    }
}

fn parse_pairs(pairs: Pairs<Rule>) -> Expr {
    PARSER
        .map_primary(|p| parse_primary(p))
        .map_infix(|left, op, right| reduce_expr(left, op, right))
        .parse(pairs)
}

fn reduce_expr(left: Expr, op: Pair<Rule>, right: Expr) -> Expr {
    if op.as_rule() == Rule::Dot {
        if let Expr::Call(name, mut args) = right {
            args.insert(0, left);
            return Expr::Call(name, args)
        }
    }
    Expr::Call(operator_name(op), vec!(left, right))
}


fn parse_primary(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::Int => Expr::Int(pair.as_str().trim().replace("_", "").parse::<i64>().unwrap()),
        Rule::Float => Expr::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::Special => to_literal(pair.as_str()),
        Rule::String => Expr::Str(unquote(pair.as_str())),
        Rule::Symbol => Expr::Symbol(pair.as_str().to_owned()),
        Rule::TypeSpec => Expr::read_type(pair.as_str()),
        Rule::Operator => Expr::Symbol(pair.as_str().to_owned()),
        Rule::Expr =>  parse_pairs(pair.into_inner()),
        Rule::CallExpr => build_call(to_vec(pair, 0, 0)),
        Rule::Declaration => build_call(to_vec(pair, 4, 2)),
        Rule::Assignment => Expr::Call("set".to_owned(), to_vec(pair, 0, 0)),
        Rule::IfElse =>  Expr::Call("if".to_owned(), to_vec(pair, 3, 0 )),
        Rule::While => Expr::Call("while".to_owned(), to_vec(pair, 0, 0)),
        Rule::Block => Expr::Call("block".to_owned(), to_vec(pair, 0, 0)),
        _ => panic!("rule {} not implemented", operator_name(pair))
    }
}



fn build_call(mut args: Vec<Expr>) -> Expr {
    if let Expr::Symbol(name) = args.remove(0) {
        Expr::Call(name, args)
    } else {
        panic!("first arg should be a symbol here")
    }
}

fn to_vec(pair: Pair<Rule>, expected_len: usize, optional_pos: usize) -> Vec<Expr> {
    let mut args: Vec<Expr> = pair.into_inner().into_iter().map(|p| parse_primary(p)).collect();
    if expected_len > 0 && args.len() < expected_len {
        if optional_pos > 0 {
            args.insert(optional_pos, Expr::Nil)
        } else {
            args.resize(expected_len, Expr::Nil)
        }
    }
    args
}

fn unquote(str: &str) -> String {
    (&str[1..str.len()-1]).to_owned()
}
fn operator_name(pair: Pair<Rule>) -> String {
    format!("{:?}", pair.as_rule()).to_lowercase()
}
fn to_literal(str: &str) -> Expr {
    match str {
        "true" => TRUE,
        "false" => FALSE,
        "nil" => NIL,
        _ => panic!("unsupported literal {}", str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn read(str: &str) -> String { parse(str).unwrap().format() }

    #[test]
    fn test_literals() {
        assert_eq!(Expr::Int(1), parse("1").unwrap());
        assert_eq!(Expr::Int(1234567), parse("1_234_567").unwrap());
        assert_eq!(Expr::Int(-23_000), parse("-23_000").unwrap());
        assert_eq!(Expr::Float(3.4), parse("3.4").unwrap());
        assert_eq!(Expr::Float(12000.0), parse("1.2e4").unwrap());
        assert_eq!(Expr::Float(0.12), parse("1.2e-1").unwrap());
        assert_eq!(TRUE, parse("true").unwrap());
        assert_eq!(FALSE, parse("false").unwrap());
        assert_eq!(NIL, parse("nil").unwrap());
        assert_eq!(Expr::Str("abc".to_owned()), parse("\"abc\"").unwrap());
        assert_eq!(Expr::Str("true".to_owned()), parse("\"true\"").unwrap());
    }

    #[test]
    fn test_failure() {
        assert_eq!(parse("=2").err(), Some("expected Symbol, Expr, IfElse, or While".to_owned()));
    }

    #[test]
    fn test_declarations() {
        assert_eq!("Call(var, [Symbol(a), Nil, Int(1)])", read("var a = 1"));
        assert_eq!("Call(val, [Symbol(f), TypeSpec(Float), Float(1.0)])", read("val f: Float = 1.0"));
    }

    #[test]
    fn test_assignments() {
        assert_eq!("Call(set, [Symbol(a), Int(2)])", read("a = 2"));
        assert_eq!("Call(set, [Symbol(a), Int(2)])", read("set(a, 2)"));
    }

    #[test]
    fn test_arithmetic_order() {
        assert_eq!("Call(mul, [Int(1), Int(2)])", read("1 * 2"));
        assert_eq!("Call(add, [Int(1), Call(mul, [Int(2), Int(3)])])", read("1 + 2 * 3"));
        assert_eq!("Call(mul, [Int(1), Call(add, [Int(-2), Int(3)])])", read("1 * (-2 + 3)"));
    }

    #[test]
    fn test_boolean_expressions() {
        assert_eq!("Call(and, [Call(or, [Call(eq, [Symbol(x), Int(2)]), Call(eq, [Symbol(y), Int(1)])]), Symbol(z)])",
            read("(x == 2) || (y == 1) && z"));
    }

    #[test]
    fn test_calls() {
        assert_eq!("Call(print, [Symbol(a)])", read("print(a)"));
        assert_eq!("Call(mul, [Int(1), Int(2)])", read("1.mul(2)]"));
        assert_eq!("Call(mul, [Int(1), Call(add, [Int(-2), Int(3)])])", read("1.mul(-2.add(3))"));
    }

    #[test]
    fn test_builtins() {
        assert_eq!("Call(if, [Call(eq, [Symbol(a), Int(1)]), Call(block, [Int(2)]), Call(block, [Int(3)])])", read("if (a == 1) { 2 } else { 3 }"));
        assert_eq!("Call(if, [Bool(true), Call(block, [Int(1)]), Nil])", read("if (true) { 1 } "));
        assert_eq!("Call(while, [Call(le, [Symbol(a), Int(10)]), Call(block, [Call(set, [Symbol(a), Call(add, [Symbol(a), Int(1)])])])])",
                   read("while (a < 10) { a = a + 1 }"));
    }
}

