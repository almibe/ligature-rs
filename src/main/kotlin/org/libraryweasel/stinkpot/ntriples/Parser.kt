/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

abstract class Parser(val lexer: Lexer) {
    var lookAhead: Token = Token(NTriplesTokenType.BLANK_NODE_LABEL, "test")

    init {
        consume()
    }

    fun match(tokenType: NTriplesTokenType) : Token {
        val token = lookAhead
        if (lookAhead.tokenType == tokenType) consume()
        else throw RuntimeException("Error Parsing - Expected [$tokenType] Found [${lookAhead.tokenType}]")
        return token
    }

    fun consume() : Unit {
        lookAhead = lexer.nextToken()
    }
}
