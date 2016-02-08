/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.NTriplesLexer
import org.libraryweasel.stinkpot.ntriples.NTriplesParser
import org.libraryweasel.stinkpot.ntriples.Triple
import java.io.InputStreamReader
import java.nio.file.Files
import java.nio.file.Path
import java.util.*

class Stinkpot {
    fun parseTriples(text: String) : List<Triple>  {
        val triples : ArrayList<Triple> = ArrayList()
        parseTriples(text) { triples.add(it) }
        return triples
    }

    fun parseTriples(text: String, handler: (Triple) -> Unit) {
        val lexer = NTriplesLexer(StringUtil.createStringStream(text))
        val parser = NTriplesParser(lexer, handler)
        parser.start()
    }

    fun parseTriples(path: Path) : List<Triple> {
        val triples : ArrayList<Triple> = ArrayList()
        parseTriples(path) { triples.add(it) }
        return triples
    }

    fun parseTriples(path: Path, handler: (Triple) -> Unit) {
        val lexer = NTriplesLexer(Files.lines(path))
        val parser = NTriplesParser(lexer, handler)
        parser.start()
    }

    fun parseTurtle(text: String) : ArrayList<Triple>  {
        throw RuntimeException()
    }

    fun parseTurtle(text: String, handler: (Triple) -> Unit) {
        throw RuntimeException()
    }

    fun parseTurtle(reader: InputStreamReader, handler: (Triple) -> Unit) {
        throw RuntimeException()
    }
}
