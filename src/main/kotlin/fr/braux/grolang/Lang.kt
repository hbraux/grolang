package fr.braux.grolang

import java.io.IOException


const val LANG_NAME = "groLang"
const val LANG_VERSION = "0.1"
const val STRING_NULL = "null"

// primary types (which are also classes)
const val ANY = "Any"
const val CLASS = "Class"
const val INT = "Int"
const val FLOAT = "Float"
const val STR = "Str"
const val BOOL = "Bool"
const val SYMBOL = "Symbol"
const val ERROR = "Error"
const val FUNCTION = "Function"
const val EXPR = "Expr"

object Lang {
  private val types = listOf(ANY, CLASS, INT, FLOAT, STR, BOOL, SYMBOL, ERROR, FUNCTION, EXPR).toMutableList()
  private val data = mutableMapOf<Symbol, Expr>()
  private val globals = mutableMapOf<String, Symbol>()

  fun String.isDefined(): Boolean = this in globals
  fun String.toSymbol(): Symbol? = globals[this]
  fun builtin(name: String, expr: Expr) = Symbol(name, expr.getType(), false).set(expr)


  init {
    // load classes
    types.forEach { ClassExpr(it) }
    // builtInFunctions
    Function("print", listOf(ANY), null) { _, args -> println(args[0].asString()); NullExpr }
    Function("read", listOf(ANY), STR) { _, _ -> StrExpr(readln()) }
    Function("declare", listOf(SYMBOL, ANY, BOOL), STR) { ctx, args -> ctx.declare(args[0].asString(), args[1].asString(), args[2].asString().toBoolean()) }
    Function("assign", listOf(SYMBOL, ANY), SYMBOL) { ctx, args -> ctx.assign(args[0].asString(), args[1]) }
  }

  data class Symbol(val name: String, val type: String, val isMutable: Boolean = false) {
    fun get(): Expr = data[this] ?: ErrorExpr("symbol '%s is not set", name)
    fun set(value: Expr): Expr? = if (isMutable) value.also { data[this] = it } else null
    init {
      globals[name] = this
    }
  }


  fun assign(name: String, expr: Expr): Expr {
    val symbol = name.toSymbol() ?: return ErrorExpr("$name is not defined")
    if (expr.getType() != symbol.type) return ErrorExpr("not expected type + " + symbol.type)
    symbol.set(expr) ?: return ErrorExpr("$symbol is not mutable")
    return expr
  }

}

class LangException(message: String): IOException(message)

