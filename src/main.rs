use std::{env, io};
use std::collections::HashMap;

use parser::{eval_expr, read_expr};

static VERSION: &str = "0.1";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        repl()
    } else {
        match args[1].as_ref() {
            "-v" => println!("groLang v{}", VERSION),
            _ => println!("paramètre non reconnu: {}", args[1]),
        }
    }
}

fn repl() {
    println!("Bienvenue sur groLang version {}", VERSION);
    println!("Taper :q pour quiier, :h pour de l'aide");
    let values: HashMap<&str, i64> = HashMap::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => break,
            _ => {},
        }
        let expr = read_expr(&input);
        let result = eval_expr(expr, &values);
        println!("{}", result)
    }
}

