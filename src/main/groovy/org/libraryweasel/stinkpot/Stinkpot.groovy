/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.*

import java.lang.Object

public class Stinkpot {
    List<Triple> parseTriples(String text) {
        def triples = []
        parseTriples(text) { triples.add(it) }
        return triples
    }

    void parseTriples(String text, Closure<Triple> handler) {
        text.eachLine { line ->
            Iterator<Integer> it = line.chars().iterator()

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

            character = ignoreWhitespace(it)

            if (character == '<') {
                predicate = readIRI(it)
            } else {
                throw new RuntimeException("Error parsing")
            }

            character = ignoreWhitespace(it)

            if (character == '<') {
                object = readIRI(it)
                checkEndOfLine(it)
            } else if (character == '_' && it.hasNext() && it.next() == ':') {
                object = readBlank(it)
                checkEndOfLine(it)
            } else if (character == '"') {
                object = readLiteralAndCheckEndOfLine(it)
            } else {
                throw new RuntimeException("Error parsing")
            }

            Triple triple = new Triple(subject, predicate, object)
            handler.call(triple)
        }
    }

    IRI readIRI(Iterator<Character> it) {
        if (!it.hasNext()) throw new RuntimeException("Error parsing")

        StringBuilder builder = new StringBuilder('')
        while(it.hasNext()) {
            Character character = it.next()
            if (character == ">") {
                break
            } else {
                builder.append(character)
            }
        }
        return new IRI(builder.toString())
    }

    BlankNode readBlank(Iterator<Character> it) {
        if (!it.hasNext()) throw new RuntimeException("Error parsing")

        //TODO finish
    }

    Character ignoreWhitespace(Iterator<Character> it) {
        if (!it.hasNext()) throw new RuntimeException("Error parsing")

        while(it.hasNext()) {
            Character character = it.next()
            if (character != ' ' && character != '\t') return character
        }
    }

    Literal readLiteralAndCheckEndOfLine(Iterator<Character> it) { //TODO finish
        if (!it.hasNext()) throw new RuntimeException("Error parsing")

        StringBuilder builder = new StringBuilder('')
        while (it.hasNext()) {
            Character character = it.next()
            //TODO handle invalid characters from spec -- [^#x22#x5C#xA#xD]
            //TODO check \\u chars when checking escape characters
            if (character == '\\') { //handle escape characters
                if (!it.hasNext()) throw new RuntimeException("Error parsing")
                character = it.next()
                switch (character) {
                    case 't': builder.append('\t'); break
                    case 'b': builder.append('\b'); break
                    case 'n': builder.append('\n'); break
                    case 'r': builder.append('\r'); break
                    case 'f': builder.append('\f'); break
                    case '"': builder.append('\"'); break
                    case '\'': builder.append('\''); break
                    case '\\': builder.append('\\'); break
                    default: throw new RuntimeException("Error parsing no such escape character " + character)
                }
            } else if (character == '"') {
                if (!it.hasNext()) throw new RuntimeException("Error parsing")
                character = it.next()
                if (character == '^') {
                    if (it.hasNext() && it.next() == '^' && it.hasNext() && it.next() == '<') {
                        IRI iri = readIRI(it)
                        //TODO check end of line
                        return new TypedLiteral(builder.toString(), iri)
                    } else {
                        throw new RuntimeException("Error parsing")
                    }
                } else if (character == '@') {
                    //TODO check end of line
                    //TODO check for valid lang
                } else {
                    //TODO check end of line
                    return new PlainLiteral(builder.toString())
                }
            } else {
                builder.append(character)
            }
        }
        throw new RuntimeException("Error parsing")
    }

    def checkEndOfLine(Iterator<Character> it) {
        if (!it.hasNext()) throw new RuntimeException("Error parsing")
        //TODO finish
    }
}
