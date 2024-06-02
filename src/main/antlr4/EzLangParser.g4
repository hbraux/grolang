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
     | simpleIdentifier
     | methodCall
     ;

methodCall
    : (IDENTIFIER | THIS) '(' expressions? ')'
    ;

assignment
    : (VAR|VAL)? typeParameter '=' NL* expression
    ;

typeParameter
    : simpleIdentifier (':' typeReference)?
    ;

typeReference
    : simpleIdentifier
    ;

simpleIdentifier
    : IDENTIFIER
    ;

literal
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | NULL_LITERAL
    | BOOLEAN_LITERAL
    | STRING_LITERAL
    | SYMBOL_LITERAL;
