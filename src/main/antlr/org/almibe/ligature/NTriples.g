grammar NTriples;

@lexer::header {
    package org.almibe.ligature;
}

@parser::header {
    package org.almibe.ligature;
}

document
  : triple *
;

triple
  : subject predicate object PERIOD
;

subject
  : iri | blankNode
;

predicate
  : iri
;

object
  : iri | blankNode | literal | typedLiteral | languageLiteral
;

iri
  : '<' (PN_CHARS | UCHAR)* '>'
;

literal
  : '"' (ECHAR | UCHAR)* '"'
;

typedLiteral
  : literal '^^' iri
;

languageLiteral
  : literal LANG_TAG
;

blankNode
  : '_:' PN_CHARS_U
;

PERIOD
  : '.'
;

HEX
  : [0-9] | [A-F] | [a-f]
;

UCHAR
  : '\\u' HEX HEX HEX HEX | '\\U' HEX HEX HEX HEX HEX HEX HEX HEX
;

ECHAR
  : '\\' [tbnrf"'\\]
;

PN_CHARS_BASE
  : 'A' .. 'Z' | 'a' .. 'z' | '\u00C0' .. '\u00D6' | '\u00D8' .. '\u00F6' | '\u00F8' .. '\u02FF' | '\u0370' .. '\u037D' | '\u037F' .. '\u1FFF' | '\u200C' .. '\u200D' | '\u2070' .. '\u218F' | '\u2C00' .. '\u2FEF' | '\u3001' .. '\uD7FF' | '\uF900' .. '\uFDCF' | '\uFDF0' .. '\uFFFD'
;
PN_CHARS_U
  : PN_CHARS_BASE | '_'
;
PN_CHARS
  : PN_CHARS_U | '-' | [0-9] | '\u00B7' | [\u0300-\u036F] | [\u203F-\u2040]
;

LANG_TAG
  : '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*
;

WS
  : (' ' | '\t' | '\n' | '\r')+ -> skip
;
