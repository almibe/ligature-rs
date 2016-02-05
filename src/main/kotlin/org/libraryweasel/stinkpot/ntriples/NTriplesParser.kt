/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

import org.libraryweasel.stinkpot.NTriplesTokenType

class NTriplesParser(lexer: NTriplesLexer, val handler: (Triple) -> Unit) : Parser(lexer) {


    fun start() : Unit {
        while (lookAhead.tokenType != NTriplesTokenType.EOF) {
            triple()
        }
    }

    fun triple() : Unit {
        val subject = subject()
        val predicate = predicate()
        val `object` = `object`()
        match(NTriplesTokenType.PERIOD)
        handler(Triple(subject, predicate, `object`))
    }

    fun subject() : Subject {
        val token = match(NTriplesTokenType.IRIREF)
        return IRI(token.text)
    }

    fun predicate() : Predicate {
        val token = match(NTriplesTokenType.IRIREF)
        return IRI(token.text)
    }

    fun `object`() : Object {
        when (lookAhead.tokenType) {
            NTriplesTokenType.IRIREF -> {
                val token = match(NTriplesTokenType.IRIREF)
                return IRI(token.text)
            }
            NTriplesTokenType.STRING_LITERAL_QUOTE -> {
                return literal()
            }
            else -> throw RuntimeException("Error parsing object -- must be IRI or literal")
        }

    }

    fun literal() : Literal {
        val token = match(NTriplesTokenType.STRING_LITERAL_QUOTE)
        when (lookAhead.tokenType) {
            NTriplesTokenType.PERIOD -> return PlainLiteral(token.text)
            NTriplesTokenType.LANGTAG -> {
                val lang = match(NTriplesTokenType.LANGTAG)
                return LangLiteral(token.text, lang.text)
            }
            NTriplesTokenType.IRIREF -> {
                val iri = match(NTriplesTokenType.IRIREF)
                return TypedLiteral(token.text, IRI(iri.text))
            }
            else -> throw RuntimeException("Error Parsing")
        }
    }
}
