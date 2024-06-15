use std::{env, io};
use std::collections::HashMap;
use std::io::Write;

use parser::{eval_expr, read_expr};

const VERSION: &str = "0.1";
const PROMPT: &str  = "> ";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
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
                _ => continue,
            }
        }
        let expr = read_expr(input);
        let result = eval_expr(expr, &values);
        println!("{}", result)
    }
    println!(".")
}

