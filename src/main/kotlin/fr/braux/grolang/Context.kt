package fr.braux.grolang


class Context {
  private val symbols = mutableMapOf<String, Symbol>()
  private val objects = mutableMapOf<Symbol, Expr>()

  init {
    builtInClasses.forEach { register(it) }
    builtInFunctions.forEach { register(it) }
  }

  fun defSymbol(name: String, type: String, isMutable: Boolean): SymbolObject {
    if (name in symbols)
      throw LangException(ExceptionType.ALREADY_DEFINED, name)
    val clazz = symbols[type]?.let { objects[it] } ?: throw LangException(ExceptionType.UNKNOWN_TYPE, type)
    if (clazz !is ClassExpr)
      throw LangException(ExceptionType.UNKNOWN_CLASS, type)
    symbols[name] = Symbol(name, clazz.name, isMutable)
    return SymbolObject(name)
  }

  private fun register(any: Expr) {
    val name = any.asString()
    val symbol = Symbol(name, any.getType())
    symbols[name] = symbol
    objects[symbol] = any
  }


  fun assign(name: String, obj: Expr) {
    val symbol = getSymbol(name)
    if (symbol in objects) {
      if (!symbol.isMutable) throw LangException(ExceptionType.NOT_MUTABLE, symbol)
      if (obj.getType() != symbol.type)
        throw LangException(ExceptionType.NOT_TYPE, symbol.type, symbol.type)
    }
    objects[symbol] = obj
  }
  fun declare(name: String, obj: Expr) {
    defSymbol(name, obj.getType(), false)
    assign(name, obj)
  }

  fun isDefined(name: String): Boolean = name in symbols

  fun getSymbol(name: String): Symbol = symbols[name] ?: throw LangException(ExceptionType.NOT_DEFINED, name)

  fun getObject(name: String): Expr = objects[getSymbol(name)] ?: throw LangException(ExceptionType.NOT_SET, name)

  fun getFunction(name: String): Function = getObject(name).let{
    if (it is Function) it else throw LangException(ExceptionType.NOT_FUNCTION, name)
  }

}

