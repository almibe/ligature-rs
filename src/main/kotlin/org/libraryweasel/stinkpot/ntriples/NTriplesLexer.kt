/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

import org.libraryweasel.stinkpot.Lexer
import org.libraryweasel.stinkpot.Token

class NTriplesLexer(input: String) : Lexer<NTriplesTokenType>(input) {

    override fun nextToken(): Token<NTriplesTokenType> {
        loop@ while (c != EOF) {
            return when (c) {
                '#'-> {comment(); continue@loop}
                ' ','\t','\n','\r'-> {ws(); continue@loop}
                '_'-> return blankNode();
                '<'-> return iri();
                '@'-> return langTag();
                '^'-> return typeTag();
                '"'-> return stringLiteralQuote();
                '.'-> return period();
                else-> throw RuntimeException("Error Parsing Found - $c")
            }
        }
        return Token(NTriplesTokenType.EOF, "<EOF>")
    }

    fun ws() : Unit {
        while (c == ' ' || c == '\t' || c == '\n' || c == '\r') consume()
    }

    fun comment() : Unit {
        while (c != '\n' && c != EOF) {
            consume()
        }
    }

    fun blankNode() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        consume() //ignore _
        if (c != ':' && c != null) throw RuntimeException("Error parsing expecting _ after : for blank nodes.")
        consume() //ignore :
        while ( c != ' ') {
            stringBuilder.append(c)
            consume()
        }
        return Token(NTriplesTokenType.BLANK_NODE_LABEL, stringBuilder.toString())
    }

    fun iri() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        consume() //ignore <
        while ( c != '>') {
            stringBuilder.append(c)
            consume()
        }
        consume() //ignore >
        return Token(NTriplesTokenType.IRIREF, stringBuilder.toString())
    }

    fun langTag() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        consume() //ignore @
        while ( c != ' ') {
            stringBuilder.append(c);
            consume();
        }
        return Token(NTriplesTokenType.LANGTAG, stringBuilder.toString())
    }

    fun typeTag() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        consume() //ignore ^
        if (c != '^') throw RuntimeException("Error parsing expecting ^^ after literal.")
        consume() //ignore ^
        return iri()
    }

    fun stringLiteralQuote() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        consume() //ignore "
        while ( c != '"') {
            stringBuilder.append(c)
            if (c == '\\') { //TODO handle escaped characters better
                consume()
                stringBuilder.append(c ?: ' ')
            }
            consume()
        }
        consume() //ignore "
        return Token(NTriplesTokenType.STRING_LITERAL_QUOTE, stringBuilder.toString())
    }

    fun period() : Token<NTriplesTokenType> {
        consume() //ignore .
        return Token(NTriplesTokenType.PERIOD, ".")
    }
}
