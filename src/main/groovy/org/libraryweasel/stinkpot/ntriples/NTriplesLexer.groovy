/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot.ntriples

import org.libraryweasel.stinkpot.Lexer
import org.libraryweasel.stinkpot.Token

class NTriplesLexer extends Lexer {
    NTriplesLexer(String input) {
        super(input)
    }

    Token nextToken() {
        while (c != EOF) {
            switch (c) {
                case '#': comment(); continue;
                case ' ': case '\t': case '\n': case '\r': ws(); continue;
                case '<': return iri();
                case '@': return langTag();
                case '^': return typeTag();
                case '"': return stringLiteralQuote();
                case '.': return period();
                default: throw new RuntimeException("Error Parsing Found - $c")
            }
        }
        return new Token(NTriplesTokenType.EOF, "<EOF>")
    }

    void ws() {
        while (c == ' ' || c == '\t' || c == '\n' || c == '\r') consume()
    }

    void comment() {
        while (c != '\n' && c != EOF) {
            consume()
        }
    }

    Token iri() {
        StringBuilder stringBuilder = new StringBuilder()
        consume() //ignore <
        while ( c != '>') {
            stringBuilder.append(c)
            consume()
        }
        consume() //ignore >
        return new Token(NTriplesTokenType.IRIREF, stringBuilder.toString())
    }

    Token langTag() {
        StringBuilder stringBuilder = new StringBuilder()
        consume() //ignore @
        while ( c != ' ') {
            stringBuilder.append(c);
            consume();
        }
        return new Token(NTriplesTokenType.LANGTAG, stringBuilder.toString())
    }

    Token typeTag() {
        StringBuilder stringBuilder = new StringBuilder()
        consume() //ignore ^
        if (c != "^") throw new RuntimeException("Error parsing expecting ^^ after literal.")
        consume() //ignore ^
        return iri()
    }

    Token stringLiteralQuote() {
        StringBuilder stringBuilder = new StringBuilder()
        consume() //ignore "
        while ( c != '"') {
            stringBuilder.append(c)
            if (c == '\\') { //TODO handle escaped characters better
                consume()
                stringBuilder.append(c)
            }
            consume()
        }
        consume() //ignore "
        return new Token(NTriplesTokenType.STRING_LITERAL_QUOTE, stringBuilder.toString())
    }

    Token period() {
        consume() //ignore .
        return new Token(NTriplesTokenType.PERIOD, ".")
    }
}
