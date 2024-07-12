use std::fmt::{Debug, Formatter};
use crate::exception::Exception;
use crate::expr::Expr;

type FunExpr =  fn(Vec<Expr>) -> Result<Expr, Exception>;

pub struct Lambda {
    fun: FunExpr
}

impl Lambda {
    pub fn new(fun: FunExpr) -> Lambda { Lambda { fun } }
}

impl Clone for Lambda {
    fn clone(&self) -> Self { unreachable!() }
}
impl Debug for Lambda {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { unreachable!() }
}
impl PartialEq for Lambda {
    fn eq(&self, _other: &Self) -> bool { unreachable!() }
}
