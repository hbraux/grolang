use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::{Op, PrattParser};
use pest::pratt_parser::Assoc::Left;
use pest_derive::Parser;

use crate::{ErrorCode, Expr, FALSE, NIL, TRUE};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GroParser;

lazy_static! {
    static ref PARSER: PrattParser<Rule> = {
        PrattParser::new()
        .op(Op::infix(Rule::Dot, Left))
        .op(Op::infix(Rule::Add, Left) | Op::infix(Rule::Sub, Left))
        .op(Op::infix(Rule::Mul, Left) | Op::infix(Rule::Div, Left) | Op::infix(Rule::Mod, Left))
    };
}

pub fn parse(str: &str) -> Expr {
    match GroParser::parse(Rule::Statement, str) {
        Ok(pairs) => parse_expr(pairs),
        Err(e)    => Expr::Error(ErrorCode::SyntaxError(e.to_string()))
    }
}

fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PARSER
        .map_primary(|p| parse_primary(p))
        .map_infix(|left, op, right| reduce_expr(left, op, right))
        .parse(pairs)
}

fn reduce_expr(left: Expr, op: Pair<Rule>, right: Expr) -> Expr {
    if op.as_rule() == Rule::Dot {
        if let Expr::ChainCall(x, y) = right {
            return Expr::Call(Box::new(left), x, y)
        }
    }
    Expr::Call(Box::new(left), Box::new(Expr::Id(operator_name(op))), vec!(right))
}

fn parse_primary(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::Int => Expr::Int(pair.as_str().trim().replace("_", "").parse::<i64>().unwrap()),
        Rule::Float => Expr::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::Special => to_literal(pair.as_str()),
        Rule::String => Expr::Str(unquote(pair.as_str())),
        Rule::Id => Expr::Id(pair.as_str().to_string()),
        Rule::Symbol => Expr::Symbol(remove_first(pair.as_str())),
        Rule::TypeSpec => Expr::TypeSpec(pair.as_str().replace(":", "").trim().to_string()),
        Rule::Operator => Expr::Id(pair.as_str().to_string()),
        Rule::Expr =>  parse_expr(pair.into_inner()),
        Rule::Declaration => macro_call("def".to_owned() + pair.as_str().split(" ").next().unwrap(), pair),
        Rule::Assignment => macro_call("set".to_owned(), pair),
        Rule::CallExpr => fun_call(pair),
        _ => unreachable!("Rule not implemented {}", pair.to_string())
    }
}

fn macro_call(operator: String, pair: Pair<Rule>) -> Expr {
    let mut args: Vec<Expr> = pair.into_inner().into_iter().map(|p| parse_primary(p)).collect();
    let left = args.remove(0);
    if let Expr::Id(x) = left {
        Expr::Call(Box::new(Expr::Symbol(x)), Box::new(Expr::Id(operator.to_string())), args)
    } else {
        panic!()
    }
}

fn fun_call(pair: Pair<Rule>) -> Expr {
    let mut args: Vec<Expr> = pair.into_inner().into_iter().map(|p| parse_primary(p)).collect();
    let mut left = args.remove(0);
    if let Expr::Id(_) = left {
        Expr::ChainCall(Box::new(left), args)
    } else {
        panic!()
    }
}

fn unquote(str: &str) -> String {
    (&str[1..str.len()-1]).to_string()
}

fn remove_first(str: &str) -> String {
    (&str[1..str.len()]).to_string()
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
    fn test_expressions() {
        assert_eq!("Call(Symbol(\"a\"), Id(\"defvar\"), [Int(1)])", parse("var a = 1").format());
        assert_eq!("Call(Symbol(\"a\"), Id(\"defvar\"), [Int(1)])", parse("'a.defvar(1)").format());
        assert_eq!("Call(Symbol(\"f\"), Id(\"defval\"), [TypeSpec(\"Float\"), Float(1.0)])", parse("val f: Float = 1.0").format());
        assert_eq!("Call(Symbol(\"a\"), Id(\"set\"), [Int(2)])", parse("a = 2").format());
        assert_eq!("Call(Symbol(\"a\"), Id(\"set\"), [Int(2)])", parse("'a.set(2)").format());
    }

    #[test]
    fn test_arithmetic_order() {
        assert_eq!("Call(Int(1), Id(\"mul\"), [Int(2)])", parse("1 * 2").format());
        assert_eq!("Call(Int(1), Id(\"add\"), [Call(Int(2), Id(\"mul\"), [Int(3)])])", parse("1 + 2 * 3").format());
        assert_eq!("Call(Int(1), Id(\"mul\"), [Call(Int(-2), Id(\"add\"), [Int(3)])])", parse("1 * (-2 + 3)").format());

    }

    #[test]
    fn test_chain_calls() {
        assert_eq!("ChainCall(Id(\"print\"), [Id(\"a\")])", parse("print(a)").format());
        assert_eq!("Call(Int(1), Id(\"mul\"), [Int(2)])", parse("1.mul(2)]").format());
        assert_eq!("Call(Int(1), Id(\"mul\"), [Call(Int(-2), Id(\"add\"), [Int(3)])])", parse("1.mul(-2.add(3))").format());
    }
}

