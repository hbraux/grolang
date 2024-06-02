package fr.braux.ezlang

import fr.braux.ezlang.Lang.NAME
import fr.braux.ezlang.Lang.VERSION

object Repl {

  fun loop(debug: Boolean = false) {
    val context = Context()
    println("Welcome to $NAME $VERSION REPL")
    println("type :h for help, :q to quit")
    while (true) {
      print(PROMPT)
      val input: String
      val expression: Expression
      try {
        input = readln()
      } catch (e: Exception) {
        break
      }
      if (input.startsWith(":")) {
        when (input) {
          ":q" -> break
          ":h" -> printHelp()
        }
        continue
      }
      try {
        expression = Parser.parse(input)
      } catch (e: Exception) {
        println("SYNTAX ERROR: ${e.message}")
        continue
      }
      if (debug) {
        println("DEBUG: $expression")
      }
      val result = expression.eval(context)
      println(result.asString())
    }
    println("$NAME terminated")
  }

  private fun printHelp() {
    println("TODO")
  }


  private const val PROMPT = "> "
}
