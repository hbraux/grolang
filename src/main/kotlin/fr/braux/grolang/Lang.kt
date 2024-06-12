package fr.braux.grolang

import java.io.IOException


const val LANG_NAME = "groLang"
const val LANG_VERSION = "0.1"
const val STRING_NULL = "null"

// primary types (which are also classes)
const val TYPE_ANY = "Any"
const val TYPE_CLASS = "Class"
const val TYPE_INT = "Int"
const val TYPE_FLOAT = "Float"
const val TYPE_STR = "Str"
const val TYPE_BOOL = "Bool"
const val TYPE_SYMBOL = "Symbol"
const val TYPE_ERROR = "Error"
const val TYPE_FUNCTION = "Function"
const val TYPE_EXPR = "Expr"

object Lang {
  private val types = listOf(TYPE_ANY, TYPE_CLASS, TYPE_INT, TYPE_FLOAT, TYPE_STR, TYPE_BOOL, TYPE_SYMBOL, TYPE_ERROR, TYPE_FUNCTION, TYPE_EXPR).toMutableList()
  private val data = mutableMapOf<Symbol, Expr>()
  private val globals = mutableMapOf<String, Symbol>()

  fun String.isDefined(): Boolean = this in globals
  fun String.toSymbol() = globals[this]
  
  fun builtin(name: String, expr: Expr): Symbol = Symbol(name, expr.type, false).also { data[it] = expr }

  init {
    // load classes
    types.forEach { ClassExpr(it) }
  }


  data class Symbol(val name: String, val type: String, val isMutable: Boolean = false) {
    fun get(): Expr = data[this] ?: ErrorExpr("symbol '%s is no set", this)
    fun set(value: Expr): Expr? = if (isMutable) value.also { data[this] = it } else null 
    fun isClass() = type == TYPE_CLASS
    init {
      globals[name] = this
    }
  }

}

class LangException(message: String): IOException(message)

