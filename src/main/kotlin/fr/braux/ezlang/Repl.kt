package fr.braux.ezlang

object Repl {

  fun loop(debug: Boolean = false) {
    val context = Context()
    println("Welcome to $LANG_NAME $LANG_VERSION REPL")
    println("type :h for help, :q to quit")
    while (true) {
      print(PROMPT)
      var input: String
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
      // automatically adds 'var' if needed
      ASSIGNMENT.find(input)?.let { if (!context.isDefined(it.groupValues[1]))
          input = "var $input"
      }
      try {
        expression = Parser.parse(input)
      } catch (e: LangException) {
        println("READ ERROR: ${e.message}")
        continue
      }
      if (debug) {
        println("READ OUTPUT: ${expression.asString()}")
      }
      val result: AnyObject
      try {
         result = expression.eval(context)
      } catch (e: LangException) {
        println("EVAL ERROR: ${e.message}")
        continue
      }
      val output: String
      try {
        output = result.asString()
      } catch (e: LangException) {
        println("PRINT ERROR: ${e.message}")
        continue
      }
      prettyPrint(output)
    }
    println("$LANG_NAME terminated")
  }

  private fun printHelp() {
    println("Some help to be added..")
  }

  private fun prettyPrint(output: String) {
    println("\u001B[1m$output\u001B")
  }


  private const val PROMPT = "> "
  private val ASSIGNMENT = """\W*(\w+)\W*=""".toRegex()
}
