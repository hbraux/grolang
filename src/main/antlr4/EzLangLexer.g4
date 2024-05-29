lexer grammar EzLangLexer;

// Keywords
CLASS        : 'class';
ELSE         : 'else';
ENUM         : 'enum';
FUN          : 'fun';
IF           : 'if';
IMPORT       : 'import';
IS           : 'is';
NAMESPACE    : 'namespace';
PUBLIC       : 'public';
RETURN       : 'return';
SUPER        : 'super';
THIS         : 'this';
TRAIT        : 'trait';
VAR          : 'var';
VAL          : 'val';

// Literals
INTEGER_LITERAL : ('0' | [1-9] Digits?);
DECIMAL_LITERAL : (Digits '.' Digits? | '.' Digits) ExponentPart? ;
BOOLEAN_LITERAL : 'True' | 'False';
STRING_LITERAL  : '"' (~["\\\r\n] | EscapeSequence)* '"';
NULL_LITERAL    : 'Null';
SYMBOL_LITERAL  : '\'' Letter LetterOrDigit*;

// Separators
LPAREN : '(';
RPAREN : ')';
LBRACE : '{';
RBRACE : '}';
LBRACK : '[';
RBRACK : ']';
SEMI   : ';';
COMMA  : ',';
DOT    : '.';

// Operators
ASSIGN   : '=';
GT       : '>';
LT       : '<';
BANG     : '!';
TILDE    : '~';
QUESTION : '?';
COLON    : ':';
EQUAL    : '==';
LE       : '<=';
GE       : '>=';
NOTEQUAL : '!=';
AND      : '&&';
OR       : '||';
INC      : '++';
DEC      : '--';
ADD      : '+';
SUB      : '-';
MUL      : '*';
DIV      : '/';
BITAND   : '&';
BITOR    : '|';
CARET    : '^';
MOD      : '%';
ARROW    : '->';

// Newline, whitespace and comments
WS       : [ \t\r\n\u000C]+ -> channel(HIDDEN);
COMMENT  : '#' ~[\r\n]*    -> channel(HIDDEN);
NL       : '\n' | '\r' '\n'?;


// Identifiers
IDENTIFIER: Letter LetterOrDigit*;

// Fragment rules
fragment ExponentPart: [eE] [+-]? Digits;
fragment EscapeSequence:'\\' 'u005c'? [btnfr"'\\];
fragment Digits: [0-9]+;
fragment LetterOrDigit: Letter | [0-9];
fragment Letter: [a-zA-Z_];
