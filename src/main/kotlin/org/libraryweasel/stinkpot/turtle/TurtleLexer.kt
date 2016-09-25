/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.turtle

import org.libraryweasel.stinkpot.Lexer
import org.libraryweasel.stinkpot.Token
import java.util.stream.Stream

class TurtleLexer(input: Stream<String>) : Lexer<TurtleTokenType>(input) {

    override fun nextToken(): Token<TurtleTokenType> {
        while (c != EOF) {
            when (c) {
                '#'-> {comment()}
                ' ','\t','\n','\r'-> {ws()}
                '_'-> return blankNode()
                '<'-> return iri()
                '@'-> return langTagPrefixBase()
                '^'-> return typeTag()
                '"', '\''-> return stringLiteralQuote()
                ';' -> return semicolon()
                ',' -> return comma()
                '[' -> return unlabeledBlankNodeOpen()
                ']' -> return unlabeledBlankNodeClose()
                //TODO support checking ( for collections
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

    override fun stringLiteralQuote() : Token<TurtleTokenType> {
        val quotationCharacter = c!!
        match(quotationCharacter)
        if ( c == quotationCharacter) { //two quotationCharacters in a row
            match(quotationCharacter)
            if (c == quotationCharacter) { //three quotationCharacters in a row
                match(quotationCharacter)
                return stringLiteralTripleQuote(quotationCharacter) //handle triple quote
            }
            return Token(TurtleTokenType.STRING_LITERAL_QUOTE, "") //return empty string
        } else {
            return stringLiteralSingleQuote(quotationCharacter) //handle single quote
        }
    }

    fun stringLiteralSingleQuote(quotationCharacter: Char) : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        while ( c != quotationCharacter) {
            stringBuilder.append(c)
            if (c == '\\') { //TODO handle escaped characters better
                consume()
                stringBuilder.append(c ?: ' ')
            }
            consume()
        }
        match(quotationCharacter)
        return Token(TurtleTokenType.STRING_LITERAL_QUOTE, stringBuilder.toString())
    }

    fun stringLiteralTripleQuote(quotationCharacter: Char) : Token<TurtleTokenType> {
        val stringBuilder = StringBuilder()
        var quotationCharacterCount = 0
        while (quotationCharacterCount < 3) {
            if (c == quotationCharacter) {
                quotationCharacterCount++
            } else {
                if (quotationCharacterCount == 1) {
                    stringBuilder.append(quotationCharacter)
                }
                if (quotationCharacterCount == 2) {
                    stringBuilder.append(quotationCharacter)
                    stringBuilder.append(quotationCharacter)
                }
                quotationCharacterCount = 0
                stringBuilder.append(c)
                if (c == '\\') { //TODO handle escaped characters better
                    consume()
                    stringBuilder.append(c ?: ' ')
                }
            }
            consume()
        }
        return Token(TurtleTokenType.STRING_LITERAL_QUOTE, stringBuilder.toString())
    }

    fun unlabeledBlankNodeOpen() : Token<TurtleTokenType> {
        match('[')
        return Token(TurtleTokenType.UNLABELED_BLANK_NODE_OPEN, "[")
    }

    fun unlabeledBlankNodeClose() : Token<TurtleTokenType> {
        match(']')
        return Token(TurtleTokenType.UNLABELED_BLANK_NODE_CLOSE, "]")
    }
}
