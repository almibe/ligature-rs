/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.turtle

import org.libraryweasel.stinkpot.Lexer
import org.libraryweasel.stinkpot.Token
import java.util.stream.Stream

class TurtleLexer(input: Stream<String>) : Lexer<TurtleTokenType>(input) {
    override fun nextToken(): Token<TurtleTokenType> {
        loop@ while (c != EOF) {
            return when (c) {
                '#'-> {comment(); continue@loop}
                ' ','\t','\n','\r'-> {ws(); continue@loop}
                '_'-> return blankNode()
                '<'-> return iri()
                '@'-> return langTag()
                '^'-> return typeTag()
                '"'-> return stringLiteralQuote()
                '.'-> return period()
                else-> throw RuntimeException("Error Parsing Found - $c")
            }
        }
        return Token(TurtleTokenType.EOF, "<EOF>")
    }
}
