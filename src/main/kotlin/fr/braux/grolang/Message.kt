package fr.braux.grolang

import java.util.*


object Message {

  val lang = Locale.getDefault().language.uppercase()

  fun format(id: String, vararg args: Any) : String = messages[id]?.let { String.format(it, *args) } ?: "NO MESSAGE FOR $id"

  private val messages: Map<String, String> by lazy {
    (getResource(lang) ?: getResource("EN") ?: throw RuntimeException("no resource file messages_$lang.properties"))
      .readText().split("\n")
      .filter { it.contains("=") }
      .map { it.substringBefore('=').trim() to it.substringAfter('=').trim() }
      .toMap()
  }

  private fun getResource(lang: String) = javaClass.getResource("/messages_$lang.properties")

}
