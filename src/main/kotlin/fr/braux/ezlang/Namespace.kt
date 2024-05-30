package fr.braux.ezlang

data class Namespace(val name: String) : Builtin {
  private val symbols = mutableMapOf<Symbol, Any>()
}


