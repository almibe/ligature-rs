/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

import org.libraryweasel.stinkpot.Parser
import org.libraryweasel.stinkpot.Token
import org.libraryweasel.stinkpot.ntriples.model.*

class NTriplesParser extends Parser {
    final Closure<Triple> handler

    public NTriplesParser(NTriplesLexer lexer, Closure<Triple> handler) {
        super(lexer)
        this.handler = handler
    }

    void start() {
        while (lookAhead.tokenType != NTriplesTokenType.EOF) {
            triple()
        }
    }

    void triple() {
        Subject subject = subject()
        Predicate predicate = predicate()
        Object object = object()
        match(NTriplesTokenType.PERIOD)
        handler(new Triple(subject, predicate, object))
    }

    Subject subject() {
        Token token = match(NTriplesTokenType.IRIREF)
        return new IRI(token.text)
    }

    Predicate predicate() {
        Token token = match(NTriplesTokenType.IRIREF)
        return new IRI(token.text)
    }

    Object object() {
        switch (lookAhead.tokenType) {
            case NTriplesTokenType.IRIREF:
                Token token = match(NTriplesTokenType.IRIREF)
                return new IRI(token.text)
            case NTriplesTokenType.STRING_LITERAL_QUOTE:
                return literal()
            default:
                throw new RuntimeException("Error parsing object -- must be IRI or literal")
        }

    }

    Literal literal() {
        Token token = match(NTriplesTokenType.STRING_LITERAL_QUOTE)
        switch (lookAhead.tokenType) {
            case NTriplesTokenType.PERIOD:
                return new PlainLiteral(token.text)
            case NTriplesTokenType.LANGTAG:
                Token lang = match(NTriplesTokenType.LANGTAG)
                return new LangLiteral(token.text, lang.text)
            case NTriplesTokenType.IRIREF:
                Token iri = match(NTriplesTokenType.IRIREF)
                return new TypedLiteral(token.text, new IRI(iri.text))
        }
    }
}
