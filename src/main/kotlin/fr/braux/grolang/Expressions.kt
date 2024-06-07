package fr.braux.grolang

import fr.braux.grolang.Expression.Companion.formatToString

sealed interface Expression: AnyObject {
  override fun getClass(): AnyObject = clazz
  val evalType: String?

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
  override val evalType: String = block.lastOrNull()?.evalType ?: TYPE_NIL
  override fun eval(ctx: Context): AnyObject {
    var result: AnyObject = NilObject
    block.forEach { result = it.eval(ctx) }
    return result
  }
  override fun asString(): String = block.joinToString("; ") { it.asString() }
}


data class LiteralExpression<T>(private val value: T?, override val evalType: String): Expression {
  override fun eval(ctx: Context): AnyObject = when {
    value == null  -> NilObject
    value is Long -> IntObject(value)
    value is Double -> FloatObject(value)
    value is String -> StrObject(value)
    value is Boolean -> BoolObject(value)
    else -> throw IllegalArgumentException()
  }
  override fun asString(): String = formatToString(value)

}

data class IdentifierExpression(private val id: String): Expression {
  override val evalType: String? = null
  override fun eval(ctx: Context): AnyObject = ctx.getObject(id)
  override fun asString(): String = "'$id"
}

data class DeclarationExpression(private val id: String, val declaredType: String, private val isMutable: Boolean): Expression {
  override val evalType = declaredType
  override fun eval(ctx: Context): SymbolObject = ctx.defSymbol(id, declaredType, isMutable)
  override fun asString(): String = "def${if (isMutable) "var" else "val"}('$id,'$declaredType)"
}

data class AssignmentExpression(private val id: String, private val right: Expression): Expression {
  override val evalType = right.evalType
  override fun eval(ctx: Context): AnyObject = right.eval(ctx).also { ctx.assign(id, it) }
  override fun asString(): String = "assign('$id, ${right.asString()})"
}


data class CallExpression(private val method: String, private val target: String?, val expressions: List<Expression>): Expression {
  override val evalType = null
  override fun eval(ctx: Context): AnyObject = ctx.getObject(target ?: method).callMethod(method, expressions.map { eval(ctx) })
  override fun asString(): String = expressions.joinToString(",","${target?:""}.$method(", ")") { it.asString() }
}

