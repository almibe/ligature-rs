/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.Callback
import org.libraryweasel.stinkpot.ntriples.NTriplesLexer
import org.libraryweasel.stinkpot.ntriples.NTriplesParser
import org.libraryweasel.stinkpot.ntriples.Triple

import java.util.ArrayList;
import java.util.List;

class Stinkpot {
    fun parseTriples(text: String) : ArrayList<Triple>  {
        val triples : ArrayList<Triple> = ArrayList()
        parseTriples(text) { triples.add(it) }
        return triples
    }

    fun parseTriples(text: String, handler: (Triple) -> Unit) {
        val lexer = NTriplesLexer(text)
        val parser = NTriplesParser(lexer, handler)
        parser.start()
    }
}
