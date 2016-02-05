/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

abstract class Parser<T : TokenType>(val lexer: Lexer<T>) {
    var lookAhead: Token<T>

    init {
        lookAhead = lexer.nextToken() //duplicated from method consume so var can be a null-safe type
    }

    fun match(tokenType: TokenType) : Token<T> {
        val token = lookAhead
        if (lookAhead.tokenType == tokenType) consume()
        else throw RuntimeException("Error Parsing - Expected [$tokenType] Found [${lookAhead.tokenType}]")
        return token
    }

    fun consume() : Unit {
        lookAhead = lexer.nextToken()
    }
}
