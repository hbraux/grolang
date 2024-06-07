package fr.braux.grolang

import java.util.*


object Messages {

  fun get(id: String, args: Array<out Any>) : String = messages[id]?.let { String.format(it, *args) } ?: "NO MESSAGE FOR $id"

  private val messages: Map<String, String> by lazy {
    val lang = Locale.getDefault().language.uppercase()
    (getResource(lang) ?: getResource("EN") ?: throw RuntimeException("no resource file for $lang"))
      .readText().split("\n")
      .filter { it.contains("=") }
      .map { it.substringBefore('=').trim() to it.substringAfter('=').trim() }
      .toMap()
  }

  private fun getResource(lang: String) = javaClass.getResource("/messages_$lang.properties")

}
