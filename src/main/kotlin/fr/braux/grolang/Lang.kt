package fr.braux.grolang


import java.io.BufferedReader
import java.io.File

const val LANG_NAME = "groLang"
const val LANG_VERSION = "0.1"
const val STRING_NULL = "null"

// primary types (which are also classes)
const val TYPE_ANY = "Any"
const val TYPE_CLASS = "Class"
const val TYPE_INT = "Int"
const val TYPE_FLOAT = "Float"
const val TYPE_STR = "Str"
const val TYPE_BOOL = "Bool"
const val TYPE_SYMBOL = "Symbol"

const val TYPE_FUNCTION = "Function"

object Lang {

  fun init(language: String = "EN") {
    // load messages
    val stream = javaClass.classLoader.getResourceAsStream(File("messages_$language.properties").name)
      ?: throw RuntimeException("no resource file for $language")
    BufferedReader(stream.reader()).readLines().forEach {
      if (it.contains("=")) messages[it.substringBefore('=').trim()] = it.substringAfter('=').trim()
    }


  }

  fun message(id: String, vararg args: Any): String = messages[id]?.let { String.format(it, *args) } ?: "NO MESSAGE $id"

  private val messages = mutableMapOf<String, String>()
}
