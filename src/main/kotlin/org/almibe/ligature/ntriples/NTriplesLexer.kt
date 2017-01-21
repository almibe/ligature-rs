/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.Lexer
import org.almibe.ligature.Token
import org.almibe.ligature.turtle.TurtleTokenType
import java.util.stream.Stream

class NTriplesLexer(input: Stream<String>) : Lexer<TurtleTokenType>(input) {
    override fun nextToken(): Token<TurtleTokenType> {
        loop@ while (c != EOF) {
            when (c) {
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