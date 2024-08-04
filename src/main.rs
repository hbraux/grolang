use std::{env};
use grolang::{eval_line, LANG, repl, VERSION};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        repl()
    } else {
        match args[1].as_ref() {
            "-v" => println!("{} v{}", LANG, VERSION),
            "-e" => args.get(2).map(|e| eval_line(e)).unwrap_or(()),
            _ => println!("Unknown command: {}", args[1]),
        };
    }
}


