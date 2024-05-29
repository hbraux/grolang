package fr.braux.ezlang.fr.braux.ezlang

import fr.braux.ezlang.NullValue
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
    assertEquals(NullValue,  eval("Null"))
  }

  private fun eval(s: String) = Parser.parse(s).eval()
}