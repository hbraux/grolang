package fr.braux.ezlang

import fr.braux.ezlang.Lang.NULL

interface Expression: AnyObject {
  override fun getClass(): AnyObject = clazz
  companion object {
    private val clazz = ClassObject("Expression")
  }
}


class LiteralExpression<T>(private val value: T?): Expression {
  override fun eval(context: Context): AnyObject = when {
    value == null  -> NullObject
    value is Long -> IntObject(value)
    value is Double -> DecObject(value)
    value is String -> StrObject(value)
    value is Boolean -> BoolObject(value)
    else -> throw IllegalArgumentException()
  }
  override fun asString(): String = value?.toString() ?: NULL

}


