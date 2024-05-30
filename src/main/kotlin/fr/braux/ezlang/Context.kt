package fr.braux.ezlang

class Context() {
  private val variables = mutableMapOf<String, Any>()

  fun assign(symbol: String, value: Any) {
    variables[symbol] = value
  }
  fun import(namespace: Namespace, symbol: String) {}
}

