package fr.braux.grolang


data class Symbol(val name: String, val clazz: ClassObject, val isMutable: Boolean = false) {
  fun getType(): String = clazz.asString()
}
