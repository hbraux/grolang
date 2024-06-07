package fr.braux.grolang

import org.junit.Test
import kotlin.test.assertEquals

class MessageTest {

  @Test
  fun test() {
    Message.load("EN")
    assertEquals( "Unknown class :Foo", Message.format("unknown_class", "Foo"))
  }
}
