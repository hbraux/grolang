package fr.braux.ezlang

import fr.braux.ezlang.parser.EzLangLexer
import fr.braux.ezlang.parser.EzLangParser
import fr.braux.ezlang.parser.EzLangParser.*
import fr.braux.ezlang.parser.EzLangParserBaseVisitor
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.misc.ParseCancellationException
import kotlin.Any

object Parser {

  fun parse(str: String): Expression {
    val lexer = EzLangLexer(CharStreams.fromString(str))
    val parser = EzLangParser(CommonTokenStream(lexer))
    val visitor = object : EzLangParserBaseVisitor<Expression>() {

      override fun visitLiteral(ctx: LiteralContext) = when (ctx.start.type) {
        INTEGER_LITERAL -> LiteralExpression(ctx.text.toLong())
        DECIMAL_LITERAL -> LiteralExpression(ctx.text.toDouble())
        STRING_LITERAL -> LiteralExpression(ctx.text.unquote())
        BOOLEAN_LITERAL -> LiteralExpression(ctx.text.lowercase().toBoolean())
        NULL_LITERAL -> LiteralExpression(null)
        SYMBOL_LITERAL -> LiteralExpression(ctx.text.substring(1))
        else -> throw LangException(LangExceptionType.SYNTAX_ERROR, "Unknown token ${ctx.start}")
      }

      override fun visitSimpleIdentifier(ctx: SimpleIdentifierContext) = IdentifierExpression(ctx.text)

    }
    lexer.removeErrorListeners()
    parser.removeErrorListeners()
    lexer.addErrorListener(errorListener)
    parser.addErrorListener(errorListener)
    try {
      return visitor.visit(parser.expression())
    } catch (e: ParseCancellationException) {
      throw LangException(LangExceptionType.SYNTAX_ERROR, e.localizedMessage)
    }
  }

  private val errorListener = object: BaseErrorListener() {
    override fun syntaxError(recognizer: Recognizer<*, *>, offendingSymbol: Any, line: Int, charPositionInLine: Int, msg: String, e: RecognitionException) {
      throw ParseCancellationException("at position $charPositionInLine, $msg")
    }
  }

  private fun String.unquote() = substring(1, length -1)
}
