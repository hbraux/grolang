package fr.braux.ezlang

interface Expression {
  fun eval(context: Context): Any
}

object NullLiteral: Expression {
  override fun eval(context: Context): Any = Lang.Null
}

data class IntegerLiteral(private val value: Long): Expression {
  override fun eval(context: Context) = value
}

data class DecimalLiteral(private val value: Double): Expression {
  override fun eval(context: Context) = value
}

data class StringLiteral(private val value: String): Expression {
  override fun eval(context: Context) = value
}

data class BooleanLiteral(private val value: Boolean): Expression {
  override fun eval(context: Context) = value
}

data class SymbolLiteral(private val value: String): Expression {
  override fun eval(context: Context) = value
}


