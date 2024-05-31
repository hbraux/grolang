package fr.braux.ezlang.fr.braux.ezlang

import fr.braux.ezlang.Context
import fr.braux.ezlang.Lang
import fr.braux.ezlang.Parser
import org.junit.Test
import kotlin.test.assertEquals

class ParserTest {

  @Test
  fun testLiterals() {
    assertEquals(0L,  eval("0"))
    assertEquals(12345678912L,  eval("12345678912"))
    assertEquals(1.0,  eval("1.0"))
    assertEquals(1.234E11,  eval("12.340e10"))
    assertEquals(true,  eval("True"))
    assertEquals(false,  eval("False"))
    //assertEquals(Lang.Null,  eval("Null"))
    assertEquals("some string",  eval(""""some string""""))
    assertEquals("Hello",  eval("'Hello"))
  }

  private val context = Context()
  private fun eval(s: String) = Parser.parse(s).eval(context)
}
