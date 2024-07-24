use std::fmt::Debug;
use std::io;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Bool, Float, Fun, Int, Null, Symbol};
use crate::if_else;
use crate::scope::Scope;
use crate::types::Type;
use crate::types::Type::Macro;

use self::Function::{BuiltIn, Stateful, Stateless, Defined};

macro_rules! def {
    ($scope:expr, $name:expr, $types:expr, $lambda:expr) => {  $scope.add_fun(Fun($name.to_owned(), $types.clone(), $lambda)) };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Stateless(fn(&Vec<Expr>) -> Result<Expr, Exception>),
    Stateful(fn(&Vec<Expr>, &Scope) -> Result<Expr, Exception>),
    BuiltIn(fn(&Vec<Expr>, &mut Scope) -> Result<Expr, Exception>),
    Defined(Vec<String>, Box<Expr>),
}

impl Function {
    pub fn apply(&self, vec: &Vec<Expr>, scope: &Scope) -> Result<Expr, Exception> {
        match self {
            Stateless(f) => f(vec),
            Stateful(f) => f(vec, scope),
            Defined(params, body) => apply_defined(scope, body, params, vec),
            _ => panic!("Cannot apply a Mutating function"),
        }
    }
}

fn apply_defined(scope: &Scope, body: &Box<Expr>, params: &Vec<String>, vec: &Vec<Expr>) -> Result<Expr, Exception> {
    let mut local = scope.child();
    local.add_args(params, vec);
    body.mut_eval(&mut local)
}

pub fn add_functions(scope: &mut Scope) {
    // int arithmetics
    let types = Type::new("(Int,Int)->Int");
    def!(scope, "Int.add", types, Stateless(|vec| Ok(Int(vec[0].to_int()? + vec[1].to_int()?))));
    def!(scope, "Int.sub", types, Stateless(|vec| Ok(Int(vec[0].to_int()? - vec[1].to_int()?))));
    def!(scope, "Int.mul", types, Stateless(|vec| Ok(Int(vec[0].to_int()? * vec[1].to_int()?))));
    def!(scope, "Int.div", types, Stateless(|vec| divide_int(vec[0].to_int()?, vec[1].to_int()?)));
    def!(scope, "Int.mod", types, Stateless(|vec| modulo_int(vec[0].to_int()?, vec[1].to_int()?)));

    // float arithmetics
    let types = Type::new("(Float,Float)->Float");
    def!(scope, "Float.add", types, Stateless(|vec| Ok(Float(vec[0].to_float()? + vec[1].to_float()?))));
    def!(scope, "Float.sub", types, Stateless(|vec| Ok(Float(vec[0].to_float()? - vec[1].to_float()?))));
    def!(scope, "Float.mul", types, Stateless(|vec| Ok(Float(vec[0].to_float()? * vec[1].to_float()?))));
    def!(scope, "Float.div", types, Stateless(|vec| divide_float(vec[0].to_float()?, vec[1].to_float()?)));

    // boolean logic
    let types = Type::new("(Bool,Bool)->Bool");
    def!(scope, "Bool.and", types, Stateless(|vec| Ok(Bool(vec[0].to_bool()? && vec[1].to_bool()?))));
    def!(scope, "Bool.or", types, Stateless(|vec| Ok(Bool(vec[0].to_bool()? || vec[1].to_bool()?))));

    // comparisons
    let types = Type::new("(Int,Int)->Bool");
    def!(scope, "Int.eq", types, Stateless(|vec| Ok(Bool(vec[0].to_int()? == vec[1].to_int()?))));
    def!(scope, "Int.neq", types, Stateless(|vec| Ok(Bool(vec[0].to_int()? != vec[1].to_int()?))));
    def!(scope, "Int.gt", types, Stateless(|vec| Ok(Bool(vec[0].to_int()? > vec[1].to_int()?))));
    def!(scope, "Int.ge", types, Stateless(|vec| Ok(Bool(vec[0].to_int()? >= vec[1].to_int()?))));
    def!(scope, "Int.lt", types, Stateless(|vec| Ok(Bool(vec[0].to_int()? < vec[1].to_int()?))));
    def!(scope, "Int.le", types, Stateless(|vec| Ok(Bool(vec[0].to_int()? <= vec[1].to_int()?))));

    // String functions
    def!(scope, "Str.read", Type::new("(Str)->Expr"), Stateful(|vec, scope| Ok(scope.read(vec[0].to_str()?))));
    def!(scope, "Str.trim", Type::new("(Str)->Str"), Stateless(|vec| Ok(Expr::Str(vec[0].to_str()?.trim().to_owned()))));

    // IO functions
    def!(scope, "readLine", Type::new("()->Any"), Stateless(|_| read_line()));
    def!(scope, "print", Type::new("(List<Any>)->Any"), Stateless(|vec,| print(vec)));
    def!(scope, "eval", Type::new("(Any)->Any"), Stateful(|vec, scope| vec[0].eval(scope)));

    // special functions
    def!(scope, "var", Macro, BuiltIn(|vec, scope| declare(vec[0].to_symbol()?, vec[1].to_type()?, vec[2].mut_eval(scope)?, scope, true)));
    def!(scope, "val", Macro, BuiltIn(|vec, scope| declare(vec[0].to_symbol()?, vec[1].to_type()?, vec[2].mut_eval(scope)?, scope, false)));
    def!(scope, "fun", Macro, BuiltIn(|vec, scope| define(vec[0].to_symbol()?, vec[1].to_params()?, vec[2].to_type()?, &vec[3], scope)));
    def!(scope, "assign", Macro, BuiltIn(|vec, scope| assign(vec[0].to_symbol()?, vec[1].mut_eval(scope)?, scope)));
    def!(scope, "while", Macro, BuiltIn(|vec, scope| run_while(&vec[0], vec, scope)));
    def!(scope, "if", Macro, BuiltIn(|vec, scope| if_else!(vec[0].mut_eval(scope)?.to_bool()?, vec[1].mut_eval(scope),vec[2].mut_eval(scope))));
}

