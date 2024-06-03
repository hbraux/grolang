package fr.braux.grolang


data class Symbol(val name: String, val clazz: ClassObject, val isMutable: Boolean) {
  fun getType(): String = clazz.asString()
}
