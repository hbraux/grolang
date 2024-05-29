package fr.braux.ezlang

import fr.braux.ezlang.Lang.NAME
import fr.braux.ezlang.Lang.VERSION

class Main {

  companion object {
    @JvmStatic fun main(args : Array<String>) {
      when {
        args.isEmpty() ->  Repl.loop(true)
        args[0] == "-v" -> println("$NAME $VERSION")
        args[0] == "-d" -> Repl.loop(true)
      }
    }
  }
}
