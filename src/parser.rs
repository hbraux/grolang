use lazy_static::lazy_static;
use pest::iterators::Pair;
use pest::Parser;
use pest::pratt_parser::{Op, PrattParser};
use pest::pratt_parser::Assoc::Left;
use pest_derive::Parser;

use crate::{Code, ErrorType, Expr, FALSE, NULL, TRUE};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GroParser;

lazy_static! {
    static ref PARSER: PrattParser<Rule> = {
        PrattParser::new().op(Op::infix(Rule::BinaryOp, Left))
    };
}

pub fn parse(str: &str) -> Expr {
    match GroParser::parse(Rule::Statement, str) {
        Ok(pairs) => PARSER.map_primary(|p| to_expr(p)).parse(pairs),
        Err(e)    => Expr::Error(ErrorType::CannotParse(e.to_string()))
    }
}


fn to_expr(pair: Pair<Rule>) -> Expr {
    println!("DEBUG {}", pair);
    match pair.as_rule() {
        Rule::Int => Expr::Int(pair.as_str().replace("_", "").parse::<i64>().unwrap()),
        Rule::Float => Expr::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::Special => literal(pair.as_str()),
        Rule::String => Expr::Str(unquote(pair.as_str())),
        Rule::BinaryExpr => {
            let mut args = pair.into_inner();
            let left =  to_expr(args.next().unwrap());
            let op = Code::new(args.next().unwrap().as_str());
            let right = to_expr(args.next().unwrap());
            Expr::BinaryOp(Box::new(left),op, Box::new(right))
        }
        _ => unreachable!()
    }
}


fn unquote(str: &str) -> String {
    (&str[1..str.len() - 1]).to_string()
}

fn literal(str: &str) -> Expr {
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
    fn test_literals() {
        assert_eq!(Expr::Int(1), parse("1"));
        assert_eq!(Expr::Int(2), parse("+2"));
        assert_eq!(Expr::Int(1234567), parse("1_234_567"));
        assert_eq!(Expr::Int(-23_000), parse("-23_000"));
        assert_eq!(Expr::Float(3.4), parse("+3.4"));
        assert_eq!(Expr::Float(12000.0), parse("1.2e4"));
        assert_eq!(TRUE, parse("true"));
        assert_eq!(FALSE, parse("false"));
        assert_eq!(NULL, parse("null"));
        assert_eq!(Expr::Str("abc".to_string()), parse("\"abc\""));
        assert_eq!(Expr::Str("true".to_string()), parse("\"true\""));
    }

    #[test]
    fn test_others() {
        assert_eq!(Expr::Int(1), parse("1 + 2"));
    }
}

