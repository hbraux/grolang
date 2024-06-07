package fr.braux.grolang


class Context {
  private val symbols = mutableMapOf<String, Symbol>()
  private val objects = mutableMapOf<Symbol, AnyObject>()
  init {
    builtInClasses.forEach { defClass(it) }
  }

  fun defSymbol(name: String, type: String, isMutable: Boolean): SymbolObject {
    if (name in symbols)
      throw LangException(ExceptionType.ALREADY_DEFINED, name)
    val obj = symbols[type]?.let { objects[it] } ?: throw LangException(ExceptionType.UNKNOWN_TYPE, type)
    if (obj !is ClassObject)
      throw LangException(ExceptionType.UNKNOWN_CLASS, type)
    symbols[name] = Symbol(name, obj, isMutable)
    return SymbolObject(name)
  }

  private fun defClass(clazz: ClassObject) {
    val name = clazz.asString()
    val symbol = Symbol(name, clazz)
    symbols[name] = symbol
    objects[symbol] = clazz
  }

  fun assign(name: String, obj: AnyObject) {
    val symbol = getSymbol(name)
    if (symbol in objects) {
      if (!symbol.isMutable) throw LangException(ExceptionType.NOT_MUTABLE, symbol)
      if (obj.getType() != symbol.getType())
        throw LangException(ExceptionType.NOT_TYPE, symbol.getType(), symbol.getType())
    }
    objects[symbol] = obj
  }
  fun declare(name: String, obj: AnyObject) {
    defSymbol(name, obj.getType(), false)
    assign(name, obj)
  }

  fun isDefined(name: String): Boolean = name in symbols

  fun getSymbol(name: String): Symbol = symbols[name] ?: throw LangException(ExceptionType.NOT_DEFINED, name)

  fun getObject(name: String): AnyObject = objects[getSymbol(name)] ?: throw LangException(ExceptionType.NOT_SET, name)

}

