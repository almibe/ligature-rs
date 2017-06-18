/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.ntriples

import org.almibe.ligature.Triple
import org.almibe.ligature.parser.NTriplesLexer
import org.almibe.ligature.parser.NTriplesParser
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import java.io.BufferedReader
import java.io.StringReader
import java.nio.file.Path
import java.util.*
import java.util.stream.Stream

class NTriples {
    fun parseNTriples(text: String) : List<Triple>  {
        val triples : ArrayList<Triple> = ArrayList()
        parseNTriples(text) { triples.add(it) }
        return triples
    }

    fun parseNTriples(text: String, handler: (Triple) -> Unit) {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokens)
        parser.document()
        val handler = NTriplesHandler()


//        val lexer = NTriplesLexer(createStream(text))
//        val parser = NTriplesParser(lexer, handler)
//        parser.start()
    }

    fun parseNTriples(path: Path) : List<Triple> {
        val triples : ArrayList<Triple> = ArrayList()
//        parseNTriples(path) { triples.add(it) }
        return triples
    }

    fun parseNTriples(path: Path, handler: (Triple) -> Unit) {
//        val lexer = NTriplesLexer(Files.lines(path))
//        val parser = NTriplesParser(lexer, handler)
//        parser.start()
    }

    //TODO should this be moved?
    fun createStream(text: String) : Stream<String> {
        val reader = BufferedReader(StringReader(text))
        return reader.lines()
    }
}
