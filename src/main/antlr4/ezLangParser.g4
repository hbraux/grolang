parser grammar ezLangParser;

options {
    tokenVocab = ezLangLexer;
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
    : INT_LITERAL
    | DEC_LITERAL
    | NULL_LITERAL
    | BOOL_LITERAL
    | STRING_LITERAL;
