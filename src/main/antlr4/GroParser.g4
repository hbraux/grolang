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

expression
     : functionCall
     | identifier
      | literal
     ;

functionCall
    : name=IDENTIFIER LPAREN (expression? (COMMA expression)*) RPAREN
    ;

declaration
    : prefix=(VAR|VAL) id=IDENTIFIER COLON type=IDENTIFIER
    ;

assignment
    : id=IDENTIFIER ASSIGN expression
    ;

declarationAssignment
    : prefix=(VAR|VAL) id=IDENTIFIER (COLON type=IDENTIFIER)? ASSIGN expression
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
