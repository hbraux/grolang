package fr.braux.ezlang

import fr.braux.ezlang.parser.EzLangLexer
import fr.braux.ezlang.parser.EzLangParser
import fr.braux.ezlang.parser.EzLangParserBaseVisitor
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.misc.ParseCancellationException
import org.slf4j.LoggerFactory

object Parser {

  fun parse(expression: String): String {
    val lexer = EzLangLexer(CharStreams.fromString(expression))
    val parser = EzLangParser(CommonTokenStream(lexer))
    val visitor = object : EzLangParserBaseVisitor<String>() {

    }
    lexer.removeErrorListeners()
    parser.removeErrorListeners()
    lexer.addErrorListener(errorListener)
    parser.addErrorListener(errorListener)
    try {
      return visitor.visit(parser.expression())
    } catch (e: ParseCancellationException) {
      logger.error("Invalid syntax [ $expression ]", e)
      throw ParserException(e.localizedMessage)
    }
  }

  private val logger = LoggerFactory.getLogger(Parser::class.java)

  private val errorListener = object: BaseErrorListener() {
    override fun syntaxError(recognizer: Recognizer<*, *>?, offendingSymbol: Any?, line: Int, charPositionInLine: Int, msg: String, e: RecognitionException?) {
      throw ParseCancellationException("at position $charPositionInLine, $msg")
    }
  }
}