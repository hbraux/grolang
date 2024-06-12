package fr.braux.grolang

import fr.braux.grolang.ClassExpr.Companion.classAny
import fr.braux.grolang.ClassExpr.Companion.classBool
import fr.braux.grolang.ClassExpr.Companion.classClass
import fr.braux.grolang.ClassExpr.Companion.classFloat
import fr.braux.grolang.ClassExpr.Companion.classFunction
import fr.braux.grolang.ClassExpr.Companion.classInt
import fr.braux.grolang.ClassExpr.Companion.classStr
import fr.braux.grolang.ClassExpr.Companion.classSymbol
import java.io.IOException


sealed interface Expr {
  fun eval(ctx: Context): Expr
  fun asString(): String
  fun getClass(): ClassExpr
  fun getType(): String = getClass().name
  fun print(): String = asString()
  fun getMethod(name: String) : Function? = null
}

// Class
data class ClassExpr(val name: String): Expr {
  override fun eval(ctx: Context) = this
  override fun asString() = name
  override fun print() = "Class(name=$name)"
  override fun getClass() = classClass

  companion object {
    val classAny = ClassExpr(TYPE_ANY)
    val classClass = ClassExpr(TYPE_CLASS)
    val classFunction = ClassExpr(TYPE_FUNCTION)
    val classSymbol = ClassExpr(TYPE_SYMBOL)
    val classInt = ClassExpr(TYPE_INT)
    val classFloat = ClassExpr(TYPE_FLOAT)
    val classBool = ClassExpr(TYPE_BOOL)
    val classStr = ClassExpr(TYPE_STR)
    val builtInClasses = listOf(classAny, classClass, classFunction, classSymbol, classInt, classFloat, classBool, classStr)
  }
}

// Literals
abstract class LiteralExpr<T>(val value: T?, private val clazz: ClassExpr): Expr {
  override fun eval(ctx: Context) = this
  override fun asString() = value?.toString() ?: STRING_NULL
  override fun getClass() = clazz
}


data object NullExpr: LiteralExpr<Any>(null, classAny)
class IntExpr(value: Long): LiteralExpr<Long>(value, classInt)
class FloatExpr(value: Double): LiteralExpr<Double>(value, classFloat)
class BoolExpr(value: Boolean): LiteralExpr<Boolean>(value, classBool)
class StrExpr(value: String): LiteralExpr<String>(value, classStr)
class SymbolExpr(value: String): LiteralExpr<String>(value, classSymbol)

// Identifier
data class IdentifierExpr(private val id: String): Expr {
  override fun eval(ctx: Context): Expr = ctx.getObject(id)
  override fun asString(): String = "'$id"
  override fun getClass() = classAny
}

data class DeclarationExpr(private val id: String, val declaredType: String, private val isMutable: Boolean): Expr {
  override fun eval(ctx: Context): SymbolExpr = ctx.defSymbol(id, declaredType, isMutable)
  override fun asString(): String = "def${if (isMutable) "var" else "val"}('$id,'$declaredType)"
  override fun getClass() = classClass
}


data class AssignmentExpr(private val id: String, private val right: Expr): Expr {
  override fun eval(ctx: Context): Expr = right.eval(ctx).also { ctx.assign(id, it) }
  override fun asString(): String = "assign('$id, ${right.asString()})"
  override fun getClass() = classAny
}

data class BlockExpr(private val block: List<Expr>) : Expr {
  constructor(vararg args: Expr) : this(args.toList())
  override fun eval(ctx: Context): Expr {
    var result: Expr = NullExpr
    block.forEach { result = it.eval(ctx) }
    return result
  }
  override fun asString(): String = block.joinToString("; ", "{", "}") { it.asString() }
  override fun getClass() = classAny
}


data class CallExpr(private val name: String, val expressions: List<Expr>): Expr {
  override fun eval(ctx: Context): Expr = ctx.getFunction(name).call(expressions.map { it.eval(ctx) })
  override fun asString(): String = expressions.joinToString(",", "$name(", ")") { it.asString() }
  override fun getClass() = classAny
}




data class Function(val name: String, val inputTypes: List<String>, val outputType: String?,
                    val impl: (List<Expr>) -> (Expr)): Expr {
  override fun eval(ctx: Context) = this
  override fun asString() = name
  override fun print() = "Function(name=$name)"
  override fun getClass() = classFunction
  fun call(args: List<Expr>): Expr {
    if (args.size != inputTypes.size) throw LangException("WRONG_ARGUMENTS")
    (args zip inputTypes).forEach {
      if (it.first.getType() != it.second) throw LangException("WRONG_ARGUMENTS")
    }
    return impl.invoke(args)
  }
}



class LangException(message: String): IOException(message)

val printFunction = Function("print", listOf("TYPE_ANY"), null) { println(it[0].asString()); NullExpr }

val builtInFunctions = listOf(
  Function("print", listOf("TYPE_ANY"), null) { println(it[0].asString()); NullExpr },
  Function("read", listOf("TYPE_ANY"), TYPE_STR) { StrExpr(readln()) }
)
