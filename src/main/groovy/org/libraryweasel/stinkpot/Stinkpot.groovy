/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.NTriplesLexer
import org.libraryweasel.stinkpot.ntriples.NTriplesParser
import org.libraryweasel.stinkpot.ntriples.Triple

public class Stinkpot {
    List<Triple> parseTriples(String text) {
        def triples = []
        parseTriples(text) { triples.add(it) }
        return triples
    }

    void parseTriples(String text, Closure<Triple> handler) {
        NTriplesLexer lexer = new NTriplesLexer(text)
        NTriplesParser parser = new NTriplesParser(lexer, handler)
        parser.start()
    }
}
