use std::io;
use std::io::Write;
use crate::expr::Expr;
use crate::scope::Scope;


mod parser;
mod types;
mod exception;
mod functions;
mod expr;
mod scope;

pub const LANG: &str = "GroLang";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str = "> ";
const RED: &str = "\x1b[1;31m";
const BLUE: &str = "\x1b[1;34m";
const STD: &str = "\x1b[0m";


fn help() {
    println!("Pas disponible pour le moment")
}


pub fn repl() {
    println!("{BLUE}Bienvenue sur {LANG} version {VERSION}{STD}");
    println!("Taper :q pour quitter, :h pour de l'aide");
    let mut scope = Scope::init();
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
        let expr = scope.read(input);
        if let Expr::Failure(error) = expr {
            println!("{RED}Erreur de syntaxe ({:?}){STD}", error);
            continue;
        }
        println!("DEBUG: {:?}", expr);
        let result = expr.eval_or_failed(&mut scope);
        if let Expr::Failure(error) = result {
            println!("{RED}Erreur d'évaluation ({:?}){STD}", error);
        } else {
            println!("{}", result.print())
        }
    }
    println!(".")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literals() {
        let mut scope = Scope::init();
        assert_eq!("1", scope.exec("1"));
        assert_eq!("9123456", scope.exec("9_123_456"));
        assert_eq!("2.0", scope.exec("2.0"));
        assert_eq!("-1.23", scope.exec("-1.23"));
        assert_eq!("23000.0", scope.exec("2.3e4"));
        assert_eq!("false", scope.exec("false"));
        assert_eq!("true", scope.exec("true"));
        assert_eq!("nil", scope.exec("nil"));
        assert_eq!("\"abc\"", scope.exec("\"abc\""));
    }

    #[test]
    fn test_variables() {
        let mut scope = Scope::init();
        assert_eq!("a", scope.exec("var a = 1"));
        assert_eq!("AlreadyDefined(a)", scope.exec("var a = 3"));
        assert_eq!("2", scope.exec("a = a + 1"));
        assert_eq!("0", scope.exec("a.set(0)"));
        assert_eq!("UnexpectedType(Float)", scope.exec("a = 3.0"));
        assert_eq!("c", scope.exec("val c=3.2"));
        assert_eq!("UnexpectedType(Float)", scope.exec("var d: Int = 3.2"));
        assert_eq!("3.2", scope.exec("c"));
        assert_eq!("i", scope.exec("val i = 0"));
        assert_eq!("NotMutable(i)", scope.exec("i = 1"));
        assert_eq!("NotDefined(z)", scope.exec("z = 0"));
    }

    #[test]
    fn test_arithmetics() {
        let mut scope = Scope::init();
        assert_eq!("14", scope.exec("2 + 3 * 4"));
        assert_eq!("20", scope.exec("(2 + 3) * 4"));
        assert_eq!("4", scope.exec("4 / 1"));
        assert_eq!("2", scope.exec("22%10"));
        assert_eq!("2", scope.exec("-2 * -1"));
        assert_eq!("3.3", scope.exec("1.0 + 2.3"));
        assert_eq!("DivisionByZero", scope.exec("1 / 0"));
        assert_eq!("UnexpectedArgumentType(Int.add, Bool)", scope.exec("2 + true"));
        // to be supported later
        assert_eq!("UnexpectedArgumentType(Int.mul, Float)", scope.exec("2 * 0.1"));
    }

    #[test]
    fn test_binaries() {
        let mut scope = Scope::init();
        scope.exec("val a = 1");
        scope.exec("val b = 2");
        assert_eq!("true", scope.exec("a == a"));
        assert_eq!("true", scope.exec("1 == a"));
        assert_eq!("false", scope.exec("a == b"));
        assert_eq!("true", scope.exec("a != b"));
        assert_eq!("true", scope.exec("a == 1 && b == 2"));
        assert_eq!("false", scope.exec("a == 1 && b == 1"));
        assert_eq!("false", scope.exec("a == 2 && b == 2"));
        assert_eq!("true", scope.exec("a < b"));
        assert_eq!("false", scope.exec("a >= b"));
    }

    #[test]
    fn test_if_else() {
        let mut scope = Scope::init();
        assert_eq!("1", scope.exec("if (true) 1 else 0"));
        assert_eq!("0", scope.exec("if (false) 1 else 0"));
        assert_eq!("1", scope.exec("if (true) { 1 } else { 0 }"));
        assert_eq!("0", scope.exec("if (false) { 1 }  else { 0 }"));
        assert_eq!("1", scope.exec("if (true) 1"));
        assert_eq!("nil", scope.exec("if (false) 1"));
    }

    #[test]
    fn test_print() {
        let mut scope = Scope::init();
        assert_eq!("nil", scope.exec("print(\"hello world\")"));
    }

    #[test]
    fn test_while() {
        let mut scope = Scope::init();
        scope.exec("var a = 0");
        assert_eq!("11", scope.exec("while (a < 10) { a = a + 1 }"));
    }

    #[test]
    fn test_fun() {
        let mut scope = Scope::init();
        assert_eq!("pi",  scope.exec("fun pi(): Float = 3.14"));
        assert_eq!("3.14", scope.exec("pi()"));

        assert_eq!("inc",  scope.exec("fun inc(a: Int): Int = { a + 1 }"));
        assert_eq!("3", scope.exec("inc(2)"));
    }
}
