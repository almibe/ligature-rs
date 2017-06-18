/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.turtle

import org.almibe.ligature.Triple
import java.io.BufferedReader
import java.io.StringReader
import java.nio.file.Path
import java.util.*
import java.util.stream.Stream

class Turtle {
    fun parseTurtle(text: String) : List<Triple>  {
        val triples : ArrayList<Triple> = ArrayList()
//        parseTurtle(text) { triples.add(it) }
        return triples
    }

    fun parseTurtle(text: String, handler: (Triple) -> Unit) {
//        val lexer = TurtleLexer(createStream(text))
//        val parser = TurtleParser(lexer, handler)
//        parser.start()
    }

    fun parseTurtle(path: Path) : List<Triple> {
        val triples : ArrayList<Triple> = ArrayList()
//        parseTurtle(path) { triples.add(it) }
        return triples
    }

    fun parseTurtle(path: Path, handler: (Triple) -> Unit) {
//        val lexer = TurtleLexer(Files.lines(path))
//        val parser = TurtleParser(lexer, handler)
//        parser.start()
    }

    //TODO should this be moved?
    fun createStream(text: String) : Stream<String> {
        val reader = BufferedReader(StringReader(text))
        return reader.lines()
    }

}