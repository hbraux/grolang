package fr.braux.grolang


import fr.braux.grolang.Lang.builtin

sealed interface Expr {
  fun getType(): String = TYPE_EXPR
  fun eval(ctx: Context): Expr = this
  fun asString(): String
  fun print(): String = asString()
  fun getMethod(name: String) : Function? = null
}

data class ClassExpr(val name: String): Expr {
  override fun getType() = TYPE_CLASS
  override fun eval(ctx: Context) = this
  override fun asString() = name
  override fun print() = "Class($name)"
  init { builtin(name, this) }
}

// Literals
abstract class LiteralExpr<T>(val value: T?, private val type: String): Expr {
  override fun getType() = type
  override fun asString() = value?.toString() ?: STRING_NULL
}

data object NullExpr: LiteralExpr<Any>(null, TYPE_ANY)
class IntExpr(value: Long): LiteralExpr<Long>(value, TYPE_INT)
class FloatExpr(value: Double): LiteralExpr<Double>(value, TYPE_FLOAT)
class BoolExpr(value: Boolean): LiteralExpr<Boolean>(value, TYPE_BOOL)
class StrExpr(value: String): LiteralExpr<String>(value, TYPE_STR)
class SymbolExpr(value: String): LiteralExpr<String>(value, TYPE_SYMBOL)
class ErrorExpr(message: String, vararg args: Any): LiteralExpr<String>(String.format(message, *args), TYPE_ERROR)

// Identifier
data class IdentifierExpr(private val name: String): Expr {
  override fun eval(ctx: Context): Expr = ctx.get(name)
  override fun asString(): String = "'$name"
}

data class DeclarationExpr(private val id: String, val declaredType: String, private val isMutable: Boolean): Expr {
  override fun eval(ctx: Context) = ctx.declare(id, declaredType, isMutable)
  override fun asString(): String = "def${if (isMutable) "var" else "val"}('$id,'$declaredType)"
}


data class AssignmentExpr(private val id: String, private val right: Expr): Expr {
  override fun eval(ctx: Context): Expr = right.eval(ctx).also { ctx.assign(id, it) }
  override fun asString(): String = "assign('$id, ${right.asString()})"
}

data class BlockExpr(private val block: List<Expr>) : Expr {
  constructor(vararg args: Expr) : this(args.toList())
  override fun eval(ctx: Context): Expr {
    var result: Expr = NullExpr
    block.forEach { result = it.eval(ctx) }
    return result
  }
  override fun asString(): String = block.joinToString("; ", "{", "}") { it.asString() }
}


data class CallExpr(private val name: String, val expressions: List<Expr>): Expr {
  override fun eval(ctx: Context): Expr = ctx.getFunction(name).call(expressions.map { it.eval(ctx) })
  override fun asString(): String = expressions.joinToString(",", "$name(", ")") { it.asString() }
}




data class Function(val name: String, val inputTypes: List<String>, val outputType: String?,
                    val impl: (List<Expr>) -> (Expr)): Expr {
  override fun eval(ctx: Context) = this
  override fun asString() = name
  override fun print() = "Function(name=$name)"
  fun call(args: List<Expr>): Expr {
    if (args.size != inputTypes.size) throw LangException("WRONG_ARGUMENTS")
    (args zip inputTypes).forEach {
      if (it.first.getType() != it.second) throw LangException("WRONG_ARGUMENTS")
    }
    return impl.invoke(args)
  }
}



val printFunction = Function("print", listOf("TYPE_ANY"), null) { println(it[0].asString()); NullExpr }

val builtInFunctions = listOf(
  Function("print", listOf("TYPE_ANY"), null) { println(it[0].asString()); NullExpr },
  Function("read", listOf("TYPE_ANY"), TYPE_STR) { StrExpr(readln()) }
)
