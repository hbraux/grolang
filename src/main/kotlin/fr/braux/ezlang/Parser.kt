package fr.braux.ezlang

import fr.braux.ezlang.parser.EzLangLexer
import fr.braux.ezlang.parser.EzLangParser
import fr.braux.ezlang.parser.EzLangParser.*
import fr.braux.ezlang.parser.EzLangParserBaseVisitor
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.misc.ParseCancellationException
import org.slf4j.LoggerFactory

object Parser {

  fun parse(expression: String): Expression {
    val lexer = EzLangLexer(CharStreams.fromString(expression))
    val parser = EzLangParser(CommonTokenStream(lexer))
    val visitor = object : EzLangParserBaseVisitor<Expression>() {

      override fun visitLiteral(ctx: LiteralContext): Expression = when (ctx.start.type) {
        INTEGER_LITERAL -> IntExpression(ctx.text.toLong())
        DECIMAL_LITERAL -> DecExpression(ctx.text.toDouble())
        STRING_LITERAL -> StringExpression(ctx.text.unquote())
        BOOLEAN_LITERAL -> BoolExpression(ctx.text.lowercase().toBoolean())
        NULL_LITERAL -> NullExpression
        SYMBOL_LITERAL -> SymbolExpression(ctx.text.substring(1))
        else -> throw ParserException("Unknown token ${ctx.start}")
      }
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
    override fun syntaxError(recognizer: Recognizer<*, *>, offendingSymbol: Any, line: Int, charPositionInLine: Int, msg: String, e: RecognitionException) {
      throw ParseCancellationException("at position $charPositionInLine, $msg")
    }
  }

  private fun String.unquote() = substring(1, length -1)
}