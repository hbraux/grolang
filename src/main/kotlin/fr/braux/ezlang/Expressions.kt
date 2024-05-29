package fr.braux.ezlang

interface Expression {
  fun eval(): Any
}

object NullValue
object NullExpression: Expression {
  override fun eval(): Any = NullValue
}

class IntExpression(private val value: Long): Expression {
  override fun eval() = value
}

class DecExpression(private val value: Double): Expression {
  override fun eval() = value
}

class StringExpression(private val value: String): Expression {
  override fun eval() = value
}

class BoolExpression(private val value: Boolean): Expression {
  override fun eval() = value
}
