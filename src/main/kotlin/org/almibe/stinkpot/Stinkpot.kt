/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.stinkpot

import org.almibe.ligature.Triple
import org.almibe.stinkpot.ntriples.NTriplesLexer
import org.almibe.stinkpot.ntriples.NTriplesParser
import org.almibe.stinkpot.turtle.TurtleLexer
import org.almibe.stinkpot.turtle.TurtleParser
import java.io.BufferedReader
import java.io.StringReader
import java.nio.file.Files
import java.nio.file.Path
import java.util.*
import java.util.stream.Stream

class Stinkpot {
    fun parseNTriples(text: String) : List<Triple>  {
        val triples : ArrayList<Triple> = ArrayList()
        parseNTriples(text) { triples.add(it) }
        return triples
    }

    fun parseNTriples(text: String, handler: (Triple) -> Unit) {
        val lexer = NTriplesLexer(createStream(text))
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

    fun parseTurtle(text: String) : List<Triple>  {
        val triples : ArrayList<Triple> = ArrayList()
        parseTurtle(text) { triples.add(it) }
        return triples
    }

    fun parseTurtle(text: String, handler: (Triple) -> Unit) {
        val lexer = TurtleLexer(createStream(text))
        val parser = TurtleParser(lexer, handler)
        parser.start()
    }

    fun parseTurtle(path: Path) : List<Triple> {
        val triples : ArrayList<Triple> = ArrayList()
        parseTurtle(path) { triples.add(it) }
        return triples
    }

    fun parseTurtle(path: Path, handler: (Triple) -> Unit) {
        val lexer = TurtleLexer(Files.lines(path))
        val parser = TurtleParser(lexer, handler)
        parser.start()
    }

    fun createStream(text: String) : Stream<String> {
        val reader = BufferedReader(StringReader(text))
        return reader.lines()
    }
}
