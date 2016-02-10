/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.NTriplesLexer
import org.libraryweasel.stinkpot.ntriples.NTriplesParser
import org.libraryweasel.stinkpot.ntriples.Triple
import java.nio.file.Files
import java.nio.file.Path
import java.util.*

class Stinkpot {
    fun parseNTriples(text: String) : List<Triple>  {
        val triples : ArrayList<Triple> = ArrayList()
        parseNTriples(text) { triples.add(it) }
        return triples
    }

    fun parseNTriples(text: String, handler: (Triple) -> Unit) {
        val lexer = NTriplesLexer(StringUtil.createStringStream(text))
        val parser = NTriplesParser(lexer, handler)
        parser.start()
    }

    fun parseNTriples(path: Path) : List<Triple> {
        val triples : ArrayList<Triple> = ArrayList()
        parseNTriples(path) { triples.add(it) }
        return triples
    }

    fun parseNTriples(path: Path, handler: (Triple) -> Unit) {
        val lexer = NTriplesLexer(Files.lines(path))
        val parser = NTriplesParser(lexer, handler)
        parser.start()
    }
}
