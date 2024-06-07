package fr.braux.grolang

import java.io.IOException

class LangException(message: String, val type: ExceptionType): IOException(message) {
  constructor(type: ExceptionType, vararg args: Any) : this(Lang.message(type.msgId, *args), type)
}

enum class ExceptionType(val msgId: String) {
  SYNTAX_ERROR("syntax_error"),
  ASSIGN_ERROR("assign_error"),
  INFER_ERROR("infer_error"),
  ALREADY_DEFINED("already_defined"),
  NOT_DEFINED("not_defined"),
  NOT_SET("not_set"),
  NOT_MUTABLE("not_mutable"),
  NOT_TYPE("not_type"),
  UNKNOWN_TYPE("unknown_type"),
  UNKNOWN_CLASS("unknown_class"),
  UNKNOWN_TOKEN("unknown_token")
}

