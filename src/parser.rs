use std::string::ToString;
use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::{Op, PrattParser};
use pest::pratt_parser::Assoc::Left;
use pest_derive::Parser;

use crate::{Expr, FALSE, NIL, TRUE};

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
        if let Expr::Call(v) = right {
            let mut args = v.clone();
            args.insert(0, left);
            return Expr::Call(args)
        }
    }
    Expr::Call(vec!(Expr::Symbol(operator_name(op)), left, right))
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
        Rule::CallExpr => Expr::Call(to_vec(pair, 0, 0, None)),
        Rule::Declaration => Expr::Call(to_vec(pair, 4, 2, None)),
        Rule::Assignment => Expr::Call(to_vec(pair, 0, 0, Some("set"))),
        Rule::IfElse =>  Expr::Call(to_vec(pair, 3, 0 , Some("if"))),
        Rule::Block => Expr::Block(to_vec(pair, 0, 0, None)),
        _ => unreachable!("rule not implemented {}", pair.to_string())
    }
}

fn to_vec(pair: Pair<Rule>, expected_len: usize, optional_pos: usize, prefix: Option<&str>) -> Vec<Expr> {
    let mut args: Vec<Expr> = pair.into_inner().into_iter().map(|p| parse_primary(p)).collect();
    if expected_len > 0 && args.len() < expected_len {
        if optional_pos > 0 {
            args.insert(optional_pos, Expr::Nil)
        } else {
            args.resize(expected_len, Expr::Nil)
        }
    }
    if let Some(s) = prefix {
        args.insert(0,  Expr::Symbol(s.to_string()))
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
        _ => panic!("unsupported literal {}", str),
    }
}

#[cfg(test)]
mod tests {
    use crate::Expr;
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
        assert_eq!(Expr::Str("abc".to_string()), parse("\"abc\"").unwrap());
        assert_eq!(Expr::Str("true".to_string()), parse("\"true\"").unwrap());
    }

    #[test]
    fn test_failure() {
        assert_eq!(parse("=2").err(), Some("expected Symbol, Expr, or IfElse".to_owned()));
    }

    #[test]
    fn test_declarations() {
        assert_eq!("Call([Symbol(var), Symbol(a), Nil, Int(1)])", read("var a = 1"));
        assert_eq!("Call([Symbol(val), Symbol(f), TypeSpec(Float), Float(1.0)])", read("val f: Float = 1.0"));
    }

    #[test]
    fn test_assignments() {
        assert_eq!("Call([Symbol(set), Symbol(a), Int(2)])", read("a = 2"));
        assert_eq!("Call([Symbol(set), Symbol(a), Int(2)])", read("set(a, 2)"));
    }

    #[test]
    fn test_arithmetic_order() {
        assert_eq!("Call([Symbol(mul), Int(1), Int(2)])", read("1 * 2"));
        assert_eq!("Call([Symbol(add), Int(1), Call([Symbol(mul), Int(2), Int(3)])])", read("1 + 2 * 3"));
        assert_eq!("Call([Symbol(mul), Int(1), Call([Symbol(add), Int(-2), Int(3)])])", read("1 * (-2 + 3)"));
    }

    #[test]
    fn test_boolean_expressions() {
        assert_eq!("Call([Symbol(and), Call([Symbol(or), Call([Symbol(eq), Symbol(x), Int(2)]), Call([Symbol(eq), Symbol(y), Int(1)])]), Symbol(z)])",
                   read("(x == 2) || (y == 1) && z"));
    }

    #[test]
    fn test_calls() {
        assert_eq!("Call([Symbol(print), Symbol(a)])", read("print(a)"));
        assert_eq!("Call([Int(1), Symbol(mul), Int(2)])", read("1.mul(2)]"));
        assert_eq!("Call([Int(1), Symbol(mul), Call([Int(-2), Symbol(add), Int(3)])])", read("1.mul(-2.add(3))"));
    }

    #[test]
    fn test_if() {
        assert_eq!("Call([Symbol(if), Call([Symbol(eq), Symbol(a), Int(1)]), Block([Int(2)]), Block([Int(3)])])", read("if (a == 1) { 2 } else { 3 }"));
        assert_eq!("Call([Symbol(if), Bool(true), Block([Int(1)]), Nil])", read("if (true) { 1 } "));
    }
}

