package fr.braux.grolang

import fr.braux.grolang.parser.GroLexer
import fr.braux.grolang.parser.GroParser
import fr.braux.grolang.parser.GroParser.*
import fr.braux.grolang.parser.GroParserBaseVisitor
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.misc.ParseCancellationException
import java.io.IOException

object Parser {

  fun parse(str: String): Expr {
    val lexer = GroLexer(CharStreams.fromString(str))
    val parser = GroParser(CommonTokenStream(lexer))
    val visitor = object : GroParserBaseVisitor<Expr>() {
      override fun visitLiteral(ctx: LiteralContext) = when (ctx.start.type) {
        INTEGER_LITERAL -> IntExpr(ctx.text.replace("_","").toLong())
        DECIMAL_LITERAL -> FloatExpr(ctx.text.toDouble())
        STRING_LITERAL -> StrExpr(ctx.text.unquote())
        BOOLEAN_LITERAL -> BoolExpr(ctx.text.lowercase().toBoolean())
        NULL_LITERAL   -> NullExpr
        SYMBOL_LITERAL -> SymbolExpr(ctx.text.substring(1))
        else -> throw IOException("Unknown token >> ${ctx.start} <<")
      }
      override fun visitIdentifier(ctx: IdentifierContext) = IdentifierExpr(ctx.text)
      override fun visitDeclaration(ctx: DeclarationContext) = DeclarationExpr(ctx.id.text, ctx.type.text, ctx.prefix.isVar())
      override fun visitAssignment(ctx: AssignmentContext) = AssignmentExpr(ctx.id.text, visit(ctx.expression()))
      override fun visitDeclarationAssignment(ctx: DeclarationAssignmentContext) = this.visit(ctx.expression()).let { right ->
        BlockExpr(DeclarationExpr(ctx.id.text, ensureType(right.type, ctx.type?.text), ctx.prefix.isVar()), AssignmentExpr(ctx.id.text, right))
      }
      override fun visitFunctionCall(ctx: FunctionCallContext) = CallExpr(ctx.name.text, ctx.expression().map { visit(it) })
    }
    lexer.removeErrorListeners()
    parser.removeErrorListeners()
    lexer.addErrorListener(errorListener)
    parser.addErrorListener(errorListener)
    try {
      return visitor.visit(parser.statement())
    } catch (e: ParseCancellationException) {
      throw IOException("Syntax error:" + e.localizedMessage)
    }
  }

  private fun ensureType(expected: String, declared: String?): String {
    if (declared != null && declared != expected) throw IOException("inconsistent declared type :$declared")
    return expected
  }


  private val errorListener = object: BaseErrorListener() {
    override fun syntaxError(recognizer: Recognizer<*, *>?, offendingSymbol: Any?, line: Int, charPositionInLine: Int, msg: String, e: RecognitionException?) {
      throw ParseCancellationException("at position $charPositionInLine, $msg")
    }
  }

  private fun String.unquote() = substring(1, length -1)
  private fun Token.isVar() = this.text == "var"
}
