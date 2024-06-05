package fr.braux.grolang

import java.io.IOException

class LangException(message: String, val type: LangExceptionType): IOException(message) {
  constructor(type: LangExceptionType, vararg args: Any) : this(type.msg.format(*args), type)
}

enum class LangExceptionType(val msg: String) {
  SYNTAX_ERROR("Cannot parse expression"),
  TYPE_ERROR("Declared type '%s not matching"),
  TYPE_NOT_INFERRED("Cannot infer type of '%s"),
  UNKNOWN_TOKEN("Unknown token %s"),
  ALREADY_DEFINED("variable '%s is already defined"),
  NOT_DEFINED("variable '%s is not defined"),
  NOT_SET("variable '%s is unset"),
  NOT_MUTABLE("variable '%s is not mutable"),
  NOT_EXPECTED_TYPE("variable '%s is not of expected type '%s"),
  UNKNOWN_TYPE("Unknown type '%s"),
  UNKNOWN_CLASS("'Unknown class '%s")
}

