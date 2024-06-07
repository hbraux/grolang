package fr.braux.grolang

import org.junit.Test
import kotlin.test.assertEquals

class MessageTest {

  @Test
  fun test() {
    assertEquals(if (Message.lang == "FR") "Erreur de syntaxe" else "Cannot parse expression",
      Message.format("exception_syntax_error"))
  }
}
