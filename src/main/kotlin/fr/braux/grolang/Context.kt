package fr.braux.grolang


class Context() {
  private val symbols = mutableMapOf<String, Symbol>()
  private val classes = mutableMapOf<Symbol, ClassObject>()
  private val variables = mutableMapOf<Symbol, AnyObject>()

  fun declare(name: String, type: String, isMutable: Boolean): SymbolObject {
    if (name in symbols) throw LangException(LangExceptionType.ALREADY_DEFINED, name)
    val className = symbols[type] ?: throw LangException(LangExceptionType.UNKNOWN_TYPE, type)
    val clazz = classes[className] ?: throw LangException(LangExceptionType.UNKNOWN_CLASS, type)
    val symbol = Symbol(name, clazz, isMutable)
    symbols[name] = symbol
    return SymbolObject(symbol.name)
  }

  fun assign(name: String, value: AnyObject, type: String): AnyObject {
    val symbol = symbols[name] ?: throw LangException(LangExceptionType.NOT_DEFINED, name)
    if (symbol in variables) {
      if (!symbol.isMutable) throw LangException(LangExceptionType.NOT_MUTABLE, symbol)
      if (type != symbol.getType()) throw LangException(LangExceptionType.NOT_TYPE, symbol, symbol.getType())
    }
    variables[symbol] = value
    return value
  }

  fun assign(name: String, value: AnyObject) : AnyObject {
    declare(name, value.getType(), false)
    return assign(name, value, value.getType())
  }

  fun isDefined(name: String): Boolean = name in symbols


  fun get(name: String) : AnyObject {
    val symbol = symbols[name] ?: throw LangException(LangExceptionType.NOT_DEFINED, name)
    return variables[symbol] ?: throw LangException(LangExceptionType.NOT_SET, name)
  }
}

