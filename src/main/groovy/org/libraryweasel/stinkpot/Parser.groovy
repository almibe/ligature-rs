/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.NTriplesTokenType

abstract class Parser {
    final Lexer lexer
    Token lookAhead

    public Parser(Lexer lexer) {
        this.lexer = lexer
        consume()
    }

    Token match(NTriplesTokenType tokenType) {
        Token token = lookAhead
        if (lookAhead.tokenType == tokenType) consume()
        else throw new RuntimeException("expecting $tokenType found $lookAhead")
        return token
    }

    public void consume() {
        lookAhead = lexer.nextToken()
    }
}
