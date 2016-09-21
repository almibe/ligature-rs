/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.turtle.TurtleTokenType
import java.util.stream.Stream

abstract class Lexer<out T: TokenType>(val inputStream: Stream<String>) {
    var currentLine: String
    val iterator: Iterator<String>
    var pos: Int = 0
    var c: Char?
    val EOF: Char? = null

    init {
        iterator = inputStream.iterator()
        currentLine = iterator.next()
        c = currentLine[pos]
        System.out.println(currentLine)
    }

    fun match(c: Char) {
        if (this.c == c) consume()
        else throw RuntimeException("Error Parsing - Expected '$c' Found '${this.c}'")
    }

    fun consume() {
        pos++
        if (pos >= currentLine.length) nextLine()
        else c = currentLine[pos]
    }

    fun nextLine() {
        while (iterator.hasNext()) {
            currentLine = iterator.next()
            if (currentLine.length > 0) {
                pos = 0
                c = currentLine[pos]
                return
            }
        }
        inputStream.close()
        c = EOF
    }

    abstract fun nextToken(): Token<T>

    //Common methods for NTriples and Turtle
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
            stringBuilder.append(c)
            consume()
        }
        return Token(TurtleTokenType.LANGTAG, stringBuilder.toString())
    }

    fun typeTag() : Token<TurtleTokenType> {
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
