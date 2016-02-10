/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import java.util.stream.Stream

abstract class Lexer<T : TokenType>(val inputStream: Stream<String>) {
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
        if (iterator.hasNext()) {
            currentLine = iterator.next()
            pos = 0
            c = currentLine[pos]
        } else {
            inputStream.close()
            c = EOF
        }
    }

    abstract fun nextToken(): Token<T>
}