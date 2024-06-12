package fr.braux.grolang

import fr.braux.grolang.ClassExpr.Companion.builtInClasses


class Context {
  private val symbols = mutableMapOf<String, Symbol>()
  private val objects = mutableMapOf<Symbol, Expr>()

  init {
    builtInClasses.forEach { register(it) }
    builtInFunctions.forEach { register(it) }
  }

  private fun register(expr: Expr) {
    val name = expr.asString()
    val symbol = Symbol(name, expr.getType())
    symbols[name] = symbol
    objects[symbol] = expr
  }

  fun defSymbol(name: String, type: String, isMutable: Boolean): SymbolExpr {
    if (name in symbols)
      throw LangException("$name is already defined")
    val clazz = symbols[type]?.let { objects[it] } ?: throw LangException("Unknown type :$type")
    if (clazz !is ClassExpr)
      throw LangException("Unknown class :$type")
    symbols[name] = Symbol(name, clazz.name, isMutable)
    return SymbolExpr(name)
  }


  fun assign(name: String, obj: Expr) {
    val symbol = getSymbol(name)
    if (symbol in objects) {
      if (!symbol.isMutable) throw LangException("$symbol is not mutable")
      if (obj.getType() != symbol.type)
        throw LangException("not expected type + " + symbol.type)
    }
    objects[symbol] = obj
  }


  fun isDefined(name: String): Boolean = name in symbols

  fun getSymbol(name: String): Symbol = symbols[name] ?: throw LangException("$name is not defined")

  fun getObject(name: String): Expr = objects[getSymbol(name)] ?: throw LangException("$name is not set")

  fun getFunction(name: String): Function = getObject(name).let{
    if (it is Function) it else throw LangException("$name is not a function")
  }

}

