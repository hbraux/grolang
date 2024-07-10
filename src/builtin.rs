use std::cmp::PartialEq;
use strum_macros::EnumString;
use crate::{Scope, Expr};
use crate::errors::Exception;
use crate::Expr::{Bool, Float, Int};
use crate::expr::Expr::{Error, Str, Symbol, TypeSpec};
use crate::expr::{FALSE,TRUE};


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
    pub fn apply(&self, args: Vec<Expr>, scope: &mut Scope) -> Result<Expr, Exception> {
        if args.len() < self.call_args() {
            return Err(Exception::WrongArgumentsNumber(self.call_args(), args.len())
        }
        match self {
            BuiltIn::ToStr => self.unitary_op(args[0].clone(), scope),
            BuiltIn::Add | BuiltIn::Sub | BuiltIn::Mul | BuiltIn::Div | BuiltIn::Mod => self.arithmetic_op(args[0].clone().eval(scope), args[1].clone().eval(scope)),
            BuiltIn::Eq | BuiltIn::Neq | BuiltIn::Ge | BuiltIn::Gt | BuiltIn::Le | BuiltIn::Lt => self.comparison_op(args[0].clone().eval(scope), args[1].clone().eval(scope)),
            BuiltIn::And | BuiltIn::Or => self.binary_op(args[0].clone(), args[1].clone(), scope),
            BuiltIn::Var => store(scope, args, Some(true)),
            BuiltIn::Val => store(scope, args, Some(false)),
            BuiltIn::Set => store(scope, args, None),
            _ => panic!(),
        }
    }

    fn unitary_op(&self, expr: Expr, scope: &mut Scope) -> Result<Expr, Exception> {
        match self {
            BuiltIn::ToStr => Ok(Str(expr.print())),
            _ => panic!(),
        }
    }

    fn arithmetic_op(&self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Int(a), Int(b))    =>  self.calc_int(a, b),
            (Float(a), Float(b)) => self.calc_float(a, b),
            (Int(a), Float(b))  => self.calc_float(a as f64, b),
            (Float(a), Int(b))  => self.calc_float(a, b as f64),
            _ => panic!(),
        }
    }
    fn calc_int(&self, a: i64, b: i64) -> Expr {
        match self {
            BuiltIn::Add => Int(a + b),
            BuiltIn::Sub => Int(a - b),
            BuiltIn::Mul => Int(a * b),
            BuiltIn::Mod => Int(a % b),
            BuiltIn::Div => if b != 0 { Int(a / b) } else { Error(Exception::DivisionByZero) }
            _ => panic!(),
        }
    }
    fn calc_float(&self, a: f64, b: f64) -> Expr {
        match self {
            BuiltIn::Add => Float(a + b),
            BuiltIn::Sub => Float(a - b),
            BuiltIn::Mul => Float(a * b),
            BuiltIn::Mod => Float(a % b),
            BuiltIn::Div => if b != 0.0 { Float(a / b) } else { Error(Exception::DivisionByZero) }
            _ => panic!(),
        }
    }
    fn comparison_op(&self, left: Expr, right: Expr) -> Expr {
        let result = match self {
            BuiltIn::Eq => left.eq(&right),
            BuiltIn::Neq => !left.eq(&right),
            _ => panic!("no yet implemented"),
        };
        Bool(result)
    }
    fn binary_op(&self, left: Expr, right: Expr, ctx: &mut Scope) -> Result<Expr, Exception> {
        match (self, left) {
            (BuiltIn::And, FALSE) => Ok(FALSE),
            (BuiltIn::Or, TRUE) => Ok(TRUE),
            (BuiltIn::And, TRUE) => right.clone().eval(ctx).to_bool(),
            (BuiltIn::Or, FALSE) => right.clone().eval(ctx).to_bool(),
            _ => panic!("not boolean"),
        }
    }
}

// TODO: move to scope
fn store(ctx: &mut Scope, args: Vec<Expr>, is_mutable: Option<bool>) -> Result<Expr, Exception> {
    if let Symbol(name) = &args[0] {
        let value = &args[args.len() -1];
        if let TypeSpec(expected) = &args[1] {
            if value.get_type() != *expected {
                return Err(Exception::InconsistentType(value.get_type().to_string())))
            }
        }
        let is_defined = ctx.is_defined(&name);
        if is_mutable.is_some() && is_defined {
            Err(Exception::AlreadyDefined(name.to_owned()))
        } else if is_mutable.is_none() && !is_defined {
            Err(Exception::NotDefined(name.to_owned()))
        } else if is_mutable.is_none() && ctx.get_type(&name) != value.get_type() {
            Err(Exception::InconsistentType(value.get_type().to_string()))
        } else if is_mutable.is_none() && ctx.is_mutable(&name) {
            Err(Exception::NotMutable(value.get_type().to_string()))
        } else {
            ctx.set(&name, value, is_mutable);
            Ok(value.clone())
        }
    } else {
        panic!("{} is not an id", args[0])
    }
}
