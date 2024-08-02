use std::borrow::ToOwned;
use std::string::ToString;

use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::{Op, PrattParser};
use pest::pratt_parser::Assoc::Left;
use pest_derive::Parser;

use crate::expr::{Expr, FALSE, NULL, TRUE};
use crate::if_else;
use crate::types::Type;

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
        Err(e)    => Err(e.variant.to_string()),
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
    Expr::Call(to_operator_name(op), vec!(left, right))
}


fn parse_primary(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::Int => Expr::Int(pair.as_str().trim().replace("_", "").parse::<i64>().unwrap()),
        Rule::Float => Expr::Float(pair.as_str().parse::<f64>().unwrap()),
        Rule::Special => to_literal(pair.as_str()),
        Rule::String => Expr::Str(un_quote(pair.as_str())),
        Rule::Symbol | Rule::VarType => Expr::Symbol(pair.as_str().to_owned()),
        Rule::RawType => Expr::TypeOf(Type::from_str(remove_first(pair.as_str()).trim()).unwrap()), // TODO: handle unwrap
        Rule::Operator => Expr::Symbol(pair.as_str().to_owned()),
        Rule::Expr =>  parse_pairs(pair.into_inner()),
        Rule::CallExpr => build_call(to_vec(pair, 0, 0)),
        Rule::Declaration => build_call(to_vec(pair, 4, 2)),
        Rule::Assignment => Expr::Call("assign".to_owned(), to_vec(pair, 0, 0)),
        Rule::IfElse =>  Expr::Call("if".to_owned(), to_vec(pair, 3, 0 )),
        Rule::While => Expr::Call("while".to_owned(), to_vec(pair, 0, 0)),
        Rule::Block => Expr::Block(to_vec(pair, 0, 0)),
        Rule::Definition => Expr::Call("fun".to_owned(), to_vec(pair, 0, 0)),
        Rule::List  => build_list(to_vec(pair, 0, 0)),
        Rule::Map  =>  build_map(to_vec(pair, 0, 0)),
        Rule::Parameters  => build_params(pair.into_inner()),
        Rule::Struct  =>  Expr::Call("struct".to_owned(), to_vec(pair, 0, 0)),
        _ => panic!("Rule '{}' not implemented", to_operator_name(pair))
    }
}

fn build_call(mut args: Vec<Expr>) -> Expr {
    if let Expr::Symbol(name) = args.remove(0) {
        Expr::Call(name, args)
    } else { panic!("first arg should be a symbol") }
}

fn build_list(args: Vec<Expr>) -> Expr {
    Expr::List(Type::infer_list(&args), args)
}

fn build_map(args: Vec<Expr>) -> Expr {
    let pairs: Vec<(Expr, Expr)> = args.chunks(2).into_iter().flat_map(|p| if_else!(p.len() == 2, Some((p[0].clone(), p[1].clone())), None)).collect();
    Expr::Map(Type::infer_map(&pairs), pairs)
}



