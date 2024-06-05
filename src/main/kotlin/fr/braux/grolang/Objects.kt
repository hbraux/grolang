package fr.braux.grolang


sealed interface AnyObject {
  fun eval(ctx: Context): AnyObject
  fun asString(): String
  fun getClass(): AnyObject
  fun getType(): String = getClass().asString()
}

data class ClassObject(private val name: String): AnyObject {
  override fun eval(ctx: Context) = this
  override fun asString(): String = name
  override fun getClass(): AnyObject = classClass
}

abstract class LiteralObject<T>(val value: T?, private val clazz: ClassObject): AnyObject {
  override fun eval(ctx: Context): AnyObject = this
  override fun asString(): String = value?.toString() ?: STRING_NULL
  override fun getClass(): AnyObject = clazz
}

data object NullObject: LiteralObject<Any>(null, classNull)
class IntObject(value: Long): LiteralObject<Long>(value, classInt)
class FloatObject(value: Double): LiteralObject<Double>(value, classFloat)
class BoolObject(value: Boolean): LiteralObject<Boolean>(value, classBool)
class StrObject(value: String): LiteralObject<String>(value, classStr)
class SymbolObject(value: String): LiteralObject<String>(value, classSymbol)

val classClass = ClassObject(TYPE_CLASS)
val classInt = ClassObject(TYPE_INT)
val classFloat = ClassObject(TYPE_FLOAT)
val classBool = ClassObject(TYPE_BOOL)
val classNull = ClassObject(TYPE_NULL)
val classStr = ClassObject(TYPE_STR)
val classSymbol = ClassObject(TYPE_SYMBOL)
val builtInClasses = listOf(classClass, classInt, classFloat, classBool, classNull, classStr, classSymbol)
