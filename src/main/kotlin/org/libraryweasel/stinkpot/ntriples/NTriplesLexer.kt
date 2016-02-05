/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

import org.libraryweasel.stinkpot.Lexer
import org.libraryweasel.stinkpot.NTriplesTokenType

class NTriplesLexer(input: String) : Lexer(input) {

    override fun nextToken(): Token {
        loop@ while (c != EOF) {
            return when (c) {
                '#'-> {comment(); continue@loop}
                ' ','\t','\n','\r'-> {ws(); continue@loop}
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

    fun iri() : Token {
        val stringBuilder = StringBuilder()
        consume() //ignore <
        while ( c != '>') {
            stringBuilder.append(c)
            consume()
        }
        consume() //ignore >
        return Token(NTriplesTokenType.IRIREF, stringBuilder.toString())
    }

    fun langTag() : Token {
        val stringBuilder = StringBuilder()
        consume() //ignore @
        while ( c != ' ') {
            stringBuilder.append(c);
            consume();
        }
        return Token(NTriplesTokenType.LANGTAG, stringBuilder.toString())
    }

    fun typeTag() : Token {
        val stringBuilder = StringBuilder()
        consume() //ignore ^
        if (c != '^') throw RuntimeException("Error parsing expecting ^^ after literal.")
        consume() //ignore ^
        return iri()
    }

    fun stringLiteralQuote() : Token {
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

    fun period() : Token {
        consume() //ignore .
        return Token(NTriplesTokenType.PERIOD, ".")
    }
}
