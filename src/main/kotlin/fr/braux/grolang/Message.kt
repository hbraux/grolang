package fr.braux.grolang

import java.io.BufferedReader
import java.io.File
import java.io.FileInputStream
import java.io.InputStream


object Message {

  private val messages = mutableMapOf<String, String>()

  fun load(lang: String) {
    if (messages.isEmpty()) {
      val stream = (getResource(lang) ?: getResource("EN") ?: throw RuntimeException("no resource file for $lang"))
      BufferedReader(stream.reader()).readLines().forEach {
        if (it.contains("="))
          messages[it.substringBefore('=').trim()] = it.substringAfter('=').trim()
      }
    }}

  fun format(id: String, vararg args: Any) : String = messages[id]?.let { String.format(it, *args) } ?: "NO MESSAGE FOR $id"

  private fun getResource(lang: String): InputStream? {
    val file = File("/messages_$lang.properties")
    return if (file.exists()) FileInputStream(file) else this::class.java.classLoader.getResourceAsStream(file.name)
  }

}
