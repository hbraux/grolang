package fr.braux.ezlang

import fr.braux.ezlang.AnyObject.Companion.classBool
import fr.braux.ezlang.AnyObject.Companion.classClass
import fr.braux.ezlang.AnyObject.Companion.classDec
import fr.braux.ezlang.AnyObject.Companion.classInt
import fr.braux.ezlang.AnyObject.Companion.classNull
import fr.braux.ezlang.AnyObject.Companion.classStr
import fr.braux.ezlang.Lang.NULL


interface AnyObject {
  fun eval(context: Context): AnyObject
  fun asString(): String
  fun getClass(): AnyObject

  companion object {
    val classClass = ClassObject("Class")
    val classInt = ClassObject("Int")
    val classDec = ClassObject("Dec")
    val classBool = ClassObject("Bool")
    val classNull = ClassObject("NullType")
    val classStr = ClassObject("Str")
  }
}

open class ClassObject(private val name: String): AnyObject {
  override fun eval(context: Context) = this
  override fun asString(): String = name
  override fun getClass(): AnyObject = classClass
}

abstract class LiteralObject<T>(private val value: T?, private val clazz: ClassObject): AnyObject {
  override fun eval(context: Context): AnyObject = this
  override fun asString(): String = value?.toString() ?: NULL
  override fun getClass(): AnyObject = clazz
}

object NullObject: LiteralObject<Any>(null, classNull)
class IntObject(value: Long): LiteralObject<Long>(value, classInt)
class DecObject(value: Double): LiteralObject<Double>(value, classDec)
class BoolObject(value: Boolean): LiteralObject<Boolean>(value, classBool)
class StrObject(value: String): LiteralObject<String>(value, classStr)


