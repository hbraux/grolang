package fr.braux.ezlang

import fr.braux.ezlang.Lang.NAME
import fr.braux.ezlang.Lang.VERSION

object Repl {

  fun loop(debug: Boolean = false) {
    val context = Context()
    println("Welcome to $NAME $VERSION REPL")
    var index = 1
    while (true) {
      print(PROMPT)
      val input: String
      val expression: Expression
      try {
        input = readln()
      } catch (e: Exception) {
        break
      }
      if (input.startsWith(EXIT))
        break
      try {
        expression = Parser.parse(input)
      } catch (e: Exception) {
        println("SYNTAX ERROR: ${e.message}")
        continue
      }
      val variable = "res${index++}"
      val assignment = Assignment(variable, expression)
      if (debug) {
        println("DEBUG: $assignment")
      }
      val result = assignment.eval(context)
      print("$variable: ")
      prettyPrint(result)
    }
  }

  private fun prettyPrint(result: Any) {
    println(result)
  }

  private const val PROMPT = "> "
  private const val EXIT = ":exit"
}
