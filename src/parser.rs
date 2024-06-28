use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::{Op, PrattParser};
use pest::pratt_parser::Assoc::Left;
use pest_derive::Parser;

use crate::{Operator, ErrorType, Expr, FALSE, NULL, TRUE, Type};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GroParser;

lazy_static! {
    static ref PARSER: PrattParser<Rule> = {
        PrattParser::new().op(Op::infix(Rule::Add, Left) | Op::infix(Rule::Sub, Left))
        .op(Op::infix(Rule::Mul, Left) | Op::infix(Rule::Div, Left) | Op::infix(Rule::Mod, Left))
    };
}

pub fn parse(str: &str) -> Expr {
    match GroParser::parse(Rule::Statement, str) {
        Ok(pairs) => parse_expr(pairs),
        Err(e)    => Expr::Error(ErrorType::CannotParse(e.to_string()))
    }
}

fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PARSER.map_primary(|p| parse_primary(p)).map_infix(|l, o, r| infix(l, o.as_rule(), r)).parse(pairs)
}


fn parse_primary(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::Int => Expr::Int(pair.as_str().trim().replace("_", "").parse::<i64>().unwrap()),
        Rule::Float => Expr::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::Special => to_literal(pair.as_str()),
        Rule::String => Expr::Str(unquote(pair.as_str())),
        Rule::TypeSpec => Expr::TypeWrapper(Type::new(pair.as_str())),
        Rule::BinaryOp => Expr::OperatorWrapper(Operator::new(pair.as_str())),
        Rule::Expr =>  parse_expr(pair.into_inner()),
        Rule::Declaration => declare(pair.into_inner()),
        Rule::Assignment => call(Operator::Assign, pair.into_inner()),
        _ => unreachable!()
    }
}

fn infix(left: Expr, rule: Rule, right: Expr)  -> Expr {
    Expr::BinaryOp(Box::new(left), Operator::new(rule_name(rule).as_str()), Box::new(right))
}

fn declare(mut pairs: Pairs<Rule>) -> Expr {
    let id = pairs.next().unwrap().as_str();
    // TODO: typespec
    let right = pairs.next().unwrap().into_inner();
    Expr::Declare(id.to_string(), None, Box::new(parse_expr(right)))
}

fn call(op: Operator, pairs: Pairs<Rule>) -> Expr {
    let mut args: Vec<Expr> = pairs.into_iter().map(|p| parse_primary(p)).collect();
    Expr::Call(Box::new(args.remove(0)), op, args)
}

fn unquote(str: &str) -> String {
    (&str[1..str.len() - 1]).to_string()
}

fn rule_name(rule: Rule) -> String {
    // TODO: there should be a better way to get rule Name
    format!("{:?}", rule)
}
fn to_literal(str: &str) -> Expr {
    match str {
        "true" => TRUE,
        "false" => FALSE,
        "null" => NULL,
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
        assert_eq!(NULL, parse("null"));
        assert_eq!(Expr::Str("abc".to_string()), parse("\"abc\""));
        assert_eq!(Expr::Str("true".to_string()), parse("\"true\""));
    }

    #[test]
    fn test_expressions() {
        assert_eq!("Declare(\"a\", None, Int(1))", parse("var a = 1").format());
       // assert_eq!("Declare(\"f\", None, Float(1.0))", parse("var f: Float = 1.0").format());
        assert_eq!("Assign(\"a\", Int(2))", parse("a = 2").format());

    }

    #[test]
    fn test_arithmetic_order() {
        assert_eq!("BinaryOp(Int(1), Mul, Int(2))", parse("1 * 2").format());
        assert_eq!("BinaryOp(Int(1), Add, BinaryOp(Int(2), Mul, Int(3)))", parse("1 + 2 * 3").format());
        assert_eq!("BinaryOp(Int(1), Mul, BinaryOp(Int(-2), Add, Int(3)))", parse("1 * (-2 + 3)").format());
    }
}

