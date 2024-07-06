use std::string::ToString;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::{Op, PrattParser};
use pest::pratt_parser::Assoc::Left;
use pest_derive::Parser;

use crate::{ErrorCode, Expr, FALSE, NIL, TRUE};
use crate::Expr::Nil;

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

pub fn parse(str: &str) -> Expr {
    match GroParser::parse(Rule::Statement, str) {
        Ok(pairs) => parse_pairs(pairs),
        Err(e)    => Expr::Error(ErrorCode::SyntaxError(e.to_string()))
    }
}

fn parse_pairs(pairs: Pairs<Rule>) -> Expr {
    PARSER
        .map_primary(|p| parse_primary(p))
        .map_infix(|left, op, right| reduce_expr(left, op, right))
        .parse(pairs)
}

fn reduce_expr(left: Expr, op: Pair<Rule>, right: Expr) -> Expr {
    if op.as_rule() == Rule::Dot {  // conditions can't be combined
        if let Expr::ChainCall(args) = right {
            return Expr::Call(Box::new(left), args)
        }
    }
    Expr::Call(Box::new(left), vec!(Expr::Symbol(operator_name(op)), right))
}

fn parse_primary(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::Int => Expr::Int(pair.as_str().trim().replace("_", "").parse::<i64>().unwrap()),
        Rule::Float => Expr::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::Special => to_literal(pair.as_str()),
        Rule::String => Expr::Str(unquote(pair.as_str())),
        Rule::Symbol => Expr::Symbol(pair.as_str().to_string()),
        Rule::TypeSpec => Expr::parse_type_spec(pair.as_str()),
        Rule::Operator => Expr::Symbol(pair.as_str().to_string()),
        Rule::Expr =>  parse_pairs(pair.into_inner()),
        Rule::CallExpr => Expr::ChainCall(to_vec(pair, 0)),
        Rule::Declaration => Expr::Call(to_box(pair.as_str().split(" ").next().unwrap()), to_vec(pair, 3)),
        Rule::Assignment => Expr::Call(to_box("set"), to_vec(pair, 0)),
        Rule::IfElse =>  Expr::Call(to_box("if"), to_vec(pair, 3)),
        Rule::Block => Expr::Block(to_vec(pair, 0)),
        _ => unreachable!("Rule not implemented {}", pair.to_string())
    }
}

fn to_box(str: &str) -> Box<Expr> {
    Box::new(Expr::Symbol(str.to_string()))
}

fn to_vec(pair: Pair<Rule>, expected_len: usize) -> Vec<Expr> {
    let mut args: Vec<Expr> = pair.into_inner().into_iter().map(|p| parse_primary(p)).collect();
    if expected_len > 0 && args.len() < expected_len {
        args.resize(expected_len, Nil)
    }
    args
}


fn unquote(str: &str) -> String {
    (&str[1..str.len()-1]).to_string()
}

fn operator_name(pair: Pair<Rule>) -> String {
    format!("{:?}", pair.as_rule()).to_lowercase()
}

fn to_literal(str: &str) -> Expr {
    match str {
        "true" => TRUE,
        "false" => FALSE,
        "nil" => NIL,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::Expr;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Expr::Int(1), parse("1"));
        assert_eq!(Expr::Int(1234567), parse("1_234_567"));
        assert_eq!(Expr::Int(-23_000), parse("-23_000"));
        assert_eq!(Expr::Float(3.4), parse("3.4"));
        assert_eq!(Expr::Float(12000.0), parse("1.2e4"));
        assert_eq!(Expr::Float(0.12), parse("1.2e-1"));
        assert_eq!(TRUE, parse("true"));
        assert_eq!(FALSE, parse("false"));
        assert_eq!(NIL, parse("nil"));
        assert_eq!(Expr::Str("abc".to_string()), parse("\"abc\""));
        assert_eq!(Expr::Str("true".to_string()), parse("\"true\""));
    }

    #[test]
    fn test_declarations() {
        assert_eq!("Call(Symbol(a), Id(defvar), [Int(1), Nil])", parse("var a = 1").format());
        assert_eq!("Call(Symbol(f), Id(defval), [TypeSpec(Float), Float(1.0)])", parse("val f: Float = 1.0").format());
    }

    #[test]
    fn test_assignments() {
        assert_eq!("Call(Symbol(a), Id(set), [Int(2)])", parse("a = 2").format());
        assert_eq!("Call(Symbol(a), Id(set), [Int(2)])", parse("'a.set(2)").format());
        assert_eq!("Call(Symbol(a), Id(eq), [Int(2)])", parse("'a == 2").format());
    }

    #[test]
    fn test_arithmetic_order() {
        assert_eq!("Call(Int(1), Id(mul), [Int(2)])", parse("1 * 2").format());
        assert_eq!("Call(Int(1), Id(add), [Call(Int(2), Id(mul), [Int(3)])])", parse("1 + 2 * 3").format());
        assert_eq!("Call(Int(1), Id(mul), [Call(Int(-2), Id(add), [Int(3)])])", parse("1 * (-2 + 3)").format());
    }

    #[test]
    fn test_boolean_expressions() {
        assert_eq!("Call(Call(Call(Id(x), Id(eq), [Int(2)]), Id(or), [Call(Id(y), Id(eq), [Int(1)])]), Id(and), [Id(z)])", parse("(x == 2) || (y == 1) && z").format());
    }

    #[test]
    fn test_chain_calls() {
        assert_eq!("ChainCall(Id(print), [Id(a)])", parse("print(a)").format());
        assert_eq!("Call(Int(1), Id(mul), [Int(2)])", parse("1.mul(2)]").format());
        assert_eq!("Call(Int(1), Id(mul), [Call(Int(-2), Id(add), [Int(3)])])", parse("1.mul(-2.add(3))").format());
    }

    #[test]
    fn test_ifelse() {
        assert_eq!("Call(Call(Id(a), Id(eq), [Int(1)]), Id(ifElse), [Block([Int(2)]), Block([Int(3)])])", parse("if (a == 1) { 2 } else { 3 }").format());

    }
}

