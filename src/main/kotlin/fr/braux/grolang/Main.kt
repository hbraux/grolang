package fr.braux.grolang

import java.util.*


fun main(args: Array<String>) {
  Lang.init(Locale.getDefault().language.uppercase())
  when {
    args.isEmpty() -> Repl.loop(false)
    args[0] == "-v" -> println("$LANG_NAME $LANG_VERSION")
    args[0] == "-d" -> Repl.loop(true)
  }
}

