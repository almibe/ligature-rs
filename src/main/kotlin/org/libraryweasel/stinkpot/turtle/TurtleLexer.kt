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
                '@'-> return langTagPrefixBase()
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
                else-> return characterToken() //A catch all for now.  There might be a better way to handle this.
            }
        }
        return Token(TurtleTokenType.EOF, "<EOF>")
    }

    override fun iri() : Token<TurtleTokenType> { //override from base iri method so relative iris can be handled
        val stringBuilder = StringBuilder()
        match('<')
        while ( c != '>') {
            stringBuilder.append(c)
            consume()
        }
        match('>')
        val result = stringBuilder.toString()
        return if (result.startsWith("http")) { //TODO this only handles IRI that start with http
            Token(TurtleTokenType.IRIREF, stringBuilder.toString())
        } else {
            Token(TurtleTokenType.RELATIVE_IRI, stringBuilder.toString())
        }
    }

    fun langTagPrefixBase() : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        match('@')
        while ( c != ' ') {
            stringBuilder.append(c)
            consume()
        }
        val result = stringBuilder.toString()
        return when(result) {
            "prefix" -> Token(TurtleTokenType.PREFIX, result)
            "base" -> Token(TurtleTokenType.BASE, result)
            else -> Token(TurtleTokenType.LANGTAG, result)
        }
    }

    fun characterToken() : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        while ( c != ' ') {
            stringBuilder.append(c)
            consume()
        }
        return Token(TurtleTokenType.CHARACTER_TOKEN, stringBuilder.toString())
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
