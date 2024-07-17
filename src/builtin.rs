use std::cmp::PartialEq;

use strum_macros::{Display, EnumString};

use crate::{Expr, Scope};
use crate::exception::Exception;
use crate::Expr::{Bool, Float, Int};
use crate::expr::{FALSE, TRUE};
use crate::expr::Expr::{Nil, Str, Symbol, TypeSpec};

use self::BuiltIn::{Add, And, Div, Eq, Ge, Gt, If, Le, Lt, Mod, Mul, Neq, Or, Print, Set, Sub, ToStr, Val, Var, While};

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum BuiltIn {
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
    In,
    Or,
    And,
    ToStr,
    Set,
    If,
    While,
    Read,
    Print,
    Var,
    Val
}




impl BuiltIn {
    fn call_args(&self) -> usize {
        match self {
            Print => 0,
            ToStr => 1,
            If | Var | Val => 3,
            _ => 2
        }
    }
    pub fn call(&self, args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
        if self.call_args() > 0 && self.call_args() != args.len() {
            return Err(Exception::WrongArgumentsNumber("call".to_owned(), self.call_args(), args.len()))
        }
        match self {
            ToStr => self.unitary_op(args[0].clone().eval(scope)?),
            Add | Sub | Mul | Div | Mod => self.arithmetic_op(&args[0].eval(scope)?, &args[1].eval(scope)?),
            Eq | Neq | Ge | Gt | Le | Lt => self.comparison_op(args[0].eval(scope)?, args[1].eval(scope)?),
            And | Or => self.binary_op(args[0].clone(), args[1].clone(), scope),
            Var => call_assign(args, scope, Some(true)),
            Val => call_assign(args, scope, Some(false)),
            Set => call_assign(args, scope, None),
            Print => call_print(args, scope),
            If => call_if(args, scope),
            While => call_while(args, scope),
            _ => Err(Exception::NotImplemented(self.to_string()))
        }
    }

    fn unitary_op(&self, expr: Expr) -> Result<Expr, Exception> {
        match self {
            ToStr => Ok(Str(expr.print())),
            _ => panic!("unexpected operator {:?}", self),
        }
    }

    fn arithmetic_op(&self, left: &Expr, right: &Expr) -> Result<Expr, Exception> {
        match (left, right) {
            (Int(a), Int(b))    =>  self.arithmetic_int(*a, *b),
            (Float(a), Float(b)) => self.arithmetic_float(*a, *b),
            (Int(a), Float(b))  => self.arithmetic_float(*a as f64, *b),
            (Float(a), Int(b))  => self.arithmetic_float(*a, *b as f64),
            _ => Err(Exception::NotNumber),
        }
    }
    fn arithmetic_int(&self, a: i64, b: i64) -> Result<Expr, Exception> {
        match self {
            Add => Ok(Int(a + b)),
            Sub => Ok(Int(a - b)),
            Mul => Ok(Int(a * b)),
            Mod => Ok(Int(a % b)),
            Div => if b != 0 { Ok(Int(a / b)) } else { Err(Exception::DivisionByZero) }
            _ => panic!("unexpected operator {:?}", self),
        }
    }
    fn arithmetic_float(&self, a: f64, b: f64) -> Result<Expr, Exception> {
        match self {
            Add => Ok(Float(a + b)),
            Sub => Ok(Float(a - b)),
            Mul => Ok(Float(a * b)),
            Mod => Ok(Float(a % b)),
            Div => if b != 0.0 { Ok(Float(a / b)) } else { Err(Exception::DivisionByZero) }
            _ => panic!("unexpected operator {:?}", self),
        }
    }
    fn comparison_op(&self, left: Expr, right: Expr) ->  Result<Expr, Exception> {
        if matches!(left, Int(_)) || matches!(left, Float(_)) {
           self.compare_numbers(left, right)
        } else {
            match self {
                Eq => Ok(Bool(left.eq(&right))),
                Neq => Ok(Bool(!left.eq(&right))),
                _ => panic!("unexpected operator {:?}", self),
            }
        }
    }
    fn compare_numbers(&self, left: Expr, right: Expr) ->  Result<Expr, Exception> {
        match (left, right) {
            (Int(a), Int(b))    =>  self.compare_floats(a as f64, b as f64),
            (Float(a), Float(b)) => self.compare_floats(a, b),
            (Int(a), Float(b))  => self.compare_floats(a as f64, b),
            (Float(a), Int(b))  => self.compare_floats(a, b as f64),
            _ => Err(Exception::NotNumber),
        }
    }
    fn compare_floats(&self, a: f64, b: f64) -> Result<Expr, Exception> {
        let b = match self {
            Eq => a == b,
            Neq => a != b,
            Ge => a >= b,
            Gt => a > b,
            Le => a <= b,
            Lt => a < b,
            _ => panic!("unexpected operator {:?}", self),
        };
        Ok(Bool(b))
    }

    fn binary_op(&self, left: Expr, right: Expr, scope: &mut Scope) -> Result<Expr, Exception> {
        match (self, left.eval(scope)?) {
            (And, FALSE) => Ok(FALSE),
            (Or, TRUE) => Ok(TRUE),
            (And, TRUE) => Ok(right.clone().eval(scope)?.to_bool()?),
            (Or, FALSE) => Ok(right.clone().eval(scope)?.to_bool()?),
            _ => panic!("unexpected operator {:?}", self),
        }
    }
}

fn call_assign(args: &Vec<Expr>, scope: &mut Scope, is_mutable: Option<bool>) -> Result<Expr, Exception> {
    if let Symbol(name) = &args[0] {
        let value = (&args[args.len() - 1]).eval(scope)?;
        if let TypeSpec(expected) = &args[1] {
            if value.get_type() != *expected {
                return Err(Exception::InconsistentType(value.get_type().to_string()))
            }
        }
        scope.store(name.to_owned(), value, is_mutable)
    } else {
        Err(Exception::NotSymbol(args[0].to_string()))
    }
}

fn call_if(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    if let Bool(bool) = args[0].eval(scope)? {
        if bool {
            args[1].clone().eval(scope)
        } else {
            args[2].clone().eval(scope)
        }
    } else {
        Err(Exception::NotBoolean(args[0].to_string()))
    }
}


fn call_print(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    for x in args {
        print!("{:?}", x.eval(scope)?)
    }
    Ok(Nil)
}

fn call_while(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
    let mut count = 0;
    let mut result = Ok(Nil);
    loop {
        count += 1;
        if count >= 1000000 {
           break Err(Exception::InfiniteLoop)
        }
        if let Bool(bool) = args[0].eval(scope)? {
            if bool {
                result = args[1].eval(scope)
            } else {
                break result;
            }
        } else {
            break Err(Exception::NotBoolean(args[0].to_string()))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call() {
        let mut scope = Scope::new();
        assert_eq!(Err(Exception::WrongArgumentsNumber("call".to_owned(), 2, 1)), Lt.call(&vec!(Nil), &mut scope))
    }
}
