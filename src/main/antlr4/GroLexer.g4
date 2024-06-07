lexer grammar GroLexer;

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
THIS         : 'this';
TRAIT        : 'trait';
VAR          : 'var';
VAL          : 'val';

// Literals
INTEGER_LITERAL : SUB ? (Digit | [1-9] DigitOrSeparator* Digit);
DECIMAL_LITERAL : SUB ? (Digit* '.' Digit* | '.' Digit+) ExponentPart? ;
BOOLEAN_LITERAL : 'true' | 'false';
STRING_LITERAL  : '"' (~["\\\r\n] | EscapeSequence)* '"';
NIL_LITERAL     : 'nil';
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
fragment ExponentPart: [eE] [+-]? Digit+;
fragment EscapeSequence:'\\' 'u005c'? [btnfr"'\\];
fragment DigitOrSeparator: Digit | '_';
fragment LetterOrDigit: Letter | Digit;
fragment Digit: [0-9];
fragment Letter: [a-zA-Z_];
