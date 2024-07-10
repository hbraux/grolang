use std::cmp::PartialEq;

use strum_macros::EnumString;

use crate::{Expr, Scope};
use crate::exception::Exception;
use crate::Expr::{Bool, Float, Int};
use crate::expr::{FALSE, TRUE};
use crate::expr::Expr::{Str, Symbol, TypeSpec};

#[derive(Debug, Clone, PartialEq, EnumString)]
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
    Var,
    Val
}


impl BuiltIn {
    fn call_args(&self) -> usize {
        match self {
            BuiltIn::ToStr => 1,
            _ => 2
        }
    }
    pub fn apply(&self, args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
        if args.len() < self.call_args() {
            return Err(Exception::WrongArgumentsNumber(self.call_args(), args.len()))
        }
        match self {
            BuiltIn::ToStr => self.unitary_op(args[0].clone().eval(scope)?),
            BuiltIn::Add | BuiltIn::Sub | BuiltIn::Mul | BuiltIn::Div | BuiltIn::Mod => self.arithmetic_op(args[0].eval(scope)?, args[1].eval(scope)?),
            BuiltIn::Eq | BuiltIn::Neq | BuiltIn::Ge | BuiltIn::Gt | BuiltIn::Le | BuiltIn::Lt => self.comparison_op(args[0].eval(scope)?, args[1].eval(scope)?),
            BuiltIn::And | BuiltIn::Or => self.binary_op(args[0].clone(), args[1].clone(), scope),
            BuiltIn::Var => assign(args, scope, Some(true)),
            BuiltIn::Val => assign(args, scope, Some(false)),
            BuiltIn::Set => assign(args, scope, None),
            BuiltIn::If => if_else(args, scope),
            _ => panic!("operator {:?} not yet implemented", self),
        }
    }

    fn unitary_op(&self, expr: Expr) -> Result<Expr, Exception> {
        match self {
            BuiltIn::ToStr => Ok(Str(expr.print())),
            _ => panic!("unexpected operator {:?}", self),
        }
    }

    fn arithmetic_op(&self, left: Expr, right: Expr) -> Result<Expr, Exception> {
        match (left, right) {
            (Int(a), Int(b))    =>  self.arithmetic_int(a, b),
            (Float(a), Float(b)) => self.arithmetic_float(a, b),
            (Int(a), Float(b))  => self.arithmetic_float(a as f64, b),
            (Float(a), Int(b))  => self.arithmetic_float(a, b as f64),
            _ => Err(Exception::NotNumber),
        }
    }
    fn arithmetic_int(&self, a: i64, b: i64) -> Result<Expr, Exception> {
        match self {
            BuiltIn::Add => Ok(Int(a + b)),
            BuiltIn::Sub => Ok(Int(a - b)),
            BuiltIn::Mul => Ok(Int(a * b)),
            BuiltIn::Mod => Ok(Int(a % b)),
            BuiltIn::Div => if b != 0 { Ok(Int(a / b)) } else { Err(Exception::DivisionByZero) }
            _ => panic!("unexpected operator {:?}", self),
        }
    }
    fn arithmetic_float(&self, a: f64, b: f64) -> Result<Expr, Exception> {
        match self {
            BuiltIn::Add => Ok(Float(a + b)),
            BuiltIn::Sub => Ok(Float(a - b)),
            BuiltIn::Mul => Ok(Float(a * b)),
            BuiltIn::Mod => Ok(Float(a % b)),
            BuiltIn::Div => if b != 0.0 { Ok(Float(a / b)) } else { Err(Exception::DivisionByZero) }
            _ => panic!("unexpected operator {:?}", self),
        }
    }
    fn comparison_op(&self, left: Expr, right: Expr) ->  Result<Expr, Exception> {
        match self {
            BuiltIn::Eq => Ok(Bool(left.eq(&right))),
            BuiltIn::Neq => Ok(Bool(!left.eq(&right))),
            _ => panic!("unexpected operator {:?}", self),
        }
    }
    fn binary_op(&self, left: Expr, right: Expr, scope: &mut Scope) -> Result<Expr, Exception> {
        match (self, left.eval(scope)?) {
            (BuiltIn::And, FALSE) => Ok(FALSE),
            (BuiltIn::Or, TRUE) => Ok(TRUE),
            (BuiltIn::And, TRUE) => Ok(right.clone().eval(scope)?.to_bool()?),
            (BuiltIn::Or, FALSE) => Ok(right.clone().eval(scope)?.to_bool()?),
            _ => panic!("unexpected operator {:?}", self),
        }
    }
}

fn assign(args: &Vec<Expr>, scope: &mut Scope, is_mutable: Option<bool>) -> Result<Expr, Exception> {
    if let Symbol(name) = &args[0] {
        let value = &args[args.len() - 1];
        if let TypeSpec(expected) = &args[1] {
            if value.get_type() != *expected {
                return Err(Exception::InconsistentType(value.get_type().to_string()))
            }
        }
        scope.store(&name, value, is_mutable)
    } else {
        Err(Exception::NotSymbol(args[0].to_string()))
    }
}

fn if_else(args: &Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
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
