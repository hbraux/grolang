package fr.braux.ezlang

import java.io.IOException

class LangException(val type: LangExceptionType, message: String): IOException(message)

enum class LangExceptionType {
  TYPE_ERROR,
  SYNTAX_ERROR,
  ALREADY_DEFINED,
  NOT_DEFINED
}

