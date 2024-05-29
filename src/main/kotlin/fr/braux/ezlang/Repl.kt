package fr.braux.ezlang

import fr.braux.ezlang.Lang.NAME
import fr.braux.ezlang.Lang.VERSION

object Repl {
  fun loop(debug: Boolean) {
    println("Welcome to $NAME $VERSION REPL")
    while (true) {
      print(PROMPT)
      val input: String
      try {
        input = readln()
      } catch (e: Exception) {
        break
      }
      if (input.startsWith(EXIT))
        break
      val expression = Parser.parse(input)
      if (debug) {
        expression.print()
      }
      val result = expression.eval()
      prettyPrint(result)
    }
  }

  private fun prettyPrint(result: Any) {
    println(result)
  }

  private const val PROMPT = "> "
  private const val EXIT = ":exit"
}