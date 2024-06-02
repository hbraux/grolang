package fr.braux.ezlang


class Context() {
  private val symbols = mutableMapOf<String, Symbol>()
  private val variables = mutableMapOf<Symbol, AnyObject>()

  fun declare(name: String, type: String, isMutable: Boolean): NullObject {
    if (name in symbols)
      throw LangException(LangExceptionType.ALREADY_DEFINED, name)
    symbols[name] = Symbol(type, isMutable)
    return NullObject
  }

  fun assign(name: String, value: AnyObject): AnyObject {
    val symbol = symbols[name] ?: throw LangException(LangExceptionType.NOT_DEFINED, name)
    if (symbol in variables && !symbol.isMutable)
      throw LangException(LangExceptionType.NOT_MUTABLE, symbol)
    variables[symbol] = value
    return value
  }

  fun assign(name: String, value: AnyObject, isMutable: Boolean) : AnyObject {
    declare(name, value.getClass().asString(), isMutable)
    return assign(name, value)
  }

  fun isDefined(name: String): Boolean = name in symbols

  fun getType(name: String) : String =
    symbols[name]?.type ?: throw LangException(LangExceptionType.NOT_SET, name)

  fun get(name: String) : AnyObject {
    val symbol = symbols[name] ?: throw LangException(LangExceptionType.NOT_DEFINED, name)
    return variables[symbol] ?: throw LangException(LangExceptionType.NOT_SET, name)
  }

  data class Symbol(val type: String, val isMutable: Boolean)
}

