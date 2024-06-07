package fr.braux.grolang

import java.io.IOException
import java.text.MessageFormat

class LangException(message: String, val type: LangExceptionType): IOException(message) {
  constructor(type: LangExceptionType, vararg args: Any) : this(MessageFormat(type.msg).format(args.toList().toTypedArray()), type)
}

enum class LangExceptionType(val msg: String) {
  SYNTAX_ERROR("Cannot parse expression"),
  TYPE_ERROR("Declared type {0} not matching {1}"),
  TYPE_NOT_INFERRED("Cannot infer type of {0}"),
  UNKNOWN_TOKEN("Unknown token {0}"),
  ALREADY_DEFINED("variable {0} is already defined"),
  NOT_DEFINED("variable {0} is not defined"),
  NOT_SET("variable {0} is unset"),
  NOT_MUTABLE("variable {0} is not mutable"),
  NOT_EXPECTED_TYPE("variable {0} is not of expected type {0}"),
  UNKNOWN_TYPE("Unknown type {0}"),
  UNKNOWN_CLASS("Unknown class {0}")
}

