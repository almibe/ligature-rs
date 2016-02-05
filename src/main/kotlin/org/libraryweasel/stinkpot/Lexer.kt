/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

abstract class Lexer<T : TokenType>(val input:String) {
    var pos: Int = 0
    var c: Char?
    val EOF: Char? = null

    init {
        c = input[pos]
    }

    fun match(c: Char) {
        if (this.c == c) consume()
        else throw RuntimeException("Error Parsing - Expected '$c' Found '${this.c}'")
    }

    fun consume() {
        pos++
        if (pos >= input.length) c = EOF
        else c = input[pos]
    }

    abstract fun nextToken(): Token<T>
}
