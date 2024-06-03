parser grammar GroParser;


options {
    tokenVocab = GroLexer;
}

block
    : LBRACE NL* statements NL* RBRACE
    ;

statements
    : (statement ((SEMI | NL)+ statement)*)?
    ;

statement
    : declaration
    | assignment
    | declarationAssignment
    | expression
    ;

expressions
    : expression (COMMA expression)*
    ;

expression
     : literal
     | identifier
     | methodCall
     ;

methodCall
    : (IDENTIFIER | THIS) LPAREN expressions? RPAREN
    ;

declaration
    : prefix=(VAR|VAL) symbol=IDENTIFIER COLON type=IDENTIFIER
    ;

assignment
    : symbol=IDENTIFIER ASSIGN expression
    ;

declarationAssignment
    : prefix=(VAR|VAL) symbol=IDENTIFIER (COLON type=IDENTIFIER)? ASSIGN expression
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
