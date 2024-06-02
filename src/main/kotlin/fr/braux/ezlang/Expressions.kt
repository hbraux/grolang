package fr.braux.ezlang

import fr.braux.ezlang.Expression.Companion.formatToString
import fr.braux.ezlang.Lang.NULL

sealed interface Expression: AnyObject {
  override fun getClass(): AnyObject = clazz

  companion object {
    private val clazz = ClassObject("Expr")

    fun formatToString(value: Any?) : String = when {
      value == null -> NULL
      value is String -> "\"" + value + "\""
      else -> value.toString()
    }
  }
}


data class LiteralExpression<T>(private val value: T?): Expression {
  override fun eval(context: Context): AnyObject = when {
    value == null  -> NullObject
    value is Long -> IntObject(value)
    value is Double -> DecObject(value)
    value is String -> StrObject(value)
    value is Boolean -> BoolObject(value)
    else -> throw IllegalArgumentException()
  }
  override fun asString(): String = formatToString(value)

}

data class IdentifierExpression(private val symbol: String): Expression {
  override fun eval(context: Context): AnyObject = context.get(symbol)
  override fun asString(): String = "'$symbol"
}
