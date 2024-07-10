use std::cmp::PartialEq;
use strum_macros::EnumString;
use crate::{Scope, Expr};
use crate::fail::Fail;
use crate::Expr::{Bool, Float, Int};
use crate::expr::Expr::{Failure, Str, Symbol, TypeSpec};
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
    pub fn apply(&self, args: Vec<Expr>, scope: &mut Scope) -> Result<Expr, Fail> {
        if args.len() < self.call_args() {
            return Err(Fail::WrongArgumentsNumber(self.call_args(), args.len()))
        }
        match self {
            BuiltIn::ToStr => self.unitary_op(args[0].clone().eval(scope)?),
            BuiltIn::Add | BuiltIn::Sub | BuiltIn::Mul | BuiltIn::Div | BuiltIn::Mod => self.arithmetic_op(args[0].clone().eval(scope)?, args[1].clone().eval(scope)?),
            BuiltIn::Eq | BuiltIn::Neq | BuiltIn::Ge | BuiltIn::Gt | BuiltIn::Le | BuiltIn::Lt => self.comparison_op(args[0].clone().eval(scope)?, args[1].clone().eval(scope)?),
            BuiltIn::And | BuiltIn::Or => self.binary_op(args[0].clone(), args[1].clone(), scope),
            BuiltIn::Var => store(scope, args, Some(true)),
            BuiltIn::Val => store(scope, args, Some(false)),
            BuiltIn::Set => store(scope, args, None),
            _ => panic!("operator {} not yet implemented", self),
        }
    }

    fn unitary_op(&self, expr: Expr) -> Result<Expr, Fail> {
        match self {
            BuiltIn::ToStr => Ok(Str(expr.print())),
            _ => panic!("unexpected operator {}", self),
        }
    }

    fn arithmetic_op(&self, left: Expr, right: Expr) -> Result<Expr, Fail> {
        match (left, right) {
            (Int(a), Int(b))    =>  self.arithmetic_int(a, b),
            (Float(a), Float(b)) => self.arithmetic_float(a, b),
            (Int(a), Float(b))  => self.arithmetic_float(a as f64, b),
            (Float(a), Int(b))  => self.arithmetic_float(a, b as f64),
            _ => Err(Fail::NotNumber),
        }
    }
    fn arithmetic_int(&self, a: i64, b: i64) -> Result<Expr, Fail> {
        match self {
            BuiltIn::Add => Ok(Int(a + b)),
            BuiltIn::Sub => Ok(Int(a - b)),
            BuiltIn::Mul => Ok(Int(a * b)),
            BuiltIn::Mod => Ok(Int(a % b)),
            BuiltIn::Div => if b != 0 { Ok(Int(a / b)) } else { Err(Fail::DivisionByZero) }
            _ => panic!("unexpected operator {}", self),
        }
    }
    fn arithmetic_float(&self, a: f64, b: f64) -> Result<Expr, Fail> {
        match self {
            BuiltIn::Add => Ok(Float(a + b)),
            BuiltIn::Sub => Ok(Float(a - b)),
            BuiltIn::Mul => Ok(Float(a * b)),
            BuiltIn::Mod => Ok(Float(a % b)),
            BuiltIn::Div => if b != 0.0 { Ok(Float(a / b)) } else { Err(Fail::DivisionByZero) }
            _ => panic!("unexpected operator {}", self),
        }
    }
    fn comparison_op(&self, left: Expr, right: Expr) ->  Result<Expr, Fail> {
        match self {
            BuiltIn::Eq => Ok(Bool(left.eq(&right))),
            BuiltIn::Neq => Ok(Bool(!left.eq(&right))),
            _ => panic!("unexpected operator {}", self),
        }
    }
    fn binary_op(&self, left: Expr, right: Expr, ctx: &mut Scope) -> Result<Expr, Fail> {
        match (self, left) {
            (BuiltIn::And, FALSE) => Ok(FALSE),
            (BuiltIn::Or, TRUE) => Ok(TRUE),
            (BuiltIn::And, TRUE) => Ok(right.clone().eval(ctx)?.to_bool()),
            (BuiltIn::Or, FALSE) => Ok(right.clone().eval(ctx)?.to_bool()),
            _ => panic!("unexpected operator {}", self),
        }
    }
}

// TODO: move to scope
fn store(ctx: &mut Scope, args: Vec<Expr>, is_mutable: Option<bool>) -> Result<Expr, Fail> {
    if let Symbol(name) = &args[0] {
        let value = &args[args.len() -1];
        if let TypeSpec(expected) = &args[1] {
            if value.get_type() != *expected {
                return Err(Fail::InconsistentType(value.get_type().to_string()))
            }
        }
        let is_defined = ctx.is_defined(&name);
        if is_mutable.is_some() && is_defined {
            Err(Fail::AlreadyDefined(name.to_owned()))
        } else if is_mutable.is_none() && !is_defined {
            Err(Fail::NotDefined(name.to_owned()))
        } else if is_mutable.is_none() && ctx.get_type(&name) != value.get_type() {
            Err(Fail::InconsistentType(value.get_type().to_string()))
        } else if is_mutable.is_none() && ctx.is_mutable(&name) {
            Err(Fail::NotMutable(value.get_type().to_string()))
        } else {
            ctx.set(&name, value, is_mutable);
            Ok(value.clone())
        }
    } else {
        panic!("{} is not an id", args[0])
    }
}
