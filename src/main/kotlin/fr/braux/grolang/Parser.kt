package fr.braux.grolang


import fr.braux.grolang.parser.GroLexer
import fr.braux.grolang.parser.GroParser
import fr.braux.grolang.parser.GroParser.*
import fr.braux.grolang.parser.GroParserBaseVisitor
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.misc.ParseCancellationException

object Parser {

  fun parse(str: String): Expr {
    val lexer = GroLexer(CharStreams.fromString(str))
    val parser = GroParser(CommonTokenStream(lexer))
    val visitor = object : GroParserBaseVisitor<Expr>() {

      override fun visitLiteral(ctx: LiteralContext) = when (ctx.start.type) {
        INTEGER_LITERAL -> IntObject(ctx.text.replace("_","").toLong())
        DECIMAL_LITERAL -> FloatObject(ctx.text.toDouble())
        STRING_LITERAL -> StrObject(ctx.text.unquote())
        BOOLEAN_LITERAL -> BoolObject(ctx.text.lowercase().toBoolean())
        NULL_LITERAL   -> NullObject
        SYMBOL_LITERAL -> SymbolObject(ctx.text.substring(1))
        else -> throw LangException(ExceptionType.UNKNOWN_TOKEN, ctx.start.text)
      }
      override fun visitIdentifier(ctx: IdentifierContext) = Identifier(ctx.text)
      override fun visitDeclaration(ctx: DeclarationContext) = DeclarationExpr(ctx.id.text, ctx.type.text, ctx.prefix.isVar())
      override fun visitAssignment(ctx: AssignmentContext) = AssignmentExpr(ctx.id.text, visit(ctx.expression()))

      override fun visitDeclarationAssignment(ctx: DeclarationAssignmentContext): Expr {
          val right = this.visit(ctx.expression())
          val declaredType = ctx.type?.text ?: right.getType()
          if (right.getType() != declaredType) throw LangException(ExceptionType.ASSIGN_ERROR, declaredType, right.getType() )
        return BlockExpr(DeclarationExpr(ctx.id.text, declaredType, ctx.prefix.isVar()), AssignmentExpr(ctx.id.text, right))
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
      throw LangException(ExceptionType.SYNTAX_ERROR, e.localizedMessage)
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
