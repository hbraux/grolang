package fr.braux.ezlang.fr.braux.ezlang

import fr.braux.ezlang.Context
import fr.braux.ezlang.IntObject
import fr.braux.ezlang.LiteralObject
import fr.braux.ezlang.Parser
import org.junit.Test
import kotlin.test.assertEquals

class ParserTest {

  @Test
  fun testLiteralExpressions() {
    assertEquals(0L,  eval("0"))
    assertEquals(12345678912L,  eval("12345678912"))
    assertEquals(1.0,  eval("1.0"))
    assertEquals(1.234E11,  eval("12.340e10"))
    assertEquals(true,  eval("true"))
    assertEquals(false,  eval("false"))
    assertEquals(null,  eval("null"))
    assertEquals("some string",  eval(""""some string""""))
    assertEquals("Hello",  eval("'Hello"))
  }

  @Test
  fun testIdentifiers() {
    assertEquals(1L,  eval("someInt"))
  }

  @Test
  fun testDeclaration() {
    assertEquals("_declare('anInt,\"Int\",false)",  read("val anInt :Int"))
    assertEquals("_declare('aFloat,\"Float\",true)",  read("var aFloat :Float"))
    assertEquals("_block(_declare('myBool,\"Bool\",false),_assign('myBool, true))",  read("val myBool = true"))
    // type inference
    assertEquals(3L,  eval("val inferInt = 3"))
    assertEquals(3.0,  eval("val inferFloat = 3.0"))
    assertEquals(true,  eval("val inferBool = true"))
  }

  private val context = Context().also {
    it.assign("someInt", false, IntObject(1L))
  }

  private fun eval(s: String) = (Parser.parse(s).eval(context) as LiteralObject<*>).value
  private fun read(s: String) = Parser.parse(s).asString()
}
