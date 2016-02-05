/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.libraryweasel.stinkpot

import org.libraryweasel.stinkpot.ntriples.BlankNode
import org.libraryweasel.stinkpot.ntriples.IRI
import org.libraryweasel.stinkpot.ntriples.LangLiteral
import org.libraryweasel.stinkpot.ntriples.PlainLiteral
import org.libraryweasel.stinkpot.ntriples.Triple
import org.libraryweasel.stinkpot.ntriples.TypedLiteral
import spock.lang.Specification

public class StinkpotSpec extends Specification {
    Stinkpot stinkpot = new Stinkpot()

    def 'support basic IRI triple'() {
        given:
        def expectedResult = new Triple(new IRI("http://example.org/#spiderman"),
            new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/basicTriple.nt').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }

    def 'support multiple IRI triples'() {
        given:
        def expectedResult1 = new Triple(new IRI("http://example.org/#spiderman"),
                    new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        def expectedResult2 = new Triple(new IRI("http://example.org/#spiderman"),
                    new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#black-cat"))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/multipleIRITriples.nt').text)
        then:
        results.size() == 2
        results.first() == expectedResult1
        results.last() == expectedResult2
    }

    def 'support beginning of line and end of line comments'() {
        given:
        def expectedResult = new Triple(new IRI("http://example.org/#spiderman"),
                new IRI("http://www.perceive.net/schemas/relationship/enemyOf"), new IRI("http://example.org/#green-goblin"))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/comments.nt').text)
        then:
        results.size() == 1
        results.first() == expectedResult
    }

    def 'support literals with languages and types'() {
        given:
        def expectedResults = []
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new TypedLiteral("That Seventies Show", new IRI("http://www.w3.org/2001/XMLSchema#string"))))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://www.w3.org/2000/01/rdf-schema#label"), new PlainLiteral("That Seventies Show")))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("That Seventies Show", "en")))
        expectedResults.add(new Triple(new IRI("http://example.org/show/218"), new IRI("http://example.org/show/localName"), new LangLiteral("Cette Série des Années Septante", "fr-be")))
        expectedResults.add(new Triple(new IRI("http://example.org/#spiderman"), new IRI("http://example.org/text"), new PlainLiteral("This is a multi-line\\nliteral with many quotes (\\\"\\\"\\\"\\\"\\\")\\nand two apostrophes ('').")))
        expectedResults.add(new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/atomicNumber"), new TypedLiteral("2", new IRI("http://www.w3.org/2001/XMLSchema#integer"))))
        expectedResults.add(new Triple(new IRI("http://en.wikipedia.org/wiki/Helium"), new IRI("http://example.org/elements/specificGravity"), new TypedLiteral("1.663E-4", new IRI("http://www.w3.org/2001/XMLSchema#double"))))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/literals.nt').text)
        then:
        results == expectedResults
    }

    def 'support blank nodes'() {
        given:
        def expectedResult1 = new Triple(new BlankNode('alice'), new IRI('http://xmlns.com/foaf/0.1/knows'), new BlankNode('bob'))
        def expectedResult2 = new Triple(new BlankNode('bob'), new IRI('http://xmlns.com/foaf/0.1/knows'), new BlankNode('alice'))
        when:
        List<Triple> results = stinkpot.parseTriples(this.getClass().getResource('/ntriples/blankNodes.nt').text)
        then:
        results.size() == 2
        results.first() == expectedResult1
        results.last() == expectedResult2
    }
}
