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
            when (c) {
                '#'-> {comment(); continue@loop}
                ' ','\t','\n','\r'-> {ws(); continue@loop}
                '_'-> return blankNode()
                '<'-> return iri()
                '@'-> return langTag()
                '^'-> return typeTag()
                '"', '\''-> return stringLiteralQuote() //TODO support ' for strings and also """ and ''' for multiline strings
                ';' -> return semicolon()
                ',' -> return comma()
                //'t', 'f' -> //TODO support boolean values
                //TODO support parsing numbers integers, decimal, double
                //TODO support checking ; for predicate list
                //TODO support checking , for object list
                //TODO support checking ( for collections
                //TODO support checking [ for unlabeled blank nodes
                //TODO support checking a for type predicate
                //TODO support @base or BASE
                //TODO support @prefix or PREFIX
                //TODO support : for prefixed names
                '.'-> return period()
                else-> throw RuntimeException("Error Parsing Found - $c")
            }
        }
        return Token(TurtleTokenType.EOF, "<EOF>")
    }

    fun semicolon() : Token<TurtleTokenType> {
        match(';')
        return Token(TurtleTokenType.SEMICOLON, ";")
    }

    fun comma() : Token<TurtleTokenType> {
        match(',')
        return Token(TurtleTokenType.COMMA, ",")
    }
}
