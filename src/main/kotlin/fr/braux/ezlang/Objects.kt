package fr.braux.ezlang

import fr.braux.ezlang.AnyObject.Companion.classBool
import fr.braux.ezlang.AnyObject.Companion.classClass
import fr.braux.ezlang.AnyObject.Companion.classFloat
import fr.braux.ezlang.AnyObject.Companion.classInt
import fr.braux.ezlang.AnyObject.Companion.classNull
import fr.braux.ezlang.AnyObject.Companion.classStr


sealed interface AnyObject {
  fun eval(context: Context): AnyObject
  fun asString(): String
  fun getClass(): AnyObject

  companion object {
    val classClass = ClassObject(TYPE_CLASS)
    val classInt = ClassObject(TYPE_INT)
    val classFloat = ClassObject(TYPE_FLOAT)
    val classBool = ClassObject(TYPE_BOOL)
    val classNull = ClassObject(TYPE_NULL)
    val classStr = ClassObject(TYPE_STR)
  }
}

open class ClassObject(private val name: String): AnyObject {
  override fun eval(context: Context) = this
  override fun asString(): String = name
  override fun getClass(): AnyObject = classClass
}

abstract class LiteralObject<T>(val value: T?, private val clazz: ClassObject): AnyObject {
  override fun eval(context: Context): AnyObject = this
  override fun asString(): String = value?.toString() ?: STRING_NULL
  override fun getClass(): AnyObject = clazz
}

object NullObject: LiteralObject<Any>(null, classNull)
class IntObject(value: Long): LiteralObject<Long>(value, classInt)
class FloatObject(value: Double): LiteralObject<Double>(value, classFloat)
class BoolObject(value: Boolean): LiteralObject<Boolean>(value, classBool)
class StrObject(value: String): LiteralObject<String>(value, classStr)


