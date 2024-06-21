use std::{env, io};
use std::io::Write;

use grolang::{Context, Expr};
use grolang::Expr::Error;

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
    let mut context = Context::new();
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
        let expr = Expr::new(input);
        if let Error(error) = expr {
            println!("{RED}Erreur de syntaxe ({:?}){STD}", error);
            continue;
        }
        println!("DEBUG: {:?}", expr);
        let result = expr.eval(&mut context);
        if let Error(error) = result {
            println!("{RED}Erreur d'évaluation ({:?}){STD}", error);
        } else {
            println!("{}", result.print())
        }
    }
    println!(".")
}

fn help() {
    println!("Pas disponible pour le moment")
}


#[test]
fn test() {
    let mut ctx = Context::new();
    let mut rep = |str: &str| -> String {
        Expr::new(str).eval(&mut ctx).print()
    };
    // literals
    assert_eq!("1", rep("1"));
    assert_eq!("-1.23", rep("-1.23"));
    assert_eq!("false", rep("false"));
    assert_eq!("true", rep("true"));
    assert_eq!("null", rep("null"));
    assert_eq!("\"abc\"", rep("\"abc\""));

    // variables
    assert_eq!("null", rep("var a = 1"));
    assert_eq!("null", rep("var b = 2"));
    assert_eq!("1", rep("a"));
    assert_eq!("2", rep("b"));

    // arithmetics
    assert_eq!("14", rep("2 + 3 * 4"));
    assert_eq!("20", rep("(2 + 3) * 4"));
    assert_eq!("4", rep("4 / 1"));
    assert_eq!("2", rep("-2 * -1"));
    assert_eq!("5", rep("4 + a"));
    assert_eq!("2", rep("b / a"));
    assert_eq!("Error(DivisionByZero)", rep("1 / 0"));
}
