package fr.braux.grolang

import java.io.IOException

class LangException(message: String, val type: LangExceptionType): IOException(message) {

  constructor(type: LangExceptionType, vararg args: Any) : this(Messages.get(type.msgId(), args), type)
}

enum class LangExceptionType() {
  SYNTAX_ERROR,
  ASSIGN_ERROR,
  CANNOT_INFER,
  UNKNOWN_TOKEN,
  ALREADY_DEFINED,
  NOT_DEFINED,
  NOT_SET,
  NOT_MUTABLE,
  NOT_EXPECTED_TYPE,
  UNKNOWN_TYPE,
  UNKNOWN_CLASS;

  fun msgId(): String = "exception_${name.lowercase()}"
}

