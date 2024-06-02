package fr.braux.ezlang

import java.io.IOException

class LangException(val type: LangExceptionType, arg: Any): IOException(type.msg.format(arg))

enum class LangExceptionType(val msg: String) {
  SYNTAX_ERROR("Cannot parse expression"),
  TYPE_ERROR("Declared type '%s not matching"),
  UNKNOWN_TOKEN("Unknown token %s"),
  ALREADY_DEFINED("variable '%s is already defined"),
  NOT_DEFINED("variable '%s is not defined"),
  NOT_SET("variable '%s is unset"),
  NOT_MUTABLE("variable '%s is not mutable")
}

