package fr.braux.ezlang


class Context() {
  private val variables = mutableMapOf<String, AnyObject>()

  fun assign(symbol: String, value: AnyObject) {
    if (variables.contains(symbol))
      throw LangException(LangExceptionType.ALREADY_DEFINED, "$symbol already defined")
    variables[symbol] = value
  }
  fun isDefined(symbol: String): Boolean = symbol in variables

  fun get(symbol: String) : AnyObject =
    variables.get(symbol) ?: throw LangException(LangExceptionType.NOT_DEFINED, "$symbol already defined")

}

