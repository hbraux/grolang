package fr.braux.grolang

import org.junit.BeforeClass
import org.junit.Test

class LangTest {



  @Test
  fun testPrint() {
    eval("print(x)")
  }

  private fun eval(s: String) = Parser.parse(s).eval(context)

  private val context = Context().also {
    it.declare("x",  IntObject(1L))
  }

  companion object {
    @JvmStatic
    @BeforeClass
    fun before() {
      Lang.init()
    }
  }
}
