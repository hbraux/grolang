package fr.braux.ezlang


interface Object {
  val ofClass: Object
  val type: ObjectType
}


open class ObjectType(name: String): Object {
  override val type: ObjectType
    get() = ObjectType("TYPE")
}

object Any: ObjectType("Any")

class ObjectLiteral {

}
