use std::fmt::Debug;
use std::io;

use crate::exception::Exception;
use crate::expr::Expr;
use crate::expr::Expr::{Bool, Float, Fun, Int, Null, Symbol};
use crate::if_else;
use crate::scope::Scope;
use crate::types::Type;

use self::Function::{BuiltIn, Defined, Stateful, Stateless};

macro_rules! def {
    ($scope:expr, $name:expr, $types:expr, $lambda:expr) => {  $scope.add_fun(Fun($name.to_owned(), Type::parse($types).unwrap(), $lambda)) };
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
    body.eval_mutable(&mut local)
}

pub fn add_functions(sc: &mut Scope) {
    // arithmetics
    let types = "(Number,Number)->Number";
    def!(sc, "Number.add", types, Stateless(|vec| NumberFun::Add.eval(&vec[0], &vec[1])));
    def!(sc, "Number.sub", types, Stateless(|vec| NumberFun::Sub.eval(&vec[0], &vec[1])));
    def!(sc, "Number.mul", types, Stateless(|vec| NumberFun::Mul.eval(&vec[0], &vec[1])));
    def!(sc, "Number.div", types, Stateless(|vec| NumberFun::Div.eval(&vec[0], &vec[1])));
    def!(sc, "Number.mod", types, Stateless(|vec| NumberFun::Mod.eval(&vec[0], &vec[1])));
    // comparisons
    let types = "(Number,Number)->Bool";
    def!(sc, "Number.eq", types, Stateless(|vec| NumberFun::Eq.eval(&vec[0], &vec[1])));
    def!(sc, "Number.neq", types, Stateless(|vec| NumberFun::Neq.eval(&vec[0], &vec[1])));
    def!(sc, "Number.ge", types, Stateless(|vec| NumberFun::Ge.eval(&vec[0], &vec[1])));
    def!(sc, "Number.gt", types, Stateless(|vec| NumberFun::Gt.eval(&vec[0], &vec[1])));
    def!(sc, "Number.lt", types, Stateless(|vec| NumberFun::Lt.eval(&vec[0], &vec[1])));
    def!(sc, "Number.le", types, Stateless(|vec| NumberFun::Le.eval(&vec[0], &vec[1])));

    // boolean operators
    let types = "(Bool,Bool)->Bool";
    def!(sc, "Bool.and", types, Stateless(|vec| Ok(Bool(vec[0].to_bool()? && vec[1].to_bool()?))));
    def!(sc, "Bool.or", types, Stateless(|vec| Ok(Bool(vec[0].to_bool()? || vec[1].to_bool()?))));


    // String functions
    def!(sc, "Str.read", "(Str)->Expr", Stateful(|vec, scope| Ok(scope.read(vec[0].to_str()?))));
    def!(sc, "Str.trim", "(Str)->Str", Stateless(|vec| Ok(Expr::Str(vec[0].to_str()?.trim().to_owned()))));

    // IO functions
    def!(sc, "readLine", "()->Any", Stateless(|_| read_line()));
    def!(sc, "print", "(List<Any>)->Any", Stateless(|vec,| print(vec)));
    def!(sc, "eval", "(Any)->Any", Stateful(|vec, scope| vec[0].eval(scope)));

    // Misc functions
    def!(sc, "type", "(Any)->Str", Stateless(|vec| Ok(Expr::Str(vec[0].get_type().to_string()))));

    // macros
    def!(sc, "const", "Macro", BuiltIn(|vec, scope| def_variable(vec[0].to_symbol()?, vec[2].eval(scope)?.expect(vec[1].to_type()?)?, scope, None)));
    def!(sc, "var", "Macro", BuiltIn(|vec, scope| def_variable(vec[0].to_symbol()?, vec[2].eval(scope)?.expect(vec[1].to_type()?)?, scope, Some(true))));
    def!(sc, "val", "Macro", BuiltIn(|vec, scope| def_variable(vec[0].to_symbol()?, vec[2].eval(scope)?.expect(vec[1].to_type()?)?, scope, Some(false))));
    def!(sc, "fun", "Macro", BuiltIn(|vec, scope| def_function(vec[0].to_symbol()?, vec[1].to_params()?, vec[2].to_type()?, &vec[3], scope)));
    def!(sc, "struct", "Macro", BuiltIn(|vec, scope| def_struct(vec[0].to_symbol()?, vec[1].to_params()?, scope)));
    def!(sc, "assign", "Macro", BuiltIn(|vec, scope| assign(vec[0].to_symbol()?, vec[1].eval_mutable(scope)?, scope)));
    def!(sc, "while", "Macro", BuiltIn(|vec, scope| run_while(&vec[0], vec, scope)));
    def!(sc, "if", "Macro", BuiltIn(|vec, scope| if_else!(vec[0].eval_mutable(scope)?.to_bool()?, vec[1].eval_mutable(scope),vec[2].eval_mutable(scope))));

}


