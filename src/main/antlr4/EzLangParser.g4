parser grammar EzLangParser;


options {
    tokenVocab = EzLangLexer;
}

block
    : LBRACE NL* statements NL* RBRACE
    ;

statements
    : (statement ((SEMI | NL)+ statement)*)?
    ;

statement
    :  expression
    ;

expression
     : primaryExpression
     ;

primaryExpression
    : parenthesizedExpression
    | literal
    ;

parenthesizedExpression
    : LPAREN NL* expression NL* RPAREN
    ;

literal
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | NULL_LITERAL
    | BOOLEAN_LITERAL
    | STRING_LITERAL
    | SYMBOL_LITERAL;