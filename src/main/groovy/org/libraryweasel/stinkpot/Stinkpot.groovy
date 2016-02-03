/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.BlankNode
import org.libraryweasel.stinkpot.ntriples.IRI
import org.libraryweasel.stinkpot.ntriples.Predicate
import org.libraryweasel.stinkpot.ntriples.Subject
import org.libraryweasel.stinkpot.ntriples.Triple

public class Stinkpot {
    List<Triple> parseTriples(String text) {
        def triples = []
        parseTriples(text) { triples.add(it) }
        return triples
    }

    void parseTriples(String text, Closure<Triple> handler) {
        text.eachLine { line ->
            Iterator<Integer> it = text.chars().iterator()

            if (!it.hasNext()) return;

            Subject subject
            Predicate predicate
            Object object

            Character character = ignoreWhitespace(it)

            if (character == '<') {
                subject = readIRI(it)
            } else if (character == '#') {
                return
            } else if (character == '_' && it.hasNext() && it.next() == ':') {
                subject = readBlank(it)
            } else {
                throw new RuntimeException("Error parsing")
            }

            ignoreWhitespace(it)

            if (character == '<') {
                predicate = readIRI(it)
            } else {
                throw new RuntimeException("Error parsing")
            }

            ignoreWhitespace(it)

            if (character == '<') {
                object = readIRI(it)
            } else if (character == '_' && it.hasNext() && it.next() == ':') {
                object = readBlank(it)
            } else if (character == '"') {
                object = readLiteral(it)
            } else {
                throw new RuntimeException("Error parsing")
            }

            readEndOfLine(it)

            Triple triple = new Triple(subject, predicate, object)
            handler.call(triple)
        }
    }

    IRI readIRI(Iterator<Character> it) {

    }

    BlankNode readBlank() {

    }

    Character ignoreWhitespace(Iterator<Character> it) {

    }

    def readEndOfLine(Iterator<Character> it) {

    }
}
