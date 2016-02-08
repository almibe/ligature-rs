/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

import org.libraryweasel.stinkpot.Lexer
import org.libraryweasel.stinkpot.Token
import java.util.stream.Stream

class NTriplesLexer(input: Stream<String>) : Lexer<NTriplesTokenType>(input) {

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
        nextLine()
    }

    fun blankNode() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        match('_')
        match(':')
        while ( c != ' ') {
            stringBuilder.append(c)
            consume()
        }
        return Token(NTriplesTokenType.BLANK_NODE_LABEL, stringBuilder.toString())
    }

    fun iri() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        match('<')
        while ( c != '>') {
            stringBuilder.append(c)
            consume()
        }
        match('>')
        return Token(NTriplesTokenType.IRIREF, stringBuilder.toString())
    }

    fun langTag() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        match('@')
        while ( c != ' ') {
            stringBuilder.append(c);
            consume();
        }
        return Token(NTriplesTokenType.LANGTAG, stringBuilder.toString())
    }

    fun typeTag() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        match('^')
        match('^')
        return iri()
    }

    fun stringLiteralQuote() : Token<NTriplesTokenType> {
        val stringBuilder = StringBuilder()
        match('"')
        while ( c != '"') {
            stringBuilder.append(c)
            if (c == '\\') { //TODO handle escaped characters better
                consume()
                stringBuilder.append(c ?: ' ')
            }
            consume()
        }
        match('"')
        return Token(NTriplesTokenType.STRING_LITERAL_QUOTE, stringBuilder.toString())
    }

    fun period() : Token<NTriplesTokenType> {
        match('.')
        return Token(NTriplesTokenType.PERIOD, ".")
    }
}
