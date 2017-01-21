/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.*
import org.almibe.ligature.*
import org.almibe.ligature.turtle.TurtleTokenType

class NTriplesParser(lexer: NTriplesLexer, val handler: (Triple) -> Unit) : Parser<TurtleTokenType>(lexer) {
    fun start() : Unit {
        while (lookAhead.tokenType != TurtleTokenType.EOF) {
            triple()
        }
    }

    fun triple() : Unit {
        val subject = subject()
        val predicate = predicate()
        val `object` = `object`()
        match(TurtleTokenType.PERIOD)
        handler(Triple(subject, predicate, `object`))
    }

    fun subject() : Subject {
        when (lookAhead.tokenType) {
            TurtleTokenType.IRIREF -> {
                val token = match(TurtleTokenType.IRIREF)
                return IRI(token.text)
            }
            TurtleTokenType.BLANK_NODE_LABEL -> {
                val token = match(TurtleTokenType.BLANK_NODE_LABEL)
                return BlankNode(token.text)
            }
            else -> throw RuntimeException("Error Parsing Subject -- must be IRI or Blank Node")
        }
    }

    fun predicate() : Predicate {
        val token = match(TurtleTokenType.IRIREF)
        return IRI(token.text)
    }

    fun `object`() : Object {
        when (lookAhead.tokenType) {
            TurtleTokenType.IRIREF -> {
                val token = match(TurtleTokenType.IRIREF)
                return IRI(token.text)
            }
            TurtleTokenType.BLANK_NODE_LABEL -> {
                val token = match(TurtleTokenType.BLANK_NODE_LABEL)
                return BlankNode(token.text)
            }
            TurtleTokenType.STRING_LITERAL_QUOTE -> {
                return literal()
            }
            else -> throw RuntimeException("Error Parsing Object -- must be IRI, Blank Node, or Literal")
        }

    }

    fun literal() : Literal {
        val token = match(TurtleTokenType.STRING_LITERAL_QUOTE)
        when (lookAhead.tokenType) {
            TurtleTokenType.PERIOD -> return TypedLiteral(token.text)
            TurtleTokenType.LANGTAG -> {
                val lang = match(TurtleTokenType.LANGTAG)
                return LangLiteral(token.text, lang.text)
            }
            TurtleTokenType.TYPE_TAG -> {
                consume()
                val iri = match(TurtleTokenType.IRIREF)
                return TypedLiteral(token.text, IRI(iri.text))
            }
            else -> throw RuntimeException("Error Parsing")
        }
    }
}
