use std::{env, io};
use std::io::Write;

use grolang::Scope;
use grolang::expr::Expr;


const LANG: &str = "GroLang";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str = "> ";
const RED: &str = "\x1b[1;31m";
const BLUE: &str = "\x1b[1;34m";
const STD: &str = "\x1b[0m";

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
    let mut ctx = Scope::new();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(_) => break,
            _ => {}
        }
        let input = line.trim();
        if input.starts_with(':') {
            match input {
                ":q" => break,
                ":h" => help(),
                _ => println!("{RED}Commande inconnue {input}{STD}"),
            }
            continue;
        }
        let expr = ctx.read(input);
        if let Expr::Failure(error) = expr {
            println!("{RED}Erreur de syntaxe ({:?}){STD}", error);
            continue;
        }
        println!("DEBUG: {:?}", expr);
        let result = expr.eval_or_error(&mut ctx);
        if let Expr::Failure(error) = result {
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
