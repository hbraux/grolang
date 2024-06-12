package fr.braux.grolang.fr.braux.ezlang

import fr.braux.grolang.*
import org.junit.BeforeClass
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class ParserTest {

  @Test
  fun testLiteralExpressions() {
    // ints
    assertEquals(0L,  eval("0"))
    assertEquals(2L,  eval("2"))
    assertEquals(-3L,  eval("-3"))
    assertEquals(12345678912L,  eval("12345678912"))
    assertEquals(12345678912L,  eval("12_345_678_912"))
    assertEquals(-12000L,  eval("-1200_0"))
    // floats
    assertEquals(1.2,  eval("1.2"))
    assertEquals(0.01,  eval(".01"))
    assertEquals(-1.0,  eval("-1."))
    assertEquals(1.234E11,  eval("12.340e10"))
    assertEquals(1.234E11,  eval("12.340E10"))
    assertEquals(-1.0E10,  eval("-1.E10"))
    // bools
    assertEquals(true,  eval("true"))
    assertEquals(false,  eval("false"))
    // null
    assertEquals(null,  eval("null"))
    // strings
    assertEquals("some string",  eval(""""some string""""))
    // symbols
    assertEquals("Hello",  eval("'Hello"))
  }

  @Test
  fun testDeclaration() {
    assertEquals("defval('anInt,'Int)",  read("val anInt :Int"))
    assertEquals("defvar('aFloat,'Float)",  read("var aFloat :Float"))
    assertEquals("defval('myBool,'Bool); assign('myBool, true)",  read("val myBool = true"))
  }

  @Test
  fun testFunctionCall() {
    assertEquals("print('a,1,true)",  read("print(a, 1, true)"))
  }

  @Test
  fun testTypeInference() {
    assertEquals(3L,  eval("val inferInt = 3"))
    assertEquals(3.0,  eval("val inferFloat = 3.0"))
    assertEquals(true,  eval("val inferBool = true"))
    assertEquals("Declared type is :Int whereas value is :Bool", assertFailsWith(LangException::class) { read("val badInt :Int = true") }.message)
  }


  @Test
  fun testIdentifiers() {
    assertEquals(1L,  eval("someInt"))
  }

  private val context = Context().also {
    it.declare("someInt",  IntObject(1L))
  }

  private fun eval(s: String) = (Parser.parse(s).eval(context) as Literal<*>).value
  private fun read(s: String) = Parser.parse(s).asString()

  companion object {
    @JvmStatic
    @BeforeClass
    fun before() {
      Lang.init()
    }
  }
}
