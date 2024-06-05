package fr.braux.grolang

import fr.braux.grolang.AnyObject.Companion.builtInClasses
import fr.braux.grolang.AnyObject.Companion.classClass


class Context {
  private val symbols = mutableMapOf<String, Symbol>()
  private val references = mutableMapOf<Symbol, AnyObject>()

  init {
    symbols[TYPE_CLASS] = Symbol(TYPE_CLASS, classClass).also { references[it] = classClass }
    builtInClasses.forEach { assign(it.asString(), it) }
  }
  fun declare(name: String, type: String, isMutable: Boolean): SymbolObject {
    if (name in symbols) throw LangException(LangExceptionType.ALREADY_DEFINED, name)
    val className = symbols[type] ?: throw LangException(LangExceptionType.UNKNOWN_TYPE, type)
    val clazz = references[className] ?: throw LangException(LangExceptionType.NOT_SET, type)
    if (clazz !is ClassObject) throw LangException(LangExceptionType.UNKNOWN_CLASS, type)
    val symbol = Symbol(name, clazz, isMutable)
    symbols[name] = symbol
    return SymbolObject(symbol.name)
  }

  fun assign(name: String, value: AnyObject, type: String): AnyObject {
    val symbol = symbols[name] ?: throw LangException(LangExceptionType.NOT_DEFINED, name)
    if (symbol in references) {
      if (!symbol.isMutable) throw LangException(LangExceptionType.NOT_MUTABLE, symbol)
      if (type != symbol.getType()) throw LangException(LangExceptionType.NOT_TYPE, symbol)
    }
    references[symbol] = value
    return value
  }

  fun assign(name: String, value: AnyObject) : AnyObject {
    declare(name, value.getType(), false)
    return assign(name, value, value.getType())
  }

  fun isDefined(name: String): Boolean = name in symbols


  fun get(name: String) : AnyObject {
    val symbol = symbols[name] ?: throw LangException(LangExceptionType.NOT_DEFINED, name)
    return references[symbol] ?: throw LangException(LangExceptionType.NOT_SET, name)
  }

}