fn build_params(pairs: Pairs<Rule>) -> Expr {
    Expr::Params(pairs.into_iter().map(|p| {
        let s: Vec<&str> = p.as_str().split(":").collect();
        (s[0].trim().to_string(), Type::from_str(s[1].trim()).unwrap())
    }).collect::<Vec<_>>())
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


fn un_quote(str: &str) -> String {
    (&str[1..str.len()-1]).to_owned()
}

fn remove_first(str: &str) -> String {
    (&str[1..str.len()]).to_owned()
}

fn to_operator_name(pair: Pair<Rule>) -> String {
    format!("{:?}", pair.as_rule()).to_lowercase()
}

fn to_literal(str: &str) -> Expr {
    match str {
        "true" => TRUE,
        "false" => FALSE,
        "nil" => NULL,
        _ => panic!("unsupported literal '{}'", str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Warning, using debug format for expr
    fn read(str: &str) -> String { format!("{:?}", parse(str).unwrap()).replace("\"","") }

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
        assert_eq!(NULL, parse("nil").unwrap());
        assert_eq!(Expr::Str("abc".to_owned()), parse(r#""abc""#).unwrap());
        assert_eq!(Expr::Str("true".to_owned()), parse(r#""true""#).unwrap());
        assert_eq!(Expr::Str("escaped \\n \\t \\\" \\\\ string".to_owned()), parse(r#""escaped \n \t \" \\ string""#).unwrap());
        assert_eq!(Expr::TypeOf(Type::Float), parse(": Float").unwrap());
        assert_eq!(Expr::TypeOf(Type::List(Box::new(Type::Int))), parse(":List<Int>").unwrap());
    }

    #[test]
    fn test_collections() {
        assert_eq!("List(List(Int), [Int(1), Int(2)])", read("[1,2]"));
        assert_eq!("Map(Map(Str, Int), [(Str(a), Int(1))])", read("{\"a\":1}"));
        assert_eq!("List(List(Any), [])", read("[]"));
    }

    #[test]
    fn test_json_read() {
        let json = r#"{"employees":[{"name":"alice","age":20,"grade":2.3,"email":"alice@gmail.com"}, {"name":"bob", "age": 21,"email":null,"grade":1.2}]}"#;
        assert!(parse(json).is_ok());
    }


    #[test]
    fn test_struct() {
        assert_eq!("Call(struct, [Symbol(Point), Params([(x, Float), (y, Float)])])", read("struct Point(x: Float, y:Float)"));
    }

    #[test]
    fn test_errors() {
        assert!(parse("=2").err().is_some())
    }

    #[test]
    fn test_declarations() {
        assert_eq!("Call(val, [Symbol(f), TypeOf(Float), Float(1.0)])", read("val f: Float = 1.0"));
        assert_eq!("Call(val, [Symbol(f), TypeOf(Float), Float(1.0)])", read("val(f,:Float,1.0)"));
        assert_eq!("Call(var, [Symbol(a), Nil, Int(1)])", read("var a = 1"));
        assert_eq!("Call(var, [Symbol(l), Nil, List(List(Int), [Int(1), Int(2), Int(3)])])", read("var l = [1,2,3]"));
        assert_eq!("Call(var, [Symbol(l), TypeOf(List(Int)), List(List(Int), [Int(1), Int(2), Int(3)])])", read("var l :List<Int> = [1,2,3]"));
    }

    #[test]
    fn test_assignments() {
        assert_eq!("Call(assign, [Symbol(a), Int(2)])", read("a = 2"));
        assert_eq!("Call(assign, [Symbol(a), Int(2)])", read("assign(a, 2)"));
    }

    #[test]
    fn test_boolean_expr() {
        assert_eq!("Call(and, [Call(or, [Call(eq, [Symbol(x), Int(2)]), Call(ge, [Symbol(y), Int(1)])]), Symbol(z)])", read("(x == 2) || (y >= 1) && z"));
    }

    #[test]
    fn test_calls() {
        assert_eq!("Call(print, [Symbol(a), Symbol(b)])", read("print(a,b)"));
        assert_eq!("Call(mul, [Symbol(a), Call(fact, [Call(sub, [Symbol(a), Int(1)])])])", read("a*fact(a-1)"));
    }

    #[test]
    fn test_block() {
        assert_eq!("Block([Call(val, [Symbol(a), Nil, Int(2)]), Symbol(a)])", read(r#"{
  val a = 2
  a
 }"#));
    }


    #[test]
    fn test_if_while() {
        assert_eq!("Call(if, [Call(eq, [Symbol(a), Int(1)]), Int(2), Int(3)])", read("if (a == 1) 2 else 3"));
        assert_eq!("Call(if, [Bool(true), Block([Int(1)]), Nil])", read("if (true) { 1 } "));
        assert_eq!("Call(while, [Call(le, [Symbol(a), Int(10)]), Block([Call(print, [Symbol(a)]), Call(assign, [Symbol(a), Call(add, [Symbol(a), Int(1)])])])])",
                   read("while (a <= 10) { print(a) ; a = a + 1 }"));
    }

    #[test]
    fn test_fun() {
        assert_eq!("Call(fun, [Symbol(pi), Params([]), TypeOf(Float), Float(3.14)])", read("fun pi() :Float = 3.14"));
        assert_eq!("Call(fun, [Symbol(inc), Params([(a, Int)]), TypeOf(Int), Block([Call(add, [Symbol(a), Int(1)])])])", read("fun inc(a: Int): Int = { a + 1 }"));
    }


}

