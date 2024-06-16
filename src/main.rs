use std::{env, io};
use std::collections::HashMap;
use std::io::Write;
use grolang::{Context, eval_expr, read_expr, Type};
use grolang::ast::Expr;

use grolang::ast::Expr::{Error, Int};

const LANG: &str = "GroLang";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str  = "> ";
const RED : &str = "\x1b[1;31m";
const BLUE : &str = "\x1b[1;34m";
const STD : &str = "\x1b[0m";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        repl()
    } else {
        match args[1].as_ref() {
            "-v" => println!("{} v{}", LANG, VERSION),
            _ => println!("paramètre non supporté: {}", args[1]),
        }
    }
}

fn repl() {
    println!("{BLUE}Bienvenue sur {LANG} version {VERSION}{STD}");
    println!("Taper :q pour quitter, :h pour de l'aide");
    let values: HashMap<&str, i64> = HashMap::new();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(_) => break,
            _ => {},
        }
        let input = line.trim();
        if input.starts_with(':') {
            match input {
                ":q" => break,
                ":h" => help(),
                _ => println!("{RED}Commande inconnue {input}{STD}")
            }
            continue;
        }
        let expr = read_expr(input);
        if let Error(msg) = expr {
            println!("{RED}Erreur de syntaxe ({msg}){STD}");
            continue;
        }
        println!("DEBUG: {:?}", expr);
        let result = eval_expr(expr, &values);
        println!("{}", result)
    }
    println!(".")
}

fn help() {
    println!("Pas disponible pour le moment")
}


#[test]
fn test() {
    let context = Context::new();
    context.set("a", Type::INT, Int::new(1));
    context.set("b", Type::INT, Int::new(2));
    let calc = |str: &str| -> i64 {
        if let Expr::Int(i) = eval_expr(read_expr(str), context) { i } else { -9999999 }
    };
    assert_eq!(14, calc("2 + 3 * 4"));
    assert_eq!(20, calc("(2 + 3) * 4"));
    assert_eq!(4, calc("4 / 1"));
    assert_eq!(2, calc("-2 * -1"));
    assert_eq!(5, calc("4 + a"));
    assert_eq!(2, calc("b / a"));
}
