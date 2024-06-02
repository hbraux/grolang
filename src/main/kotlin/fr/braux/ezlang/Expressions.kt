package fr.braux.ezlang

import fr.braux.ezlang.Expression.Companion.formatToString

sealed interface Expression: AnyObject {
  override fun getClass(): AnyObject = clazz
  val evalType: String

  companion object {
    private val clazz = ClassObject("Expr")

    fun formatToString(value: Any?) : String = when {
      value == null -> STRING_NULL
      value is String -> "\"" + value + "\""
      else -> value.toString()
    }
  }
}

data class BlockExpression(private val block: List<Expression>) : Expression {
  constructor(vararg args: Expression) : this(args.toList())
  override val evalType: String = block.lastOrNull()?.evalType ?: TYPE_NULL
  override fun eval(context: Context): AnyObject {
    var result: AnyObject = NullObject
    block.forEach { result = it.eval(context) }
    return result
  }
  override fun asString(): String = block.joinToString(prefix = "_block(", separator = ",", postfix = ")") { it.asString() }
}

data class LiteralExpression<T>(private val value: T?, override val evalType: String): Expression {
  override fun eval(context: Context): AnyObject = when {
    value == null  -> NullObject
    value is Long -> IntObject(value)
    value is Double -> FloatObject(value)
    value is String -> StrObject(value)
    value is Boolean -> BoolObject(value)
    else -> throw IllegalArgumentException()
  }
  override fun asString(): String = formatToString(value)

}

data class IdentifierExpression(private val symbol: String): Expression {
  override val evalType: String = TYPE_SYMBOL // FIXME: not ok
  override fun eval(context: Context): AnyObject = context.get(symbol)
  override fun asString(): String = "'$symbol"
}

data class DeclarationExpression(private val symbol: String, val type: String, private val isMutable: Boolean): Expression {
  override val evalType: String = TYPE_NULL
  override fun eval(context: Context): NullObject = context.declare(symbol, type, isMutable)
  override fun asString(): String = "_def('$symbol,\"$type\",$isMutable)"
}


data class AssignmentExpression(private val symbol: String, private val right: Expression): Expression {
  override val evalType = right.evalType
  override fun eval(context: Context): AnyObject = right.eval(context).also { context.assign(symbol, it) }
  override fun asString(): String = "_assign('$symbol, ${right.asString()})"
}
