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
        INTEGER_LITERAL -> LiteralExpression(ctx.text.toLong(), TYPE_INT)
        DECIMAL_LITERAL -> LiteralExpression(ctx.text.toDouble(), TYPE_FLOAT)
        STRING_LITERAL -> LiteralExpression(ctx.text.unquote(), TYPE_STR)
        BOOLEAN_LITERAL -> LiteralExpression(ctx.text.lowercase().toBoolean(), TYPE_BOOL)
        NULL_LITERAL -> LiteralExpression(null, TYPE_NULL)
        SYMBOL_LITERAL -> LiteralExpression(ctx.text.substring(1), TYPE_SYMBOL)
        else -> throw LangException(LangExceptionType.UNKNOWN_TOKEN, ctx.start.text)
      }

      override fun visitIdentifier(ctx: IdentifierContext) = IdentifierExpression(ctx.text)
      override fun visitDeclaration(ctx: DeclarationContext) = DeclarationExpression(ctx.symbol.text, ctx.type.text, ctx.prefix.isVar())
      override fun visitAssignment(ctx: AssignmentContext) = AssignmentExpression(ctx.symbol.text, visit(ctx.expression()))

      override fun visitDeclarationAssignment(ctx: DeclarationAssignmentContext): Expression {
        val right = this.visit(ctx.expression())
        ctx.type?.text?.let {
          if (it != right.evalType)
            throw LangException(LangExceptionType.TYPE_ERROR, it)
          return BlockExpression(listOf(AssignmentExpression(ctx.symbol.text, right)))
        }
        return AssignmentExpression(ctx.symbol.text, right)
      }

    }
    lexer.removeErrorListeners()
    parser.removeErrorListeners()
    lexer.addErrorListener(errorListener)
    parser.addErrorListener(errorListener)
    try {
      return visitor.visit(parser.expression())
    } catch (e: ParseCancellationException) {
      throw LangException(LangExceptionType.PARSE_ERROR, e.localizedMessage)
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
