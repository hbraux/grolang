package fr.braux.grolang


import fr.braux.grolang.parser.GroLexer
import fr.braux.grolang.parser.GroParser
import fr.braux.grolang.parser.GroParser.*
import fr.braux.grolang.parser.GroParserBaseVisitor
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.misc.ParseCancellationException

object Parser {
  
  fun parse(str: String): Expression {
    val lexer = GroLexer(CharStreams.fromString(str))
    val parser = GroParser(CommonTokenStream(lexer))
    val visitor = object : GroParserBaseVisitor<Expression>() {

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
          val name = ctx.symbol.text
          val right = this.visit(ctx.expression())
          val inferredType = ctx.type?.text ?: right.evalType ?: throw LangException(LangExceptionType.TYPE_NOT_INFERRED, name)     
          if (inferredType != right.evalType) throw LangException(LangExceptionType.TYPE_ERROR, name)
        return BlockExpression(
          DeclarationExpression(ctx.symbol.text, inferredType, ctx.prefix.isVar()),
          AssignmentExpression(ctx.symbol.text, right))
      }

    }
    lexer.removeErrorListeners()
    parser.removeErrorListeners()
    lexer.addErrorListener(errorListener)
    parser.addErrorListener(errorListener)
    try {
      return visitor.visit(parser.statement())
    } catch (e: ParseCancellationException) {
      throw LangException(LangExceptionType.SYNTAX_ERROR, e.localizedMessage)
    }
  }

  private val errorListener = object: BaseErrorListener() {
    override fun syntaxError(recognizer: Recognizer<*, *>?, offendingSymbol: Any?, line: Int, charPositionInLine: Int, msg: String, e: RecognitionException?) {
      throw ParseCancellationException("at position $charPositionInLine, $msg")
    }
  }

  private fun String.unquote() = substring(1, length -1)
  private fun Token.isVar() = this.text == "var"
}
