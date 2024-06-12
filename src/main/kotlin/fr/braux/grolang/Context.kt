package fr.braux.grolang


import fr.braux.grolang.Lang.isDefined
import fr.braux.grolang.Lang.toSymbol


class Context {
  fun get(name: String): Expr = name.toSymbol()?.get() ?: ErrorExpr("$name is not set")

  fun declare(name: String, type: String, isMutable: Boolean): Expr {
    if (name.isDefined()) return ErrorExpr("$name is already defined")
    if (type.toSymbol()?.isClass() != true)  return ErrorExpr("Unknown class :$type")
    val symbol = Lang.Symbol(name, type, isMutable)
    return SymbolExpr(symbol.name)
  }

  fun assign(name: String, expr: Expr): Expr {
    val symbol = name.toSymbol() ?: return ErrorExpr("$name is not defined")
    if (!symbol.isMutable) return ErrorExpr("$symbol is not mutable")
    if (expr.getType() != symbol.type) return ErrorExpr("not expected type + " + symbol.type)
    symbol.set(expr) ?: return ErrorExpr("$symbol is not mutable")
    return expr
  }

}

