package fr.braux.grolang


import fr.braux.grolang.Lang.Symbol
import fr.braux.grolang.Lang.isDefined
import fr.braux.grolang.Lang.toSymbol


class Context {
  fun isDefined(name: String): Boolean = name.toSymbol() != null

  fun get(name: String): Expr = name.toSymbol()?.get() ?: ErrorExpr("$name is not set")

  fun declare(name: String, type: String, isMutable: Boolean): Expr {
    if (name.isDefined()) return ErrorExpr("$name is already defined")
    if (type.toSymbol()?.type != CLASS)  return ErrorExpr("Unknown class :$type")
    Symbol(name, type, isMutable)
    return SymbolExpr(name)
  }

  fun assign(name: String, expr: Expr): Expr {
    val symbol = name.toSymbol() ?: return ErrorExpr("$name is not defined")
    if (expr.getType() != symbol.type) return ErrorExpr("not expected type + " + symbol.type)
    symbol.set(expr) ?: return ErrorExpr("$symbol is not mutable")
    return expr
  }

  fun getFunction(name: String): Function? = name.toSymbol()?.get()?.let { if (it is Function) it else null }

}