fn def_variable(name: &str, value: Expr, scope: &mut Scope, is_mutable: Option<bool>) -> Result<Expr, Exception> {
    if scope.is_defined(&name, is_mutable.is_none()) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        scope.set(name, value, is_mutable);
        Ok(Symbol(name.to_owned()))
    }
}

fn def_function(name: &str, params: &Vec<(String, Type)>, output: Type, expr: &Expr, scope: &mut Scope) -> Result<Expr, Exception> {
    if scope.is_defined(&name, name.contains(".")) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        let types = Type::Fun(params.iter().map(|p| p.1.clone()).collect(), Box::new(output.clone()));
        scope.add_fun(Fun(name.to_owned(), types, Defined(params.iter().map(|p| p.0.clone()).collect(), Box::new(expr.as_block()))));
        Ok(Symbol(name.to_owned()))
    }
}

fn def_struct(name: &str, params: &Vec<(String, Type)>, scope: &mut Scope) -> Result<Expr, Exception> {
    if scope.is_defined(&name, true) {
        Err(Exception::AlreadyDefined(name.to_owned()))
    } else {
        scope.set(name, Expr::Class(params.clone()), None);
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

#[derive(Debug)]
pub enum NumberFun {
    Mul,
    Div,
    Add,
    Sub,
    Mod,
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
}
impl NumberFun {
    fn eval(&self, left: &Expr, right: &Expr) -> Result<Expr, Exception> {
        match (left, right) {
            (Int(a), Int(b))    =>  self.eval_int(*a, *b),
            (Float(a), Float(b)) => self.eval_float(*a, *b),
            (Int(a), Float(b))  => self.eval_float(*a as f64, *b),
            (Float(a), Int(b))  => self.eval_float(*a, *b as f64),
            _ => Err(Exception::NotA("Number".to_owned(), left.to_string())),
        }
    }
    fn eval_int(&self, a: i64, b: i64) -> Result<Expr, Exception> {
        match self {
            NumberFun::Add => Ok(Int(a + b)),
            NumberFun::Sub => Ok(Int(a - b)),
            NumberFun::Mul => Ok(Int(a * b)),
            NumberFun::Mod => if_else!(b != 0, Ok(Int(a % b)), Err(Exception::DivisionByZero)),
            NumberFun::Div => if_else!(b != 0, Ok(Int(a / b)), Err(Exception::DivisionByZero)),
            NumberFun::Eq => Ok(Bool(a == b)),
            NumberFun::Neq => Ok(Bool(a != b)),
            NumberFun::Gt => Ok(Bool(a > b)),
            NumberFun::Ge => Ok(Bool(a >= b)),
            NumberFun::Lt => Ok(Bool(a < b)),
            NumberFun::Le => Ok(Bool(a <= b)),
        }
    }
    fn eval_float(&self, a: f64, b: f64) -> Result<Expr, Exception> {
        match self {
            NumberFun::Add => Ok(Float(a + b)),
            NumberFun::Sub => Ok(Float(a - b)),
            NumberFun::Mul => Ok(Float(a * b)),
            NumberFun::Mod => if_else!(b != 0.0, Ok(Float(a % b)), Err(Exception::DivisionByZero)),
            NumberFun::Div => if_else!(b != 0.0, Ok(Float(a / b)), Err(Exception::DivisionByZero)),
            NumberFun::Eq => Ok(Bool(a == b)),
            NumberFun::Neq => Ok(Bool(a != b)),
            NumberFun::Gt => Ok(Bool(a > b)),
            NumberFun::Ge => Ok(Bool(a >= b)),
            NumberFun::Lt => Ok(Bool(a < b)),
            NumberFun::Le => Ok(Bool(a <= b)),
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
                    result = e.eval_mutable(scope);
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