fn divide_int(a: &i64, b: &i64) ->  Result<Expr, Exception> {
    if *b != 0 { Ok(Int(a/b)) } else { Err(Exception::DivisionByZero)}
}
fn modulo_int(a: &i64, b: &i64) ->  Result<Expr, Exception> {
    if *b != 0 { Ok(Int(a % b)) } else { Err(Exception::DivisionByZero)}
}
fn divide_float(a: &f64, b: &f64) ->  Result<Expr, Exception> {
    if *b != 0.0 { Ok(Float(a/b)) } else { Err(Exception::DivisionByZero)}
}


fn declare(name: &str, expected: Type, value: Expr, scope: &mut Scope, is_mutable: bool) -> Result<Expr, Exception> {
    if expected != Type::Any && expected != value.get_type()  {
        Err(Exception::UnexpectedType(value.get_type().to_string()))
    } else if scope.is_defined(&name) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        scope.set(name, value, Some(is_mutable));
        Ok(Symbol(name.to_owned()))
    }
}

fn define(name: &str, params: &Vec<(String, Type)>, output: Type, expr: &Expr, scope: &mut Scope) -> Result<Expr, Exception> {
    if scope.is_defined(&name) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        let types = Type::Fun(params.iter().map(|p| p.1.clone()).collect(), Box::new(output.clone()));
        scope.add_fun(Fun(name.to_owned(), types, Defined(params.iter().map(|p| p.0.clone()).collect(), Box::new(expr.as_block()))));
        Ok(Symbol(name.to_owned()))
    }
}


fn assign(name: &str, value: Expr, scope: &mut Scope) -> Result<Expr, Exception> {
    match scope.is_mutable(&name) {
        None  => Err(Exception::NotDefined(name.to_owned())),
        Some(false) => Err(Exception::NotMutable(name.to_owned())),
        _ if scope.get_type(name) != value.get_type() => Err(Exception::UnexpectedType(value.get_type().to_string())),
        _ => {
            scope.set(name, value.clone(), None);
            Ok(value)
        }
    }
}


fn print(vec: &Vec<Expr>) -> Result<Expr, Exception> {
    for x in vec { print!("{}", x) }
    println!();
    Ok(Null)
}

fn run_while(cond: &Expr, body: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    let mut count = 0;
    let mut result = Ok(Null);
    loop {
        count += 1;
        if count >= 1000000 {
            break Err(Exception::InfiniteLoop)
        }
        if let Bool(bool) = cond.eval(scope)? {
            if bool {
                for e in body {
                    result = e.mut_eval(scope);
                }
            } else {
                break result;
            }
        } else {
            break Err(Exception::NotA(Type::Bool.to_string(), cond.to_string()))
        }
    }
}

fn read_line() -> Result<Expr, Exception> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Err(_) => return Err(Exception::IOError),
        _ => {}
    }
    Ok(Expr::Str(line))
}

