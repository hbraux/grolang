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
        INTEGER_LITERAL -> LiteralExpression(ctx.text.toLong(), "INT")
        DECIMAL_LITERAL -> LiteralExpression(ctx.text.toDouble(), "FLOATT")
        STRING_LITERAL -> LiteralExpression(ctx.text.unquote(), "STR")
        BOOLEAN_LITERAL -> LiteralExpression(ctx.text.lowercase().toBoolean(), "BOOL")
        NULL_LITERAL -> LiteralExpression(null ,"NULL")
        SYMBOL_LITERAL -> LiteralExpression(ctx.text.substring(1), "INT")
        else -> throw LangException(LangExceptionType.SYNTAX_ERROR, "Unknown token ${ctx.start}")
      }

      override fun visitIdentifier(ctx: IdentifierContext) = IdentifierExpression(ctx.text)

      override fun visitAssignment(ctx: AssignmentContext): Expression {
        val right = this.visit(ctx.expression())
        val declaredType = ctx.type?.text
        if (declaredType != null && declaredType != right.evalType)
          throw LangException(LangExceptionType.TYPE_ERROR, "Declared type $declaredType not matching ${right.evalType}")
        return AssignmentExpression(ctx.symbol.text, declaredType?: right.evalType , ctx.prefix.isVar(), right)
      }

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
  private fun Token.isVar() = this.text == "var"
}
