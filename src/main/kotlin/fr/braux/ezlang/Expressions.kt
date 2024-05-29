package fr.braux.ezlang

interface Expression {
  fun eval(): Any
  fun print(): String
}

object NullExpression: Expression {
  override fun eval(): Any = Lang.Null
  override fun print() = "Null"
}

class IntExpression(private val value: Long): Expression {
  override fun eval() = value
  override fun print() = value.toString()
}

class DecExpression(private val value: Double): Expression {
  override fun eval() = value
  override fun print() = value.toString()
}

class StringExpression(private val value: String): Expression {
  override fun eval() = value
  override fun print() = "\"$value\""
}

class BoolExpression(private val value: Boolean): Expression {
  override fun eval() = value
  override fun print() = value.toString().replaceFirstChar { it.titlecase() }
}

class SymbolExpression(private val value: String): Expression {
  override fun eval() = value
  override fun print() = "'$value"
}
