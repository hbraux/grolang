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
    : expression
    | assignment
    ;

expressions
    : expression (',' expression)*
    ;

expression
     : literal
     | identifier
     | methodCall
     ;

methodCall
    : (IDENTIFIER | THIS) '(' expressions? ')'
    ;

assignment
    : prefix=(VAR|VAL) symbol=IDENTIFIER (':' type=IDENTIFIER)? '=' expression
    ;

identifier
    : IDENTIFIER
    ;

literal
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | NULL_LITERAL
    | BOOLEAN_LITERAL
    | STRING_LITERAL
    | SYMBOL_LITERAL;
