package fr.braux.grolang


sealed interface Expr {
  fun eval(ctx: Context): Expr = this
  fun asString(): String
  fun getClass(): ClassExpr = classAny
  fun getType(): String = getClass().name
  fun print(): String = asString()
  fun getMethod(name: String) : Function? = null
}


data class ClassExpr(val name: String): Expr {
  override fun asString() = name
  override fun print() = "Class(name=$name)"
  override fun getClass() = classClass
}

data class DeclarationExpr(private val id: String, val declaredType: String, private val isMutable: Boolean): Expr {
  override fun eval(ctx: Context): SymbolObject = ctx.defSymbol(id, declaredType, isMutable)
  override fun asString(): String = "def${if (isMutable) "var" else "val"}('$id,'$declaredType)"
}


data class AssignmentExpr(private val id: String, private val right: Expr): Expr {
  override fun eval(ctx: Context): Expr = right.eval(ctx).also { ctx.assign(id, it) }
  override fun asString(): String = "assign('$id, ${right.asString()})"
}

data class BlockExpr(private val block: List<Expr>) : Expr {
  constructor(vararg args: Expr) : this(args.toList())
  override fun eval(ctx: Context): Expr {
    var result: Expr = NullObject
    block.forEach { result = it.eval(ctx) }
    return result
  }
  override fun asString(): String = block.joinToString("; ", "{", "}") { it.asString() }
}


data class CallExpr(private val name: String, val expressions: List<Expr>): Expr {
  override fun eval(ctx: Context): Expr = ctx.getFunction(name).call(expressions.map { it.eval(ctx) })
  override fun asString(): String = expressions.joinToString(",", "$name(", ")") { it.asString() }
}


data class Identifier(private val id: String): Expr {
  override fun eval(ctx: Context): Expr = ctx.getObject(id)
  override fun asString(): String = "'$id"
}

data class Function(val name: String, val inputTypes: List<String>, val outputType: String?,
                    val impl: (List<Expr>) -> (Expr)): Expr {
  override fun eval(ctx: Context) = this
  override fun asString() = name
  override fun print() = "Function(name=$name)"
  override fun getClass() = classFunction
  fun call(args: List<Expr>): Expr {
    if (args.size != inputTypes.size) throw LangException(ExceptionType.WRONG_ARGUMENTS, args.size, inputTypes.size)
    (args zip inputTypes).forEach {
      if (it.first.getType() != it.second) throw LangException(ExceptionType.WRONG_ARGUMENTS)
    }
    return impl.invoke(args)
  }
}


abstract class Literal<T>(val value: T?, private val clazz: ClassExpr): Expr {
  override fun eval(ctx: Context) = this
  override fun asString() = value?.toString() ?: STRING_NULL
  override fun getClass() = clazz
}

data object NullObject: Literal<Any>(null, classAny)
class IntObject(value: Long): Literal<Long>(value, classInt)
class FloatObject(value: Double): Literal<Double>(value, classFloat)
class BoolObject(value: Boolean): Literal<Boolean>(value, classBool)
class StrObject(value: String): Literal<String>(value, classStr)
class SymbolObject(value: String): Literal<String>(value, classSymbol)

val classAny = ClassExpr(TYPE_ANY)
val classClass = ClassExpr(TYPE_CLASS)
val classFunction = ClassExpr(TYPE_FUNCTION)
val classSymbol = ClassExpr(TYPE_SYMBOL)
val classInt = ClassExpr(TYPE_INT)
val classFloat = ClassExpr(TYPE_FLOAT)
val classBool = ClassExpr(TYPE_BOOL)
val classStr = ClassExpr(TYPE_STR)

val builtInClasses = listOf(classAny, classClass, classFunction, classSymbol, classInt, classFloat, classBool, classStr)

val printFunction = Function("print", listOf("TYPE_ANY"), null) { println(it[0].asString()); NullObject }

val builtInFunctions = listOf(printFunction)
