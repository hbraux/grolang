use std::collections::VecDeque;
use std::str::from_utf8;

use dialoguer::{Input, theme::ColorfulTheme};
use rust_embed::Embed;
use sys_locale::get_locale;

use crate::scope::Scope;

mod parser;
mod types;
mod exception;
mod functions;
mod expr;
mod scope;

#[macro_export]
macro_rules! if_else {
    ($condition:expr,  $true_branch:expr, $false_branch:expr) => { if $condition { $true_branch } else { $false_branch }};
}

pub const LANG: &str = "GroLang";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const RED: &str = "\x1b[1;31m";
const BLUE: &str = "\x1b[1;34m";
const STD: &str = "\x1b[0m";


#[derive(Embed)]
#[folder = "resources/"]
struct Asset;



#[derive(Debug, Default)]
pub struct History {
    deque: VecDeque<String>,
}
impl History {
    fn print(&self) {
        self.deque.iter().for_each(|e| println!("# {}", e))
    }

}

impl<T: ToString> dialoguer::History<T> for History {
    fn read(&self, pos: usize) -> Option<String> {
        self.deque.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        let val = val.to_string();
        if !val.starts_with(":") {
            self.deque.push_front(val);
        }
    }
}

fn get_resource(name: &str) -> String {
    let locale = get_locale().unwrap_or_else(|| String::from("fr-FR"));
    let lang = &locale[0..2];
    let asset = Asset::get(&format!("{}_{}.txt", name, lang)).expect(&format!("No help file for language {}", lang));
    let str = from_utf8(asset.data.as_ref()).expect("Invalid resource file");
    return str.to_owned();
}

pub fn repl() {
    let mut debug = false;
    let help = get_resource("help");
    let repl: Vec<String> = get_resource("repl").split("\n").map(|s| s.to_owned()).collect();
    println!("{BLUE}{LANG} Version {VERSION}{STD}\n{}\n", repl[0]);
    let mut scope = Scope::init();
    let mut history = History::default();
    loop {
        //let autocomplete = AutoComplete::new(&scope);
        let input = Input::<String>::with_theme(&ColorfulTheme::default())
            .completion_with(&scope)
            .history_with(&mut history)
            .interact_text().expect("Unable to read stdin");

        if input.starts_with(':') {
            match input[1..2].to_string().as_str() {
                "q" => break,
                "d" => { debug = !debug; println!("# debug={}", debug) },
                "h" => history.print(),
                _ => println!("{}", help),
            }
            continue;
        }
        let expr = scope.read(&input);
        if expr.is_failure() {
            println!("{RED}{} ({:?}){STD}", repl[1], expr.to_exception().unwrap());
            continue;
        }
        let result = expr.eval_or_failed(&mut scope);
        if expr.is_failure() {
            println!("{RED}{} ({:?}){STD}", repl[2], expr.to_exception().unwrap());
        } else {
            println!("{}", result.print())
        }
    }
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
        assert_eq!("null", scope.exec("null"));
        assert_eq!("\"abc\"", scope.exec("\"abc\""));
        assert_eq!("[1,2,3]", scope.exec("[1,2,3]"));
        assert_eq!("{\"a\":1,\"b\":2}", scope.exec("{\"a\":1,\"b\":2}"));
    }

    #[test]
    fn test_variables() {
        let mut scope = Scope::init();
        assert_eq!("a", scope.exec("var a = 1"));
        assert_eq!("1", scope.exec("a"));
        assert_eq!("AlreadyDefined(a)", scope.exec("var a = 3"));
        assert_eq!("2", scope.exec("a = a + 1"));
        assert_eq!("0", scope.exec("a.assign(0)"));
        assert_eq!("UnexpectedType(Float)", scope.exec("a = 3.0"));
        assert_eq!("c", scope.exec("val c=3.2"));
        assert_eq!("UnexpectedType(Float)", scope.exec("var d: Int = 3.2"));
        assert_eq!("3.2", scope.exec("c"));
        assert_eq!("i", scope.exec("val i = 0"));
        assert_eq!("NotMutable(i)", scope.exec("i = 1"));
        assert_eq!("NotDefined(z)", scope.exec("z = 0"));

    }

    #[test]
    fn test_collections() {
        let mut scope = Scope::init();
        scope.exec("val l = [1,2,3]");
        scope.exec(r#"val m = {"a":1, "b":1}"#);
        assert_eq!( "\"List<Int>\"", scope.exec("l.type()"));
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
        assert_eq!("null", scope.exec("if (false) 1"));
    }

    #[test]
    fn test_print() {
        let mut scope = Scope::init();
        assert_eq!("null", scope.exec("print(2, \"hello world\")"));
    }

    #[test]
    fn test_read_eval() {
        let mut scope = Scope::init();
        scope.exec("val n = 10");
        assert_eq!("n", scope.exec(r#""n".read()"#));
        assert_eq!("10", scope.exec(r#"read("n").eval() "#));
    }


    #[test]
    fn test_while() {
        let mut scope = Scope::init();
        scope.exec("var a = 0");
        assert_eq!("11", scope.exec("while (a <= 10) { a = a + 1 }"));
    }

    #[test]
    fn test_exceptions() {
        let mut scope = Scope::init();
        assert_eq!("UndefinedFunction(read)", scope.exec("read()"));
        assert_eq!("UndefinedMethod(Int.inc)", scope.exec("inc(2)"));
        assert_eq!("UndefinedSymbol(n)", scope.exec("n.eval()"));
    }

    #[test]
    fn test_functions() {
        let mut scope = Scope::init();
        assert_eq!("pi",  scope.exec("fun pi(): Float = 3.14"));
        assert_eq!("3.14", scope.exec("pi()"));

        scope.exec("fun dec(a: Int): Int = a - 1");
        assert_eq!("1", scope.exec("dec(2)"));

        scope.exec("fun inc(a: Int): Int = { a + 1 }");
        assert_eq!("3", scope.exec("inc(2)"));

        scope.exec("fun zero(): Int = { val x = 0 ; x }");
        assert_eq!("0", scope.exec("zero()"));

        scope.exec(r#"fun fact(n: Int): Int = { if (n <= 1) 1 else n*fact(n-1)}"#);
        assert_eq!("1", scope.exec("fact(0)"));
        assert_eq!("24", scope.exec("fact(4)"));
    }


}
