/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

import org.libraryweasel.stinkpot.Lexer
import org.libraryweasel.stinkpot.Token
import org.libraryweasel.stinkpot.turtle.TurtleTokenType
import java.util.stream.Stream

class NTriplesLexer(input: Stream<String>) : Lexer<TurtleTokenType>(input) {

    override fun nextToken(): Token<TurtleTokenType> {
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
        return Token(TurtleTokenType.EOF, "<EOF>")
    }

    fun ws() : Unit {
        while (c == ' ' || c == '\t' || c == '\n' || c == '\r') consume()
    }

    fun comment() : Unit {
        nextLine()
    }

    fun blankNode() : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        match('_')
        match(':')
        while ( c != ' ') {
            stringBuilder.append(c)
            consume()
        }
        return Token(TurtleTokenType.BLANK_NODE_LABEL, stringBuilder.toString())
    }

    fun iri() : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        match('<')
        while ( c != '>') {
            stringBuilder.append(c)
            consume()
        }
        match('>')
        return Token(TurtleTokenType.IRIREF, stringBuilder.toString())
    }

    fun langTag() : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        match('@')
        while ( c != ' ') {
            stringBuilder.append(c);
            consume();
        }
        return Token(TurtleTokenType.LANGTAG, stringBuilder.toString())
    }

    fun typeTag() : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        match('^')
        match('^')
        return iri()
    }

    fun stringLiteralQuote() : Token<TurtleTokenType> {
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
        return Token(TurtleTokenType.STRING_LITERAL_QUOTE, stringBuilder.toString())
    }

    fun period() : Token<TurtleTokenType> {
        match('.')
        return Token(TurtleTokenType.PERIOD, ".")
    }
}
