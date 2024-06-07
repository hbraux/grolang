package fr.braux.grolang

import java.util.*


fun main(args: Array<String>) {
  when {
    args.isEmpty() -> Repl.loop(true)
    args[0] == "-v" -> println("$LANG_NAME $LANG_VERSION")
    args[0] == "-d" -> Repl.loop(true)
  }
}

